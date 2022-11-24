use std::ops::Add;

use cosmwasm_std::Empty;
use crate::{entry, msg::InstantiateMsg};

use cw_multi_test::{next_block, App,BasicApp, Contract, BankSudo, ContractWrapper, Executor, SudoMsg};
use cosmwasm_std::{to_binary, Addr, Coin, Uint128};

use icns_name_nft;
use schemars::_serde_json::map::Entry;

pub struct TestEnv {
    pub app: BasicApp,
    pub code_id: u64,
    pub contract_addr: Addr,
    pub name_nft: Addr
}

pub struct TestEnvBuilder {
    pub name_nft: Addr,
}

impl TestEnvBuilder {
    pub fn build(self) -> TestEnv {
        let mut app = BasicApp::default();
        let code_id = app.store_code(resolver_contract());

        let sender = Addr::unchecked("sender");

        let contract_addr = app
            .instantiate_contract(
                code_id,
                sender,
                &InstantiateMsg{
                    name_address: self.name_nft.to_string(),
                },
                &[],
                "resolver", 
                None,
            )
            .unwrap();

            TestEnv {
                app,
                code_id,
                contract_addr,
                name_nft: self.name_nft,
            }



        // let mut app = B
    }
}

pub fn resolver_contract() -> Box<dyn  Contract<Empty>> {
    let contract = ContractWrapper::new(entry::execute, entry::instantiate, entry::query);
    Box::new(contract)
}
pub fn name_nft_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        icns_name_nft::entry::execute,
        icns_name_nft::entry::instantiate,
        icns_name_nft::entry::query,
    );

    Box::new(contract)
}

pub fn instantiate_with_name_nft_and_admins(
    app: &mut App,
    admins: Vec<String>
) -> Addr  {
    let name_nft = app.store_code(name_nft_contract());

    let nft_address = app
        .instantiate_contract(
            name_nft, 
            Addr::unchecked("example"),
                &icns_name_nft::msg::InstantiateMsg{
                    registrar: String::from("default-registrar"),
                    admins: admins,
                    transferrable: false,
                },
                &[],
                "name-nft",
                None,
        )
        .unwrap();
    
    nft_address
}