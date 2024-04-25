#[cfg(test)]
mod tests {
    use crate::common_setup::contract_boxes::{
        contract_wrappr_factory, contract_wrappr_minter, App,
    };
    use anyhow::Error;
    use cosmwasm_std::{coin, Addr};
    use cw721::NumTokensResponse;
    use cw_multi_test::{AppResponse, BankSudo, Executor, SudoMsg};
    use wrappr721::ExecuteMsg as Wrappr721ExecuteMsg;
    use wrappr721::{CollectionInfo, InstantiateMsg};
    use wrappr_factory::helpers::FactoryContract;
    use wrappr_factory::msg::{ExecuteMsg, InstantiateMsg as FactoryInstantiateMsg};
    use wrappr_factory_utils::msg::CreateMinterMsg;
    use wrappr_factory_utils::tests::mock_collection_params;
    use wrappr_factory_utils::WrapprMinterInitMsgExtension;

    use crate::common_setup::contract_boxes::{contract_wrappr721_base, custom_mock_app};
    use crate::common_setup::setup_minter::common::constants::CREATION_FEE;
    use crate::common_setup::setup_minter::wrappr_minter::mock_params::{
        mock_create_minter, mock_init_extension, mock_params,
    };
    use cosmwasm_std::Empty;
    use cw721_base::msg::ExecuteMsg as cw721ExecuteMsg;
    use cw721_base::Ownership;

    const GOVERNANCE: &str = "governance";
    const ADMIN: &str = "admin";
    const NATIVE_DENOM: &str = "ustars";

    pub fn assert_error(res: Result<AppResponse, Error>, expected: String) {
        assert_eq!(res.unwrap_err().source().unwrap().to_string(), expected);
    }

    fn proper_instantiate_factory() -> (App, FactoryContract) {
        let mut app = custom_mock_app();
        let factory_id = app.store_code(contract_wrappr_factory());
        let minter_id = app.store_code(contract_wrappr_minter());

        let mut params = mock_params();
        params.code_id = minter_id;

        let msg = FactoryInstantiateMsg { params };
        let factory_addr = app
            .instantiate_contract(
                factory_id,
                Addr::unchecked(GOVERNANCE),
                &msg,
                &[],
                "factory",
                Some(GOVERNANCE.to_string()),
            )
            .unwrap();

        let factory_contract = FactoryContract(factory_addr);

        (app, factory_contract)
    }

    fn proper_instantiate() -> (App, Addr) {
        let (mut app, factory_contract) = proper_instantiate_factory();
        let wrappr721_id = app.store_code(contract_wrappr721_base());

        let collection_params = mock_collection_params();
        let mut m = mock_create_minter(collection_params);
        m.collection_params.code_id = wrappr721_id;
        let msg = ExecuteMsg::CreateMinter(m);

        let creation_fee = coin(CREATION_FEE, NATIVE_DENOM);

        app.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: ADMIN.to_string(),
            amount: vec![creation_fee.clone()],
        }))
        .unwrap();

        let bal = app.wrap().query_all_balances(ADMIN).unwrap();
        assert_eq!(bal, vec![creation_fee.clone()]);

        // this should create the minter + wrappr721
        let cosmos_msg = factory_contract.call_with_funds(msg, creation_fee).unwrap();

        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        assert!(res.is_ok());

        (app, Addr::unchecked("contract2"))
    }

    fn custom_proper_instantiate(
        custom_create_minter_msg: CreateMinterMsg<WrapprMinterInitMsgExtension>,
    ) -> (App, Addr) {
        let (mut app, factory_contract) = proper_instantiate_factory();
        let wrappr721_id = app.store_code(contract_wrappr721_base());

        let mut m = custom_create_minter_msg;
        m.collection_params.code_id = wrappr721_id;
        let msg = ExecuteMsg::CreateMinter(m);

        let creation_fee = coin(CREATION_FEE, NATIVE_DENOM);

        app.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: ADMIN.to_string(),
            amount: vec![creation_fee.clone()],
        }))
        .unwrap();

        let bal = app.wrap().query_all_balances(ADMIN).unwrap();
        assert_eq!(bal, vec![creation_fee.clone()]);

        // this should create the minter + wrappr721
        let cosmos_msg = factory_contract.call_with_funds(msg, creation_fee).unwrap();

        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        assert!(res.is_ok());

        (app, Addr::unchecked("contract2"))
    }

    mod init {
        use cw721_base::MinterResponse;
        use wrappr_minter::msg::ConfigResponse;

        use crate::common_setup::setup_minter::wrappr_minter::mock_params::mock_init_extension;

        use super::*;
        use wrappr721_base::msg::QueryMsg;
        use wrappr_minter_utils::QueryMsg as WrapprMinterQueryMsg;

        #[test]
        fn create_wrappr721_base_collection() {
            let (app, contract) = proper_instantiate();

            let res: NumTokensResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::NumTokens {})
                .unwrap();
            assert_eq!(res.count, 0);
        }

        #[test]
        fn check_ready_unauthorized() {
            let mut app = custom_mock_app();
            let wrappr721_id = app.store_code(contract_wrappr721_base());
            let msg = InstantiateMsg {
                name: "wrappr721".to_string(),
                symbol: "STARGAZE".to_string(),
                minter: ADMIN.to_string(),
                collection_info: CollectionInfo {
                    creator: ADMIN.to_string(),
                    description: "description".to_string(),
                    image: "description".to_string(),
                    external_link: None,
                    // royalty_info: None,
                },
            };
            let res = app.instantiate_contract(
                wrappr721_id,
                Addr::unchecked(GOVERNANCE),
                &msg,
                &[],
                "wrappr721-only",
                None,
            );
            // should not let create the contract.
            assert!(res.is_err());
        }

        #[test]
        fn check_ready_authorized() {
            let (_, _) = proper_instantiate();
        }
    }

    mod start_trading_time {
        use cosmwasm_std::{Decimal, Empty, Timestamp};
        use wrappr721::UpdateCollectionInfoMsg;

        use crate::common_setup::{
            setup_accounts_and_block::setup_block_time,
            setup_minter::wrappr_minter::{
                mock_params::mock_create_minter, setup::mock_create_minter_init_msg,
            },
        };

        use super::*;
        use wrappr721_base::{
            msg::{CollectionInfoResponse, QueryMsg},
            ContractError,
        };

        #[test]
        fn update_collection_info() {
            // customize params so external_link is None
            let mut params = mock_collection_params();
            params.info.external_link = None;
            let custom_create_minter_msg =
                mock_create_minter_init_msg(params.clone(), mock_init_extension(None, None));
            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg.clone());

            // // default trading start time is start time + default trading start time offset
            // let res: CollectionInfoResponse = app
            //     .wrap()
            //     .query_wasm_smart(contract, &QueryMsg::CollectionInfo {})
            //     .unwrap();
            // let default_start_time = mock_init_extension(None, None)
            //     .start_time
            //     .plus_seconds(mock_params().max_trading_offset_secs);
            // assert_eq!(res.start_trading_time, Some(default_start_time));

            // update collection info
            let (mut app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            let creator = Addr::unchecked("creator".to_string());
            let new_creator = Addr::unchecked("new_creator".to_string());

            // succeeds
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Wrappr721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        creator: None,
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                    },
                },
                &[],
            );
            assert!(res.is_ok());

            // update explicit content with new creator
            let res = app.execute_contract(
                creator,
                contract.clone(),
                &Wrappr721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        creator: Some(new_creator.to_string()),
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                    },
                },
                &[],
            );
            assert!(res.is_ok());

            // let res: CollectionInfoResponse = app
            //     .wrap()
            //     .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
            //     .unwrap();
            // // check explicit content changed to true
            // assert!(res.explicit_content.unwrap());

            // freeze collection throw err if not creator
            let res = app.execute_contract(
                Addr::unchecked("badguy"),
                contract.clone(),
                &Wrappr721ExecuteMsg::<Empty, Empty>::FreezeCollectionInfo {},
                &[],
            );
            assert!(res.is_err());
            // freeze collection to prevent further updates
            let res = app.execute_contract(
                new_creator.clone(),
                contract.clone(),
                &Wrappr721ExecuteMsg::<Empty, Empty>::FreezeCollectionInfo {},
                &[],
            );
            assert!(res.is_ok());

            // trying to update collection after frozen should throw err
            let res = app.execute_contract(
                new_creator,
                contract,
                &Wrappr721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        creator: None,
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link),
                    },
                },
                &[],
            );
            assert!(res.is_err());
        }
    }
    mod ownership {
        use cosmwasm_std::Attribute;
        use cw721_base::MinterResponse;

        use crate::common_setup::setup_minter::wrappr_minter::setup::mock_create_minter_init_msg;

        use super::*;
        use wrappr721_base::msg::QueryMsg;
        use wrappr_minter::msg::ConfigResponse;
        use wrappr_minter_utils::QueryMsg as WrapprMinterQueryMsg;

        #[test]
        fn test_update_ownership() {
            let base_token_uri = "ipfs://somecidhere".to_string();
            let init_msg = WrapprMinterInitMsgExtension {
                ..mock_init_extension(None, None)
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(mock_collection_params(), init_msg);
            let (mut app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            // query minter config to confirm base_token_uri got trimmed
            let res: MinterResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::Minter {})
                .unwrap();
            let minter = res.minter;
            let minter = minter.unwrap();
            let res: ConfigResponse = app
                .wrap()
                .query_wasm_smart(minter.clone(), &WrapprMinterQueryMsg::Config {})
                .unwrap();
            let wrappr721_address = res.collection_address;

            let update_ownership_msg: cw721ExecuteMsg<Empty, Empty> =
                cw721ExecuteMsg::UpdateOwnership(cw_ownable::Action::TransferOwnership {
                    new_owner: "new_owner".to_string(),
                    expiry: None,
                });
            let res = app.execute_contract(
                Addr::unchecked(minter),
                Addr::unchecked(wrappr721_address.clone()),
                &update_ownership_msg,
                &[],
            );
            let attribute_owner_response = res.unwrap().events[1].clone().attributes[2].clone();
            let expected_attribute = Attribute {
                key: "pending_owner".to_string(),
                value: "new_owner".to_string(),
            };
            assert_eq!(attribute_owner_response, expected_attribute);
            let res: cw_ownable::Ownership<Addr> = app
                .wrap()
                .query_wasm_smart(
                    wrappr721_address.clone(),
                    &wrappr721_base::msg::QueryMsg::Ownership {},
                )
                .unwrap();
            let pending_owner = res.pending_owner;
            let expected_pending_owner = Some(Addr::unchecked("new_owner".to_string()));
            assert_eq!(pending_owner, expected_pending_owner);

            let accept_ownership_msg: cw721ExecuteMsg<Empty, Empty> =
                cw721ExecuteMsg::UpdateOwnership(cw_ownable::Action::AcceptOwnership {});
            let res = app.execute_contract(
                Addr::unchecked("new_owner".to_string()),
                Addr::unchecked(wrappr721_address.clone()),
                &accept_ownership_msg,
                &[],
            );
            let pending_owner_response = res.unwrap().events[1].clone().attributes[2].clone();
            let expected_pending_owner_response = Attribute {
                key: "pending_owner".to_string(),
                value: "none".to_string(),
            };
            assert_eq!(pending_owner_response, expected_pending_owner_response);

            let res: cw_ownable::Ownership<Addr> = app
                .wrap()
                .query_wasm_smart(
                    wrappr721_address,
                    &wrappr721_base::msg::QueryMsg::Ownership {},
                )
                .unwrap();

            let expected_onwership_response = Ownership {
                owner: Some(Addr::unchecked("new_owner".to_string())),
                pending_owner: None,
                pending_expiry: None,
            };
            assert_eq!(res, expected_onwership_response);
        }
    }
}
