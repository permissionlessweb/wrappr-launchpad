use cosmwasm_std::Timestamp;
use wrappr_factory_utils::WrapprMinterInitMsgExtension;


use crate::common_setup::msg::MinterInstantiateParams;

pub fn minter_params_all(
    num_tokens: u32,
    splits_addr: Option<String>,
    start_time: Option<Timestamp>,
    init_msg: Option<WrapprMinterInitMsgExtension>,
) -> MinterInstantiateParams {
    MinterInstantiateParams {
        num_tokens,
        splits_addr,
        start_time,
        init_msg,
    }
}

pub fn minter_params_token(num_tokens: u32) -> MinterInstantiateParams {
    MinterInstantiateParams {
        num_tokens,
        splits_addr: None,
        start_time: None,
        init_msg: None,
    }
}
