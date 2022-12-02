#![cfg(test)]

use crate::{
    msg::AdminResponse,
    msg::{GetAddressResponse, GetAddressesResponse, QueryMsg},
};

use cosmwasm_std::{Addr, Empty};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};
use cw_multi_test::Executor;
use icns_name_nft::msg::ExecuteMsg as NameExecuteMsg;

use super::helpers::{default_setting, instantiate_name_nft, instantiate_resolver_with_name_nft};

#[test]
fn query_admins() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar);

    // now instantiate resolver using name nft contract
    let resolver_contract_addr = instantiate_resolver_with_name_nft(&mut app, name_nft_contract);

    // query admin
    let msg = QueryMsg::Admin {};

    // change from `let res: Vec<String> to this:
    let AdminResponse { admins } = app
        .wrap()
        .query_wasm_smart(resolver_contract_addr, &msg)
        .unwrap();

    assert_eq!(admins, vec![admin1, admin2]);
}

#[test]
fn query_addresses() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) =
        default_setting(admins, registrar.clone());

    // query addresses
    let GetAddressesResponse { addresses } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr,
            &QueryMsg::GetAddresses {
                user_name: "tony".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        addresses,
        vec![
            (
                "juno".to_string(),
                "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s".to_string()
            ),
            (
                "osmo".to_string(),
                "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string()
            )
        ]
    );

    // now add another user
    let mint = app
        .execute_contract(
            Addr::unchecked(registrar),
            name_nft_contract,
            &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: "alice".to_string(),
                owner: "alice".to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // app.execute_contract(
    //     Addr::unchecked(admins[0].clone()),
    //     resolver_contract_addr.clone(),
    //     &default_record_msg(),
    //     &[],
    // ).unwrap();

    // // query addresses
    // let GetAddressesResponse { addresses } = app
    //     .wrap()
    //     .query_wasm_smart(
    //         resolver_contract_addr.clone(),
    //         &QueryMsg::GetAddresses {
    //             user_name: "bob".to_string(),
    //         },
    //     )
    //     .unwrap();

    // assert_eq!(
    //     addresses,
    //     vec![
    //         ("osmo".to_string(), "osmo1t8qckan2yrygq7kl9apwhzfalwzgc242lk02ch".to_string()),
    //     ]
    // );
}

#[test]
fn query_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) =
        default_setting(admins, registrar.clone());

    // query address
    let GetAddressResponse { address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddress {
                user_name: "tony".to_string(),
                bech32_prefix: "osmo".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        address,
        "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string()
    );

    let GetAddressResponse { address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddress {
                user_name: "tony".to_string(),
                bech32_prefix: "juno".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        address,
        "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s".to_string()
    );

    let GetAddressResponse { address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr,
            &QueryMsg::GetAddress {
                user_name: "tony".to_string(),
                bech32_prefix: "random".to_string(),
            },
        )
        .unwrap();

    assert_eq!(address, "".to_string());

    // now add another user
    let mint = app
        .execute_contract(
            Addr::unchecked(registrar),
            name_nft_contract,
            &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: "alice".to_string(),
                owner: "alice".to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // app.execute_contract(
    //     Addr::unchecked(admins[0].clone()),
    //     resolver_contract_addr.clone(),
    //     &default_record_msg(),
    //     &[],
    // ).unwrap();

    // // query address
    // let GetAddressResponse {address} = app
    //     .wrap()
    //     .query_wasm_smart(
    //         resolver_contract_addr.clone(),
    //         &QueryMsg::GetAddress {
    //             user_name: "alice".to_string(),
    //             bech32_prefix: "osmo".to_string(),
    //         },
    //     )
    //     .unwrap();

    // assert_eq!(
    //     address,
    //     "osmo1t8qckan2yrygq7kl9apwhzfalwzgc242lk02ch".to_string()
    // );
}
