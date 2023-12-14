mod msg;
mod query;
mod route;

pub const NATIVE_DENOM: &str = "ujuno";

pub const JURISDICTION: &str = "deleware"; 
pub const ENTITY: &str = "llc"; 

// 3/11/2022 16:00:00 ET
pub const GENESIS_MINT_START_TIME: u64 = 1647032400000000000;

use cosmwasm_std::{coin, coins, Addr, BankMsg, Coin};
pub use msg::{
 create_fund_community_pool_msg, 
     WrapprMsg, WrapprMsgWrapper,
};

pub type Response = cosmwasm_std::Response<WrapprMsgWrapper>;
pub type SubMsg = cosmwasm_std::SubMsg<WrapprMsgWrapper>;
pub type CosmosMsg = cosmwasm_std::CosmosMsg<WrapprMsgWrapper>;

pub use query::WrapprQuery;
pub use route::WrapprRoute;

// This export is added to all contracts that import this package, signifying that they require
// "stargaze" support on the chain they run on.
// #[no_mangle]
// extern "C" fn requires_stargaze() {}

pub fn junos(amount: impl Into<u128>) -> Vec<Coin> {
    coins(amount.into(), NATIVE_DENOM)
}

pub fn star(amount: impl Into<u128>) -> Coin {
    coin(amount.into(), NATIVE_DENOM)
}

pub fn send_msg(to_address: &Addr, amount: impl Into<u128>) -> BankMsg {
    BankMsg::Send {
        to_address: to_address.to_string(),
        amount: junos(amount),
    }
}
