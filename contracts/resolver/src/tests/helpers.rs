use crate::{
    contract::execute,
    contract::instantiate,
    contract::query,
    crypto::{create_adr36_message, pubkey_to_bech32_address},
    msg::{self, ExecuteMsg},
    msg::{Adr36Info, InstantiateMsg, PrimaryNameResponse, QueryMsg, AddressesResponse},
};
use cosmrs::{bip32, crypto::secp256k1::SigningKey, tendermint::signature::Secp256k1Signature};
use cosmwasm_std::{Binary, Empty, StdResult};
use hex_literal::hex;
// import execute

use cosmwasm_std::Addr;
use cw_multi_test::{App, BasicApp, Contract, ContractWrapper, Executor};

use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};
use icns_name_nft::{self, msg::ExecuteMsg as NameExecuteMsg, msg::ICNSNameExecuteMsg::SetMinter};

pub fn resolver_contract() -> Box<dyn Contract<Empty>> {
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

pub fn default_osmo_set_record_msg() -> ExecuteMsg {
    {
        let original_pubkey_vec =
            hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");
        let original_signature_vec = hex!("74331c35c9dd49eb3d39f693afc363e77e5541d94839639b7c71e2f18b001295561f123cb169128a34aedb15dddd1caa42e3cbc39104cb07a32658e9de5707a1");
        let pub_key = Binary::from(original_pubkey_vec);
        let signature = Binary::from(original_signature_vec);

        ExecuteMsg::SetRecord {
            name: "tony".to_string(),
            adr36_info: Adr36Info {
                bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
                address_hash: msg::AddressHash::SHA256,
                pub_key,
                signature,
            },
            bech32_prefix: "osmo".to_string(),
            signature_salt: 1323124u128.into(),
        }
    }
}

pub fn default_juno_set_record_msg() -> ExecuteMsg {
    {
        let original_pubkey_vec =
            hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");
        let original_signature_vec = hex!("1d2048b59cc0fa1799bdc11695fb31d141429ef80c7223afb9eb6581ca7a4e1d38c8e9b70852110efbc41d59b3b0d40a9b0257dd3c34da0243cca60eea35edb1");
        let pub_key = Binary::from(original_pubkey_vec);
        let signature = Binary::from(original_signature_vec);

        ExecuteMsg::SetRecord {
            name: "tony".to_string(),
            adr36_info: Adr36Info {
                bech32_address: "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s".to_string(),
                address_hash: msg::AddressHash::SHA256,
                pub_key,
                signature,
            },
            bech32_prefix: "juno".to_string(),
            signature_salt: 13231u128.into(),
        }
    }
}

pub fn default_setting(admins: Vec<String>, registrar: String) -> (Addr, Addr, App) {
    let (name_nft_contract, mut app) = instantiate_name_nft(admins.clone(), registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    //  mint name nft to tony
    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "tony".to_string(),
            owner: "tony".to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        resolver_contract_addr.clone(),
        &default_osmo_set_record_msg(),
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        resolver_contract_addr.clone(),
        &default_juno_set_record_msg(),
        &[],
    )
    .unwrap();

    (name_nft_contract, resolver_contract_addr, app)
}

pub fn instantiate_name_nft(admins: Vec<String>, registrar: String) -> (Addr, App) {
    let mut app = BasicApp::default();
    let name_nft = app.store_code(name_nft_contract());

    let nft_address = app
        .instantiate_contract(
            name_nft,
            Addr::unchecked("example"),
            &icns_name_nft::msg::InstantiateMsg {
                admins: admins.clone(),
                transferrable: false,
            },
            &[],
            "name-nft",
            None,
        )
        .unwrap();

    // now that nft contract has been instantiated, set registrar in the nft contract
    // set minter as registrar
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        nft_address.clone(),
        &NameExecuteMsg::ICNSName(SetMinter {
            minter_address: registrar,
        }),
        &[],
    )
    .unwrap();

    (nft_address, app)
}

pub fn instantiate_resolver_with_name_nft(app: &mut BasicApp, name_nft: Addr) -> Addr {
    let code_id = app.store_code(resolver_contract());

    let sender = Addr::unchecked("sender");

    app.instantiate_contract(
        code_id,
        sender,
        &InstantiateMsg {
            name_address: name_nft.to_string(),
        },
        &[],
        "resolver",
        None,
    )
    .unwrap()
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

const DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";
pub fn signer1() -> SigningKey {
    from_mnemonic("notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius", DERIVATION_PATH)
}
pub fn signer2() -> SigningKey {
    from_mnemonic("quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty", DERIVATION_PATH)
}


pub fn mint_and_set_record(
    app: &mut BasicApp,
    name: &str,
    signer: &SigningKey,
    registrar: String,
    name_nft_contract: Addr,
    resolver_contract_addr: Addr,
) {
    let addr = pubkey_to_bech32_address(signer.to_binary(), "osmo".to_string());

    app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: name.to_string(),
            owner: addr.to_string(),
            token_uri: None,
            extension: None,
        }),
        &[],
    )
    .unwrap();

    let multitest_chain_id = "cosmos-testnet-14002";

    let msg = create_adr36_message(
        name.to_string(),
        "osmo".to_string(),
        addr.to_string(),
        multitest_chain_id.to_string(),
        resolver_contract_addr.to_string(),
        12313,
    );

    let signature = signer.sign(msg.as_bytes()).unwrap().to_binary();

    let msg = ExecuteMsg::SetRecord {
        name: name.to_string(),
        adr36_info: Adr36Info {
            bech32_address: addr.to_string(),
            address_hash: msg::AddressHash::SHA256,
            pub_key: signer.to_binary(),
            signature,
        },
        bech32_prefix: "osmo".to_string(),
        signature_salt: 12313u128.into(),
    };

    app.execute_contract(
        Addr::unchecked(addr),
        resolver_contract_addr.clone(),
        &msg,
        &[],
    )
    .unwrap();
}

pub fn primary_name(app: &BasicApp, address: String, resolver_contract_addr: Addr) -> StdResult<String> {
    let PrimaryNameResponse { name } = app.wrap().query_wasm_smart(
        resolver_contract_addr.clone(),
        &QueryMsg::PrimaryName { address },
    )?;

    Ok(name)
}

pub fn addresses(app: &BasicApp, name: String, resolver_contract_addr: Addr) -> StdResult<Vec<(String, String)>> {
    let AddressesResponse { addresses } = app.wrap().query_wasm_smart(
        resolver_contract_addr.clone(),
        &QueryMsg::Addresses { name },
    )?;

    Ok(addresses)
}