use std::ops::Add;
use hex_literal::hex;

use cosmwasm_std::{Empty, Binary};
use crate::{entry, msg::{InstantiateMsg, AddressInfo}, contract::execute, contract::instantiate, contract::query, msg::{ExecuteMsg, self}};
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

pub fn default_set_record() -> ExecuteMsg {
    let original_pubkey_vec = hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");
    let original_signature_vec = hex!("69c865c686a4b141297fee846e16a0f9c8df965fe64abea4513f653c8a3b385019f81c93081a2f3c0930c5cd3265bf621af863f48a2a9a54f8883d4a54d2c3d2");
    let pub_key = Binary::from(original_pubkey_vec);
    let signature = Binary::from(original_signature_vec);

    ExecuteMsg::SetRecord {
        user_name: "bob".to_string(),
        address_info: AddressInfo{
            bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            address_hash: msg::AddressHash::SHA256,
            pub_key,
            signature,
        },
        bech32_prefix: "osmo".to_string(),
        replace_primary_if_exists: false,
        signature_salt: 1323124,
    }
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
        &default_set_record(), 
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