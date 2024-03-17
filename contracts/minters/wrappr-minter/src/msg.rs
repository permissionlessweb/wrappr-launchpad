use wrappr_factory::{msg::WrapprMinterCreateMsg, state::WrapprMinterParams};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Empty, Timestamp};
use wrappr_minter_utils::MinterConfigResponse;

#[cw_serde]
pub struct InstantiateMsg {
    pub create_msg: WrapprMinterCreateMsg,
    pub params: WrapprMinterParams,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { token_uri: String },
    UpdateStartTradingTime(Option<Timestamp>),
}

pub type ConfigResponse = MinterConfigResponse<Empty>;
