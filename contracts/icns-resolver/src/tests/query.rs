#![cfg(test)]

use crate::{
    msg::AdminResponse,
    msg::{AddressResponse, AddressesResponse, QueryMsg}, tests::helpers::{signer2, ToBinary}, crypto::cosmos_pubkey_to_bech32_address,
};

use cosmwasm_std::StdError;

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

    let (_name_nft_contract, resolver_contract_addr, app) = default_setting(admins, registrar);

    // query addresses
    let AddressesResponse { addresses } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr,
            &QueryMsg::Addresses {
                name: "alice".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        addresses,
        vec![
            (
                "cosmos".to_string(),
                "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string()
            ),
            (
                "juno".to_string(),
                "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s".to_string()
            )
        ]
    );
    let _addr2 = cosmos_pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string());
    
    
    // mint_and_set_record(
    //     &mut app,
    //     "bob",
    //     addr2.clone(),
    //     &signer2(),
    //     registrar.clone(),
    //     name_nft_contract.clone(),
    //     resolver_contract_addr.clone(),
    // );

    // let AddressesResponse { addresses } = app
    //     .wrap()
    //     .query_wasm_smart(
    //         resolver_contract_addr.clone(),
    //         &QueryMsg::Addresses {
    //             name: "bob".to_string(),
    //         },
    //     )
    //     .unwrap();
    // println!("addresses: {:?}", addresses);
}

#[test]
fn query_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (_name_nft_contract, resolver_contract_addr, app) = default_setting(admins, registrar);

    // query address
    let AddressResponse { address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Address {
                name: "alice".to_string(),
                bech32_prefix: "cosmos".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        address,
        "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string()
    );

    let AddressResponse { address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Address {
                name: "alice".to_string(),
                bech32_prefix: "juno".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        address,
        "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s".to_string()
    );

    let err = app
        .wrap()
        .query_wasm_smart::<AddressResponse>(
            resolver_contract_addr,
            &QueryMsg::Address {
                name: "tony".to_string(),
                bech32_prefix: "random".to_string(),
            },
        )
        .unwrap_err();

    assert_eq!(
        err,
        StdError::GenericErr {
            msg: "Querier contract error: alloc::string::String not found".to_string()
        }
    );

    // now add another user
    // let mint = app
    //     .execute_contract(
    //         Addr::unchecked(registrar),
    //         name_nft_contract,
    //         &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
    //             token_id: "alice".to_string(),
    //             owner: "alice".to_string(),
    //             token_uri: None,
    //             extension: None,
    //         })),
    //         &[],
    //     )
    //     .is_err();
    // assert_eq!(mint, false);

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
    //             name: "alice".to_string(),
    //             bech32_prefix: "osmo".to_string(),
    //         },
    //     )
    //     .unwrap();

    // assert_eq!(
    //     address,
    //     "osmo1t8qckan2yrygq7kl9apwhzfalwzgc242lk02ch".to_string()
    // );
}
