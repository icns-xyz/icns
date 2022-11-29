#![cfg(test)]

use crate::{
    msg::{QueryMsg, GetAddressesResponse, GetAddressResponse},
    msg::{AdminResponse, ExecuteMsg},
    ContractError, contract::is_admin, tests::helpers::default_set_record,
};

use cosmwasm_std::{Addr, Empty, StdResult};
use cw_multi_test::{BasicApp, Executor};
use icns_name_nft::{msg::ExecuteMsg as NameExecuteMsg, msg::QueryMsg as NameQueryMsg};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};


use super::helpers::{
    instantiate_name_nft, instantiate_resolver_with_name_nft, TestEnv,
    default_setting,
    TestEnvBuilder,
};

#[test]
fn query_admins() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    // query admin
    let msg = QueryMsg::Admin {};

    // change from `let res: Vec<String> to this:
    let AdminResponse { admins } = app
        .wrap()
        .query_wasm_smart(resolver_contract_addr.clone(), &msg)
        .unwrap();
    
    assert_eq!(admins, vec![admin1, admin2]);
}

#[test]
fn query_addresses() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins.clone(), registrar.clone());

    // query addresses
    let GetAddressesResponse { addresses } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddresses {
                user_name: "bob".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        addresses,
        vec![
            ("cosmos".to_string(), "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string()),
            ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
        ]
    );

    // now add another user
    let mint = app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "alice".to_string(),
            owner: "alice".to_string(),
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

    // query addresses
    let GetAddressesResponse { addresses } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddresses {
                user_name: "bob".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        addresses,
        vec![
            ("osmo".to_string(), "osmo1t8qckan2yrygq7kl9apwhzfalwzgc242lk02ch".to_string()),
        ]
    );
}

#[test]
fn query_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins.clone(), registrar.clone());

    // query address
    let GetAddressResponse { address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddress {
                user_name: "bob".to_string(),
                bech32_prefix: "cosmos".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        address,
        "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string()
    );

    let GetAddressResponse { address } = app
    .wrap()
    .query_wasm_smart(
        resolver_contract_addr.clone(),
        &QueryMsg::GetAddress {
            user_name: "bob".to_string(),
            bech32_prefix: "random".to_string(),
        },
    )
        .unwrap();

    assert_eq!(
        address,
        "".to_string()
    );

    // now add another user
    let mint = app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "alice".to_string(),
            owner: "alice".to_string(),
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

    // query address
    let GetAddressResponse {address} = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddress {
                user_name: "alice".to_string(),
                bech32_prefix: "osmo".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        address,
        "osmo1t8qckan2yrygq7kl9apwhzfalwzgc242lk02ch".to_string()
    );
}