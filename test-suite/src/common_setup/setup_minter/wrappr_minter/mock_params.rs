use wrappr_factory::{msg::WrapprMinterCreateMsg, state::WrapprMinterParams};
use cosmwasm_std::{coin, Timestamp};
use wrappr_factory_utils::{msg::CollectionParams, WrapprMinterInitMsgExtension};
use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};

const CREATION_FEE: u128 = 1_000_000_000;
pub const MIN_MINT_PRICE: u128 = 50_000_000;
const MINT_FEE_BPS: u64 = 10_000; // 100%

pub fn mock_params() -> WrapprMinterParams {
    WrapprMinterParams {
        code_id: 1,
        allowed_wrappr721_code_ids: vec![3],
        frozen: false,
        creation_fee: coin(CREATION_FEE, NATIVE_DENOM),
        min_mint_price: coin(MIN_MINT_PRICE, NATIVE_DENOM),
        mint_fee_bps: MINT_FEE_BPS,
        max_trading_offset_secs: 60 * 60 * 24 * 7,
        extension: None,
    }
}

pub fn mock_create_minter(collection_params: CollectionParams) -> WrapprMinterCreateMsg {
    WrapprMinterCreateMsg {
        init_msg: mock_init_extension(None, None),
        collection_params,
    }
}


pub fn mock_init_extension(
    splits_addr: Option<String>,
    start_time: Option<Timestamp>,
) -> WrapprMinterInitMsgExtension {
    wrappr_factory_utils::WrapprMinterInitMsgExtension {
        payment_address: splits_addr,
        start_time: start_time.unwrap_or(Timestamp::from_nanos(GENESIS_MINT_START_TIME)),
        mint_price: coin(MIN_MINT_PRICE, NATIVE_DENOM),
        per_address_limit: 3,
        whitelist: None,
        entity: "DUNA".to_string(),
        jurisdiction: "WYOMING".to_string(),
    }
}