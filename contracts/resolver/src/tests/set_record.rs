#![cfg(test)]

use crate::{
    contract::is_admin,
    msg::{AdminResponse, ExecuteMsg},
    msg::{GetAddressesResponse, QueryMsg},
    ContractError,
};

use cosmwasm_std::{testing::MockApi, Addr, Empty, StdResult};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};
use cosmwasm_crypto::{secp256k1_verify};
use cw_multi_test::{BasicApp, Executor};
use hex_literal::hex;
use icns_name_nft::{msg::ExecuteMsg as NameExecuteMsg, msg::QueryMsg as NameQueryMsg};
use subtle_encoding::hex;

use super::helpers::{
    default_setting, instantiate_name_nft, instantiate_resolver_with_name_nft, TestEnv,
    TestEnvBuilder,
};

use bech32::ToBase32;
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use std::ops::Deref;

#[test]
fn pubkey_to_address() {
    let original_hex = hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");

    let sha256 = Sha256::digest(original_hex);
    let result = Ripemd160::digest(sha256);

    assert_eq!(
        result.as_ref(),
        hex!("6aad751bb99fb0f13237f027b45eeebc37bec200")
    );

    let a = bech32::encode("osmo", result.deref().to_base32(), bech32::Variant::Bech32);

    dbg!(a.unwrap());
}

#[test]
fn secp256k1_verification() {
    let message = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"dGVzdA==\",\"signer\":\"osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697\"}}],\"sequence\":\"0\"}";
    let bytes = message.as_bytes();
    let hashed  = Sha256::digest(bytes);

    let signature = hex!("8c009e1fa58d6ae5dfcda93208f800dbd8815f20ea9c690b56a5758e999c9cb66fdb764b1e070d65ea22fe5827214631b1aba54730a9dfa74dc37b73da529c00");
    let pub_key = hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");


    let verify_result = secp256k1_verify(&hashed, &signature, &pub_key).unwrap();
    assert_eq!(verify_result, true);

    let false_signature = hex!("8c009e1fa58d6ae5dfcda93208f800dbd8815f20ea9c690b56a5758e999c9cb66fdb764b1e070d65ea22fe5827214631b1aba54730a9dfa74dc37b73da529c01");
    let verify_result = secp256k1_verify(&hashed, &false_signature, &pub_key).unwrap();
    assert_eq!(verify_result, false);

    let false_message = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"aW52YWxpZA==\",\"signer\":\"osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697\"}}],\"sequence\":\"0\"}";
    let bytes = false_message.as_bytes();
    let hashed  = Sha256::digest(bytes);

    let verify_result = secp256k1_verify(&hashed, &signature, &pub_key).unwrap();
    assert_eq!(verify_result, false);
}

#[test]
fn set_get_single_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    let (_, resolver_contract_addr, app) = default_setting(admins, registrar);
    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let GetAddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddresses { user_name: name },
        )?;

        Ok(addresses)
    };

    // now get record
    let addresses = addresses(&app, "bob".to_string()).unwrap();
    assert_eq!(addresses.len(), 2);
    assert_eq!(addresses[0].0, "cosmos");
    assert_eq!(
        addresses[0].1,
        "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe"
    );
    assert_eq!(addresses[1].0, "juno");
    assert_eq!(
        addresses[1].1,
        "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts"
    );
}

#[test]
fn set_duplicate_username() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    let (_, resolver_contract_addr, mut app) = default_setting(admins, registrar);

    // now set record again, this should error
    let err = app
        .execute_contract(
            Addr::unchecked(admin1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    (
                        "juno".to_string(),
                        "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string(),
                    ),
                    (
                        "cosmos".to_string(),
                        "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string(),
                    ),
                ],
            },
            &[],
        )
        .is_err();

    assert_eq!(err, true);
}

#[test]
fn bech32_verification() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    // mint name nft to bob
    let mint = app
        .execute_contract(
            Addr::unchecked(registrar.clone()),
            name_nft_contract.clone(),
            &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: "bob".to_string(),
                owner: "bob".to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // now set record, first try setting invalid bech32 address
    let err = app
        .execute_contract(
            Addr::unchecked(admin1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![(
                    "cosmos".to_string(),
                    "cosmos1dsfsfasdfknsfkndfknskdfns".to_string(),
                )],
            },
            &[],
        )
        .is_err();
    assert_eq!(err, true);

    // now try setting record with unmatching bech32 prefix and address
    let err = app
        .execute_contract(
            Addr::unchecked(admin1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![(
                    "cosmos".to_string(),
                    "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string(),
                )],
            },
            &[],
        )
        .is_err();
    assert_eq!(err, true);

    // now set record with valid bech32 prefix and addresses, this should succeed
    let err = app
        .execute_contract(
            Addr::unchecked(admin1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    (
                        "juno".to_string(),
                        "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string(),
                    ),
                    (
                        "cosmos".to_string(),
                        "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string(),
                    ),
                ],
            },
            &[],
        )
        .is_err();
    assert_eq!(err, false);
}
