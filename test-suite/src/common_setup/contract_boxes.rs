use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::Empty;
use cw4_group::contract;
use cw_multi_test::{
    no_init, AppBuilder, BankKeeper, Contract, ContractWrapper, DistributionKeeper, FailingModule,
    GovFailingModule, IbcFailingModule, StakeKeeper, WasmKeeper,
};

use crate::common_setup::keeper::StargazeKeeper;
pub type App<ExecC = Empty, QueryC = Empty> = cw_multi_test::App<
    BankKeeper,
    MockApi,
    MockStorage,
    FailingModule<ExecC, QueryC, Empty>,
    WasmKeeper<ExecC, QueryC>,
    StakeKeeper,
    DistributionKeeper,
    IbcFailingModule,
    GovFailingModule,
    StargazeKeeper,
>;

pub fn custom_mock_app() -> App {
    AppBuilder::new()
        .with_stargate(StargazeKeeper)
        .build(no_init)
}



pub fn contract_wrappr_factory() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        wrappr_factory::contract::execute,
        wrappr_factory::contract::instantiate,
        wrappr_factory::contract::query,
    )
    .with_sudo(wrappr_factory::contract::sudo);
    Box::new(contract)
}

pub fn contract_wrappr_minter() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        wrappr_minter::contract::execute,
        wrappr_minter::contract::instantiate,
        wrappr_minter::contract::query,
    )
    .with_reply(wrappr_minter::contract::reply);
    Box::new(contract)
}


pub fn contract_wrappr721_base() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        wrappr721_base::entry::execute,
        wrappr721_base::entry::instantiate,
        wrappr721_base::entry::query,
    );
    Box::new(contract)
}

pub fn contract_group() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw4_group::contract::execute,
        cw4_group::contract::instantiate,
        cw4_group::contract::query,
    );
    Box::new(contract)
}