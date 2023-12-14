use cosmwasm_schema::cw_serde;

use crate::CodeId;
use crate::MinterParams;

#[cw_serde]
pub enum WrapprFactoryUtilsQueryMsg {
    /// Returns `ParamsResponse`
    Params {},
    AllowedWrappr721CodeIds {},
    AllowedWrappr721CodeId(CodeId),
}

#[cw_serde]
pub struct ParamsResponse<T> {
    pub params: MinterParams<T>,
}

#[cw_serde]
pub struct AllowedWrappr721CodeIdsResponse {
    pub code_ids: Vec<CodeId>,
}

#[cw_serde]
pub struct AllowedWrappr721CodeIdResponse {
    pub allowed: bool,
}
