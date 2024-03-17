use crate::state::Extension;
use cosmwasm_schema::cw_serde;
use wrappr_factory_utils::{msg::{CreateMinterMsg, WrapprFactoryExecuteMsg, UpdateMinterParamsMsg}, WrapprMinterInitMsgExtension};

use crate::state::WrapprMinterParams;

#[cw_serde]
pub struct InstantiateMsg {
    pub params: WrapprMinterParams,
}

pub type WrapprMinterCreateMsg = CreateMinterMsg<WrapprMinterInitMsgExtension>;

pub type ExecuteMsg = WrapprFactoryExecuteMsg<WrapprMinterInitMsgExtension>;

pub type BaseUpdateParamsMsg = UpdateMinterParamsMsg<Extension>;

#[cw_serde]
pub enum SudoMsg<T> {
    UpdateParams(Box<T>),
}
pub type BaseSudoMsg = SudoMsg<BaseUpdateParamsMsg>;

#[cw_serde]
pub struct ParamsResponse {
    pub params: WrapprMinterParams,
}
