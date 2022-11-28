use std::ops::Add;

use cosmwasm_std::Empty;
use crate::{entry, msg::InstantiateMsg, contract::execute, contract::instantiate, contract::query, msg::ExecuteMsg};
// import execute


use cw_multi_test::{next_block, App,BasicApp, Contract, BankSudo, ContractWrapper, Executor, SudoMsg};
use cosmwasm_std::{to_binary, Addr, Coin, Uint128};

use icns_name_nft::{self, msg::ExecuteMsg as NameExecuteMsg};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};


pub struct TestEnv {
    pub app: BasicApp,
    pub code_id: u64,
    pub contract_addr: Addr,
    pub name_nft: Addr
}

pub struct TestEnvBuilder {
    pub name_nft: Addr,
}

impl Default for TestEnvBuilder {
    fn default() -> Self {
        Self {
            name_nft: Addr::unchecked("name_nft"),
        }
    }
}
impl TestEnvBuilder {
    pub fn with_name_nft_contract(self, name_nft: Addr) -> Self {
        Self { name_nft }
    }

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
    }

}

pub fn resolver_contract() -> Box<dyn  Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
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

pub fn default_setting(
    admins: Vec<String>,
    registrar: String,
) -> (Addr, Addr, App){
    let (name_nft_contract, mut app) = instantiate_name_nft(admins.clone(), registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());
    
    //  mint name nft to bob
    let mint = app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "bob".to_string(),
            owner: "bob".to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    ).is_err();
    assert_eq!(mint, false);

    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                    ("cosmos".to_string(), "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string()),
                ],
            }, 
        &[],
    ).unwrap();

    return (name_nft_contract, resolver_contract_addr, app);
}

pub fn instantiate_name_nft(
    admins: Vec<String>, 
    registrar: String,
) -> (Addr, App)  {
    let mut app = BasicApp::default();
    let name_nft = app.store_code(name_nft_contract());

    let nft_address = app
        .instantiate_contract(
            name_nft, 
            Addr::unchecked("example"),
                &icns_name_nft::msg::InstantiateMsg{
                    // registrar: registrar,
                    admins: admins,
                    transferrable: false,
                },
                &[],
                "name-nft",
                None,
        )
        .unwrap();
    
    (nft_address, app)
}

pub fn instantiate_resolver_with_name_nft(
    app: &mut BasicApp,
    name_nft: Addr,
) -> Addr {
    let code_id = app.store_code(resolver_contract());

    let sender = Addr::unchecked("sender");

    let contract_addr = app
        .instantiate_contract(
            code_id,
            sender,
            &InstantiateMsg{
                name_address: name_nft.to_string(),
            },
            &[],
            "resolver", 
            None,
        )
        .unwrap();

    contract_addr
}