use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg};
use crate::state::{increment_token_index, Config, COLLECTION_ADDRESS, CONFIG, STATUS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, CosmosMsg, Decimal, Deps, DepsMut, Empty, Env, MessageInfo,
    Reply, Response, StdResult, SubMsg, Timestamp, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::{must_pay, nonpayable, parse_reply_instantiate_data};
use wrappr721::{ExecuteMsg as Wrappr721ExecuteMsg, InstantiateMsg as Wrappr721InstantiateMsg};
use wrappr721_base::msg::{CollectionInfoResponse, QueryMsg as Wrappr721QueryMsg};
use wrappr_factory::msg::{ParamsResponse, WrapprMinterCreateMsg};
use wrappr_factory::state::Extension;
use wrappr_factory_utils::query::WrapprFactoryQueryMsg;
use wrappr_minter_utils::{QueryMsg, Status, StatusResponse, SudoMsg};

use sg_std::NATIVE_DENOM;

use url::Url;

const CONTRACT_NAME: &str = "crates.io:sg-base-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_WRAPPR721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: WrapprMinterCreateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let factory = info.sender.clone();

    // set default status so it can be queried without failing
    STATUS.save(deps.storage, &Status::default())?;

    // Make sure the sender is the factory contract
    // This will fail if the sender cannot parse a response from the factory contract
    let factory_params: ParamsResponse = deps
        .querier
        .query_wasm_smart(factory.clone(), &WrapprFactoryQueryMsg::Params {})?;

    let config = Config {
        factory: factory.clone(),
        collection_code_id: msg.collection_params.code_id,
        // assume the mint price is the minimum mint price
        // 100% is fair burned
        mint_price: factory_params.params.min_mint_price,
        extension: Empty {},
    };

    // Use default start trading time if not provided
    let mut collection_info = msg.collection_params.info.clone();
    let offset = factory_params.params.max_trading_offset_secs;

    CONFIG.save(deps.storage, &config)?;

    // instantiate2 details
    let creator = deps.api.addr_canonicalize(env.contract.address.as_str())?;
    // checksum of the minter factory.
    let ContractInfoResponse { code_id, .. } = deps
        .querier
        .query_wasm_contract_info(env.contract.address)?;
    let CodeInfoResponse { checksum, .. } = deps.querier.query_wasm_code_info(code_id)?;

    let mut msgs = Vec::<WasmMsg>::new();
    let mut attributes = Vec::<Attribute>::new();

    let jusrisdiction = msg.init_msg.jurisdiction;
    let entity = msg.init_msg.entity;
    let deployer = msg.init_msg.payment_address.unwrap_or(factory);

    let path = format!("{jurisdiction}/{entity}/{deployer}");
    let label = format!("Instance {path}");
    let salt = Binary::from(path.as_bytes());

    let address =
        deps.api
            .addr_humanize(&instantiate2_address(checksum.as_ref(), &creator, &salt)?)?;
    attributes.push(Attribute::new(
        format!("predicted_address"),
        address.clone(),
    ));

    msgs.push(WasmMsg::Instantiate2 {
        admin: Some(
            deps.api
                .addr_validate(&msg.collection_params.info.creator)?
                .to_string(),
        ),
        code_id: msg.collection_params.code_id,
        label: format!(
            "WRAPPR721-{}-{}",
            msg.collection_params.code_id,
            msg.collection_params.name.trim()
        ),
        msg: to_json_binary(&Wrappr721InstantiateMsg {
            name: msg.collection_params.name.clone(),
            symbol: msg.collection_params.symbol,
            minter: env.contract.address.to_string(),
            collection_info,
        })?,
        funds: info.funds,
        salt: salt,
    });
    // let submsg = SubMsg::reply_on_success(wasm_msg, INSTANTIATE_WRAPPR721_REPLY_ID);

    // mints wrappr.
    msgs.push(WasmMsg::Execute {
        contract_addr: address.into(),
        msg: to_json_binary(&ExecuteMsg::Mint {
            token_uri: msg.init_msg.token_uri,
        })?,
        funds: vec![],
    });

    Ok(
        Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION)
            .add_attribute("sender", factory)
            .add_attribute(format!("wrappr-instance"), path.clone())
            .add_attributes(attributes)
            .add_messages(msgs), // .add_submessage(submsg)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { token_uri } => execute_mint_sender(deps, info, token_uri),
        ExecuteMsg::UpdateStartTradingTime(time) => {
            execute_update_start_trading_time(deps, env, info, time)
        }
    }
}

pub fn execute_mint_sender(
    deps: DepsMut,
    info: MessageInfo,
    token_uri: String,
) -> Result<Response, ContractError> {
    let minter = info.sender.clone();

    // Make sure the sender is the minter contract
    // This will fail if the sender cannot parse a response from the mint contract
    let minter_params: ParamsResponse = deps
        .querier
        .query_wasm_smart(factory.clone(), &QueryMsg::Config{})?;

    let config = CONFIG.load(deps.storage)?;
    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;

    // This is a 1:1 minter, minted at min_mint_price
    // Should mint and then list on the marketplace for secondary sales
    let collection_info: CollectionInfoResponse = deps.querier.query_wasm_smart(
        collection_address.clone(),
        &Wrappr721QueryMsg::CollectionInfo {},
    )?;
    // allow only wrappr721 creator address to mint
    if collection_info.creator != info.sender {
        return Err(ContractError::Unauthorized(
            "Sender is not wrappr721 creator".to_owned(),
        ));
    };

    // Token URI must be a valid URL (ipfs, https, etc.)
    Url::parse(&token_uri).map_err(|_| ContractError::InvalidTokenURI {})?;

    let mut res = Response::new();

    let factory: ParamsResponse = deps
        .querier
        .query_wasm_smart(config.factory, &WrapprFactoryQueryMsg::Params {})?;
    let factory_params = factory.params;

    let funds_sent = must_pay(&info, NATIVE_DENOM)?;

    // Create network fee msgs
    let mint_fee_percent = Decimal::bps(factory_params.mint_fee_bps);
    let network_fee = config.mint_price.amount * mint_fee_percent;
    // For the base 1/1 minter, the entire mint price should be Fair Burned
    if network_fee != funds_sent {
        return Err(ContractError::InvalidMintPrice {});
    }
    // checked_fair_burn(&info, network_fee.u128(), None, &mut res)?;

    // Create mint msgs
    let mint_msg = Wrappr721ExecuteMsg::<Extension, Empty>::Mint {
        token_id: increment_token_index(deps.storage)?.to_string(),
        owner: info.sender.to_string(),
        token_uri: Some(token_uri.clone()),
        extension: None,
    };
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collection_address.to_string(),
        msg: to_json_binary(&mint_msg)?,
        funds: vec![],
    });
    res = res.add_message(msg);

    Ok(res
        .add_attribute("action", "mint")
        .add_attribute("sender", info.sender)
        .add_attribute("token_uri", token_uri)
        .add_attribute("network_fee", network_fee.to_string()))
}

pub fn execute_update_start_trading_time(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    start_time: Option<Timestamp>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    let wrappr721_contract_addr = COLLECTION_ADDRESS.load(deps.storage)?;

    let collection_info: CollectionInfoResponse = deps.querier.query_wasm_smart(
        wrappr721_contract_addr.clone(),
        &Wrappr721QueryMsg::CollectionInfo {},
    )?;
    if info.sender != collection_info.creator {
        return Err(ContractError::Unauthorized(
            "Sender is not creator".to_owned(),
        ));
    }

    // add custom rules here
    if let Some(start_time) = start_time {
        if env.block.time > start_time {
            return Err(ContractError::InvalidStartTradingTime(
                env.block.time,
                start_time,
            ));
        }
    }

    // execute wrappr721 contract
    let msg = WasmMsg::Execute {
        contract_addr: wrappr721_contract_addr.to_string(),
        msg: to_json_binary(
            &Wrappr721ExecuteMsg::<Extension, Empty>::UpdateStartTradingTime(start_time),
        )?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_attribute("action", "update_start_time")
        .add_attribute("sender", info.sender)
        .add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::UpdateStatus {
            is_verified,
            is_blocked,
            is_explicit,
        } => update_status(deps, is_verified, is_blocked, is_explicit)
            .map_err(|_| ContractError::UpdateStatus {}),
    }
}

/// Only governance can update contract params
pub fn update_status(
    deps: DepsMut,
    is_verified: bool,
    is_blocked: bool,
    is_explicit: bool,
) -> StdResult<Response> {
    let mut status = STATUS.load(deps.storage)?;
    status.is_verified = is_verified;
    status.is_blocked = is_blocked;
    status.is_explicit = is_explicit;
    STATUS.save(deps.storage, &status)?;

    Ok(Response::new().add_attribute("action", "sudo_update_status"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::Status {} => to_json_binary(&query_status(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let collection_address = COLLECTION_ADDRESS.load(deps.storage)?;

    Ok(ConfigResponse {
        collection_address: collection_address.to_string(),
        config,
    })
}

pub fn query_status(deps: Deps) -> StdResult<StatusResponse> {
    let status = STATUS.load(deps.storage)?;

    Ok(StatusResponse { status })
}

// Reply callback triggered from wrappr721 contract instantiation in instantiate()
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != INSTANTIATE_WRAPPR721_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }

    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            let collection_address = res.contract_address;
            COLLECTION_ADDRESS.save(deps.storage, &Addr::unchecked(collection_address.clone()))?;
            Ok(Response::default()
                .add_attribute("action", "instantiate_wrappr721_reply")
                .add_attribute("wrappr721_address", collection_address))
        }
        Err(_) => Err(ContractError::InstantiateSg721Error {}),
    }
}
