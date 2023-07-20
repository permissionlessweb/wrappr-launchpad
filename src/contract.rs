#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::Addr;
use cw2::set_contract_version;

use crate::query::{check_fee, query_fee_info} // mint fee 

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Ids, Amounts};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-wrappr";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admins_cw4_group  = deps.api.addr_validate(&msg.admins_cw4_group)?;
    let managers_cw4_group = deps.api.addr_validate(&msg.makers_cw4_group)?;

    let config = Config {
        admins_cw4_group,
        managers_cw4_group,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            to,
            id,
            amount,
            /*{data}*/
            token_uri,
            owner,
        } => execute_mint(
            deps, env, info, to, id, amount, /*{data}*/ token_uri, owner
        ),
        ExecuteMsg::Burn {
            from,
            id,
            amount,
        } => execute_burn(
            deps, env, info, from, id, amount,
        ),
        ExecuteMsg::ManageMint {
            to,
            id,
            amount,
            /*{data}*/
            token_uri,
            owner,
        } => execute_manage_mint(
            deps, env, info, to, id, amount, /*{data}*/ token_uri, owner
        ),
        ExecuteMsg::ManageBurn {
            from,
            id,
            amount,
        } => execute_manage_burn(
            deps, env, info, from, id, amount,
        ),
        ExecuteMsg::SetOwnerOf {
            to,
            id,
        } => execute_set_owner_of(
            deps, env, info, to, id,
        ),
        ExecuteMsg::SetTransferability {
            id,
            set,
        } => execute_set_transferability(
            deps, env, info, id, set,
        ),
        ExecuteMsg::SetPermissions {
            id,
            set,
        } => execute_set_permissions(
            deps, env, info, id, set,
        ),
        ExecuteMsg::SetUserPermissions {
            to,
            id,
            set,
        } => execute_set_user_permissions(
            deps, env, info, to, id, set,
        ),
        ExecuteMsg::SetURI {
            id,
            token_uri,
        } => execute_set_uri(
            deps, env, info, id, token_uri,
        ),
        ExecuteMsg::SetUserURI {
            to,
            id,
            user_uri,
        } => execute_set_user_uri(
            deps, env, info, to, id, user_uri,
        ),
        ExecuteMsg::SetManager {
            to,
            manager,
        } => execute_set_manager(
            deps, env, info, to, set,
        ),
        ExecuteMsg::SetAdmin {
            to,
            admin,
        } => execute_set_admin(
            deps, env, info, to, admin,
        ),
        ExecuteMsg::SetBaseURI {
            base_uri,
        } => execute_set_base_uri(
            deps, env, info, base_uri,
        ),
        ExecuteMsg::SetMintFee {
            mint_fee,
        } => execute_set_mint_fee(
            deps, env, info, mint_fee,
        ),
        ExecuteMsg::ClaimFee {
            to,
            amount,
        } => execute_claim_fee(
            deps, env, info, to, amount,
        ),
        ExecuteMsg::SafeTransferFrom {
            from,
            to,
            id,
            amount,
        } => execute_safe_transfer_from(
            deps, env, info, from, to, id, amount,
        ),
        ExecuteMsg::SafeBatchTransferFrom {
            from,
            to,
            ids,
            amounts,
        } => execute_safe_batch_transfer_from(
            deps, env, info, from, to, ids, amounts,
        ),
    }
}

// Execute Message Logics

fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    id: u64,
    amount: u64,
    /*data: bytes*/
    token_uri: String,
    owner: Addr,
) -> Result<Response, ContractError> {
    // mint logic
    // - if the mint fee is not zero, require the fee 

    // - panic if id is already registered
    
    // - if the owner does not equal 0, set owner of id to the owner

    // - register the id 
}

fn execute_burn(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    from: Addr,
    id: u64,
    amount: u64,
) -> Result<Response, ContractError> {
    // burn logic

    // check if info.sender is owner or admin of contract.
}

fn execute_manage_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    id: u64,
    amount: u64,
    /*data: bytes*/
    token_uri: String,
    owner: Addr,
) -> Result<Response, ContractError> {
    // manage mint logic:

    // Check if info.sender is in managers_cw4_group, admins_cw4_group, or owner_of id

    // if the id is not registered, register it.

    // if there is no owner of the id, assign the owner to the id. 

    // Check to make sure that the newly assigned owner does not equal 0.

}

fn execute_manage_burn(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    from: Addr,
    id: u64,
    amount: u64,
) -> Result<Response, ContractError> {
    // manage burn logic

    // requre info.sender to be owner of id, or admin of contract
    if info.sender !=  {
        return Err(ContractError::Unauthorized {});
    };

}

fn execute_set_owner_of(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    id: u64,
) -> Result<Response, ContractError> {
    // set owner of logic

    // requre info.sender to be owner of id, or admin of contract
    if info.sender !=  {
        return Err(ContractError::Unauthorized {});
    };

    
    // set the ownerOf[to]
}

fn execute_set_transferability(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    set: bool,
) -> Result<Response, ContractError> {
    // set transferability logic

    // requre info.sender to be owner of, or admin of the id
    if info.sender !=  {
        return Err(ContractError::Unauthorized {});
    };


    // set the transferable[id] 
}

fn execute_set_permissions(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    set: bool,
) -> Result<Response, ContractError> {
    // set permissions logic

    // requre info.sender to be owner of, or admin of the id
    if info.sender !=  {
        return Err(ContractError::Unauthorized {});
    };


    // set the permissioned[id]
}

fn execute_set_user_permissions(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    id: u64,
    set: bool,
) -> Result<Response, ContractError> {
    // set user permissions logic

    // requre info.sender to be owner of, or admin of the id
    if info.sender !=  {
        return Err(ContractError::Unauthorized {});
    };


    // set a userPermissioned[to][id]
}

fn execute_set_uri(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    token_uri: String,
) -> Result<Response, ContractError> {
    // set uri logic

    // require message sender to only be owner of, or admin. 
    if info.sender !=  {
        return Err(ContractError::Unauthorized {});
    };

    // uris[id] = tokenURI;
}

fn execute_set_user_uri(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    id: u64,
    user_uri: String,
) -> Result<Response, ContractError> {
    // set user_uri logic

    // requre info.sender to be owner of, or admin of the id
    if info.sender !=  {
        return Err(ContractError::Unauthorized {});
    };

    // userURI[to][id] = uuri;
}

fn execute_set_manager(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    set: bool,
) -> Result<Response, ContractError> {
    // set manager logic

    // require only the admin for this msg.
    check_admin_membership(&deps, &info.sender)?;
    
    // manager[to] = set;
}

fn execute_set_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    admin: Addr
) -> Result<Response, ContractError> {
    // set admin logic

    // require only the admin for this msg. 
    check_admin_membership(&deps, &info.sender)?;

    // admin = _admin;
}

fn execute_set_base_uri(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    base_uri: String,
) -> Result<Response, ContractError> {
    // set base_uri logic

    // require only the admin for this msg. 
    check_admin_membership(&deps, &info.sender)?;

    // baseURI = _baseURI;
}

fn execute_set_mint_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mint_fee: u64,
) -> Result<Response, ContractError> {
    // set mint fee logic

    // require only the admin for this msg. 
    check_admin_membership(&deps, &info.sender)?;

    // mintFee = _mintFee;

}

fn execute_claim_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    amount: u64,
) -> Result<Response, ContractError> {
    // set claim fee logic

    // require only the admin for this msg. 
    check_admin_membership(&deps, &info.sender)?;

}

fn execute_safe_transfer_from(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    from: Addr,
    to: Addr,
    id: u64,
    amount: u64,
    /*data: bytes*/
) -> Result<Response, ContractError> {
    // safe transfer logic

    // validate token id is transferrable.

    // if (permissioned[id]) require(userPermissioned[from][id] && userPermissioned[to][id], "NOT_PERMITTED");
}

fn execute_safe_batch_transfer_from(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    from: Addr,
    to: Addr,
    ids: Ids,
    amounts: Amounts,
) -> Result<Response, ContractError> {
    // safe batch transfer logic
    //
    // validate if each id in msg is transferrable.
    // check for permissions
}

fn check_admin_membership (deps: &DepsMut, sender: &Addr) -> Result<(), ContractError> {

    let config = CONFIG.load(deps.storage)?;
    let res: MemberResponse = deps.querier.query_wasm_smart(
        config.admins_cw4_group,
        &Cw4QueryMsg::Member {
            addr: sender.to_string(),
            at_height: None,
        },
    )?;
    if res.weight.is_none() {
        return Err(ContractError::Unauthorized {});
    };

    Ok(())
}


// Query Logic

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
