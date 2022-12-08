use cosmrs::{bip32, crypto::secp256k1::SigningKey, tendermint::signature::Secp256k1Signature};
use cosmwasm_std::{Addr, Binary, Decimal, Empty, Coin};
use cw_multi_test::{BasicApp, Contract, ContractWrapper, Executor};
use icns_name_nft::msg::ICNSNameExecuteMsg;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    ContractError,
};

use self::fixtures::{verifier1, verifier2, verifier3, verifier4};

pub fn name_nft_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        icns_name_nft::entry::execute,
        icns_name_nft::entry::instantiate,
        icns_name_nft::entry::query,
    );
    Box::new(contract)
}

pub fn registrar_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn default_contracts_setup(
    app: &mut BasicApp,
    name_nft_code_id: u64,
    registrar_code_id: u64,
    admins: Vec<String>,
    fee: Option<Coin>
) -> (Addr, Addr) {
    // setup name nft contract
    let name_nft_contract_addr = app
        .instantiate_contract(
            name_nft_code_id,
            Addr::unchecked(admins[0].clone()),
            &icns_name_nft::InstantiateMsg {
                admins: admins.clone(),
                transferrable: false,
            },
            &[],
            "name",
            None,
        )
        .unwrap();

    let registrar_contract_addr = app
        .instantiate_contract(
            registrar_code_id,
            Addr::unchecked(admins[0].clone()),
            &InstantiateMsg {
                name_nft_addr: name_nft_contract_addr.to_string(),
                verifier_pubkeys: vec![verifier1(), verifier2(), verifier3(), verifier4()]
                    .iter()
                    .map(|v| v.to_binary())
                    .collect(),
                verification_threshold: Decimal::percent(50),
                fee,
            },
            &[],
            "registar",
            None,
        )
        .unwrap();

    // now set registrar as name nft minter
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        name_nft_contract_addr.clone(),
        &icns_name_nft::msg::ExecuteMsg::Extension {
            msg: ICNSNameExecuteMsg::SetMinter {
                minter_address: registrar_contract_addr.to_string(),
            },
        },
        &[],
    )
        .unwrap();

    (name_nft_contract_addr, registrar_contract_addr)
}

pub fn test_only_admin<T>(execute_msg: ExecuteMsg, query_msg: QueryMsg, initial: T, updated: T)
where
    T: DeserializeOwned + PartialEq + Debug,
{
    // setup contracts
    let mut app = BasicApp::default();
    let name_nft_code_id = app.store_code(name_nft_contract());
    let registrar_code_id = app.store_code(registrar_contract());
    let admins = vec!["admin1".to_string(), "admin2".to_string()];

    // setup contracts
    let name_nft_contract_addr = app
        .instantiate_contract(
            name_nft_code_id,
            Addr::unchecked(admins[0].clone()),
            &icns_name_nft::InstantiateMsg {
                admins: admins.clone(),
                transferrable: false,
            },
            &[],
            "name",
            None,
        )
        .unwrap();

    let registrar_contract_addr = app
        .instantiate_contract(
            registrar_code_id,
            Addr::unchecked(admins[0].clone()),
            &InstantiateMsg {
                name_nft_addr: name_nft_contract_addr.to_string(),
                verifier_pubkeys: vec![verifier2().to_binary()],
                verification_threshold: Decimal::percent(50),
                fee: None,
            },
            &[],
            "registar",
            None,
        )
        .unwrap();

    let response: T = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr.clone(), &query_msg)
        .unwrap();

    assert_eq!(response, initial);

    // unauthorized if not admin
    let err = app
        .execute_contract(
            Addr::unchecked("random_guy"),
            registrar_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    let response: T = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr.clone(), &query_msg)
        .unwrap();

    assert_eq!(response, initial);

    // authorized if admin
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        registrar_contract_addr.clone(),
        &execute_msg,
        &[],
    )
    .unwrap();

    let response: T = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr, &query_msg)
        .unwrap();

    assert_eq!(response, updated);
}

pub fn from_mnemonic(phrase: &str, derivation_path: &str) -> SigningKey {
    let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse().unwrap()).unwrap();
    xprv.into()
}

pub trait ToBinary {
    fn to_binary(&self) -> Binary;
}

impl ToBinary for SigningKey {
    fn to_binary(&self) -> Binary {
        Binary(self.public_key().to_bytes())
    }
}

impl ToBinary for Secp256k1Signature {
    fn to_binary(&self) -> Binary {
        Binary(self.to_vec())
    }
}

pub mod fixtures {
    use super::*;

    const DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";
    pub fn verifier1() -> SigningKey {
        from_mnemonic("notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius", DERIVATION_PATH)
    }

    pub fn verifier2() -> SigningKey {
        from_mnemonic("quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty", DERIVATION_PATH)
    }
    pub fn verifier3() -> SigningKey {
        from_mnemonic("symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb", DERIVATION_PATH)
    }
    pub fn verifier4() -> SigningKey {
        from_mnemonic("bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty", DERIVATION_PATH)
    }
    pub fn non_verifier() -> SigningKey {
        from_mnemonic("prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder", DERIVATION_PATH)
    }
}
