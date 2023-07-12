#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::Addr;
use cw2::set_contract_version;

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

    // Instantiate Logic Here.

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
            deps, env, info, to, manager,
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
    //
    // require message sender to be the owner, manager, or admin

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
    //
    // require message sender to be the owner, manager, or admin
}

fn execute_set_owner_of(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    id: u64,
) -> Result<Response, ContractError> {
    // set owner of logic
    //
    // requre message sender to only be owner of, or admin of the id
}

fn execute_set_transferability(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    set: bool,
) -> Result<Response, ContractError> {
    // set transferability logic
    //
    // requre message sender to only be owner of, or admin of the id
}

fn execute_set_permissions(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    set: bool,
) -> Result<Response, ContractError> {
    // set permissions logic
    //
    // requre message sender to only be owner of, or admin.
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
    //
    // requre message sender to only be owner of, or admin.
}

fn execute_set_uri(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: u64,
    token_uri: String,
) -> Result<Response, ContractError> {

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
    //
    // requre message sender to only be owner of, or admin.
}

fn execute_set_manager(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    manager: bool,
) -> Result<Response, ContractError> {
    // set manager logic
    //
    // require only the admin for this msg. 
}

fn execute_set_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    admin: Addr
) -> Result<Response, ContractError> {
    // set admin logic
    //
    // require only the admin for this msg. 
}

fn execute_set_base_uri(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    base_uri: String,
) -> Result<Response, ContractError> {
    // set base_uri logic
    //
    // require only the admin for this msg. 
}

fn execute_set_mint_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    mint_fee: u64,
) -> Result<Response, ContractError> {
    // set mint fee logic
    //
    // require only the admin for this msg. 
}

fn execute_claim_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    to: Addr,
    amount: u64,
) -> Result<Response, ContractError> {
    // set claim fee logic
    //
    // require only the admin for this msg. 
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
    //
    // validate token id is transferrable.
    // check for permissions
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

// Query Logic

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
