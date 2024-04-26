use anyhow::Error;
use cosmwasm_std::{Addr, Timestamp};

use contract_boxes::App;
use cosmwasm_std::Uint128;
use wrappr_factory_utils::msg::CollectionParams;

pub struct MinterSetupParams<'a> {
    pub router: &'a mut App,
    pub minter_admin: Addr,
    pub num_tokens: u32,
    pub collection_params: CollectionParams,
    pub splits_addr: Option<String>,
    pub start_time: Option<Timestamp>,
    pub minter_code_id: u64,
    pub factory_code_id: u64,
    pub wrappr721_code_id: u64,
    pub init_msg: Option<WrapprMinterInitMsgExtension>,
}
pub struct MinterCollectionResponse {
    pub minter: Option<Addr>,
    pub collection: Option<Addr>,
    pub factory: Option<Addr>,
    pub error: Option<Error>,
}

pub struct MinterInstantiateParams {
    pub num_tokens: u32,
    pub start_time: Option<Timestamp>,
    pub splits_addr: Option<String>,
    pub init_msg: Option<WrapprMinterInitMsgExtension>,
}

use cosmwasm_schema::cw_serde;
use wrappr_factory_utils::WrapprMinterInitMsgExtension;

use super::contract_boxes;

#[cw_serde]
pub struct CodeIds {
    pub minter_code_id: u64,
    pub factory_code_id: u64,
    pub wrappr721_code_id: u64,
}

pub struct MinterTemplateResponse<T> {
    pub collection_response_vec: Vec<MinterCollectionResponse>,
    pub router: App,
    pub accts: T,
}

pub struct MinterTemplateResponseCodeIds<T> {
    pub collection_response_vec: Vec<MinterCollectionResponse>,
    pub router: App,
    pub accts: T,
    pub code_ids: CodeIds,
}

pub struct Accounts {
    pub creator: Addr,
    pub buyer: Addr,
}