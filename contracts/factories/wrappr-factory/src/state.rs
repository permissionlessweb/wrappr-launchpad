use cosmwasm_std::Empty;
use cw_storage_plus::Item;
use wrappr_factory_utils::MinterParams;

pub type Extension = Option<Empty>;

pub type WrapprMinterParams = MinterParams<Extension>;

pub const SUDO_PARAMS: Item<WrapprMinterParams> = Item::new("sudo-params");
