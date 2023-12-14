use crate::route::WrapprRoute;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg};
static MSG_DATA_VERSION: &str = "1.0.0";

/// WrapprMsg is an override of CosmosMsg::Custom to add support for Stargaze's custom message types
#[cw_serde]
pub struct WrapprMsgWrapper {
    pub route: WrapprRoute,
    pub msg_data: WrapprMsg,
    pub version: String,
}

impl From<WrapprMsgWrapper> for CosmosMsg<WrapprMsgWrapper> {
    fn from(original: WrapprMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

impl CustomMsg for WrapprMsgWrapper {}

#[cw_serde]
pub enum WrapprMsg {
    // ClaimFor {
    //     address: String,
    //     action: ClaimAction,
    // },
    FundCommunityPool {
        amount: Vec<Coin>,
    },
    // FundFairburnPool {
    //     amount: Vec<Coin>,
    // },
}

// #[cw_serde]
// pub enum ClaimAction {
//     #[serde(rename = "mint_nft")]
//     MintNFT,
//     #[serde(rename = "bid_nft")]
//     BidNFT,
// }

// pub fn create_claim_for_msg(address: String, action: ClaimAction) -> CosmosMsg<WrapprMsgWrapper> {
//     WrapprMsgWrapper {
//         route: WrapprRoute::Claim,
//         msg_data: WrapprMsg::ClaimFor { address, action },
//         version: MSG_DATA_VERSION.to_owned(),
//     }
//     .into()
// }

pub fn create_fund_community_pool_msg(amount: Vec<Coin>) -> CosmosMsg<WrapprMsgWrapper> {
    WrapprMsgWrapper {
        route: WrapprRoute::Distribution,
        msg_data: WrapprMsg::FundCommunityPool { amount },
        version: MSG_DATA_VERSION.to_owned(),
    }
    .into()
}

// pub fn create_fund_fairburn_pool_msg(amount: Vec<Coin>) -> CosmosMsg<WrapprMsgWrapper> {
//     WrapprMsgWrapper {
//         route: WrapprRoute::Alloc,
//         msg_data: WrapprMsg::FundFairburnPool { amount },
//         version: MSG_DATA_VERSION.to_owned(),
//     }
//     .into()
// }
