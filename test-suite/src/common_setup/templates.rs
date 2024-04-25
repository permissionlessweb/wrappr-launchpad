use crate::common_setup::contract_boxes::{custom_mock_app, App};
use crate::common_setup::msg::MinterTemplateResponse;
use crate::common_setup::{
    msg::MinterCollectionResponse,
    setup_accounts_and_block::setup_accounts,
    setup_minter::common::minter_params::minter_params_token,
};

use super::msg::{Accounts, CodeIds, MinterTemplateResponseCodeIds};
use super::setup_minter::wrappr_minter::setup::wrappr_minter_wrappr721_collection_code_ids;
use super::setup_minter::common::constants::{MINT_PRICE, MIN_MINT_PRICE};
use crate::common_setup::setup_minter::wrappr_minter::setup::wrappr_minter_wrappr721_nt_code_ids;
use crate::common_setup::setup_minter::wrappr_minter::setup::configure_wrappr_minter;
use cosmwasm_std::{coin, Timestamp};
use wrappr_factory_utils::tests::{mock_collection_params_1, mock_collection_two};

use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};


pub fn wrappr_minter_with_wrappr721nt(num_tokens: u32) -> MinterTemplateResponse<Accounts> {
    let mut router = custom_mock_app();
    let (creator, buyer) = setup_accounts(&mut router);
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let code_ids = wrappr_minter_wrappr721_nt_code_ids(&mut router);
    let minter_collection_response = configure_wrappr_minter(
        &mut router,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    MinterTemplateResponse {
        router,
        collection_response_vec: minter_collection_response,
        accts: Accounts { creator, buyer },
    }
}

pub fn wrappr_minter_with_wrappr721(num_tokens: u32) -> MinterTemplateResponse<Accounts> {
    let mut router = custom_mock_app();
    let (creator, buyer) = setup_accounts(&mut router);
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let code_ids = wrappr_minter_wrappr721_collection_code_ids(&mut router);
    let minter_collection_response = configure_wrappr_minter(
        &mut router,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    MinterTemplateResponse {
        router,
        collection_response_vec: minter_collection_response,
        accts: Accounts { creator, buyer },
    }
}

pub fn wrappr_minter_with_specified_wrappr721(
    num_tokens: u32,
    wrappr721_code_id: u64,
) -> MinterTemplateResponse<Accounts> {
    let mut router = custom_mock_app();
    let (creator, buyer) = setup_accounts(&mut router);
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let mut code_ids = wrappr_minter_wrappr721_collection_code_ids(&mut router);
    code_ids.wrappr721_code_id = wrappr721_code_id;
    let minter_collection_response = configure_wrappr_minter(
        &mut router,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    MinterTemplateResponse {
        router,
        collection_response_vec: minter_collection_response,
        accts: Accounts { creator, buyer },
    }
}

pub fn wrappr_minter_with_sudo_update_params_template(
    num_tokens: u32,
) -> MinterTemplateResponseCodeIds<Accounts> {
    let mut app = custom_mock_app();
    let (creator, buyer) = setup_accounts(&mut app);
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let code_ids = wrappr_minter_wrappr721_collection_code_ids(&mut app);
    let minter_collection_response: Vec<MinterCollectionResponse> = configure_wrappr_minter(
        &mut app,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids.clone(),
    );
    MinterTemplateResponseCodeIds {
        router: app,
        collection_response_vec: minter_collection_response,
        accts: Accounts { creator, buyer },
        code_ids,
    }
}
