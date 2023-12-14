use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

/// Saved in every minter
#[cw_serde]
pub struct MinterConfig<T> {
    pub factory: Addr,
    pub collection_code_id: u64,
    pub mint_price: Coin,
    pub jurisdiction: String,
    pub entity: String,
    pub extension: T,
}

#[cw_serde]
pub struct MinterConfigResponse<T> {
    pub config: MinterConfig<T>,
    pub collection_address: String,
}

#[cw_serde]
#[derive(Default)]
pub struct Status {
    pub is_verified: bool,
}

#[cw_serde]
pub struct StatusResponse {
    pub status: Status,
}

#[cw_serde]
pub enum QueryMsg {
    /// Returns `MinterConfigResponse<T>`
    Config {},
    /// Returns `StatusResponse`
    Status {},
}

#[cw_serde]
pub enum SudoMsg {
    UpdateStatus {
        is_verified: bool,
    },
}
