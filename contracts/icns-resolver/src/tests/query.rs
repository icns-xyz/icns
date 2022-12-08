#![cfg(test)]

use crate::{
    msg::AdminResponse,
    msg::{AddressResponse, AddressesResponse, QueryMsg, NamesResponse, IcnsNamesResponse, AddressByIcnsResponse}, tests::helpers::{signer2, ToBinary, mint_and_set_record}, crypto::cosmos_pubkey_to_bech32_address,
};

use cosmwasm_std::StdError;

use super::helpers::{default_setting, instantiate_name_nft, instantiate_resolver_with_name_nft, signer1};

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

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins, registrar.clone());
    
    // query addresses
    let AddressesResponse { addresses } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
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

    let addr2 = cosmos_pubkey_to_bech32_address(signer2().to_binary(), "cosmos".to_string());
    
    
    // try setting another record to ensure query works upon two or more records
    mint_and_set_record(
        &mut app,
        "bob",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    let AddressesResponse { addresses } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Addresses {
                name: "bob".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        addresses,
        vec![

            (
                "cosmos".to_string(),
                addr2
            )
        ]
    )
}

#[test]
fn query_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins, registrar.clone());

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
            resolver_contract_addr.clone(),
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

    // now add another user to ensure query works upon two or more user
    let addr2 = cosmos_pubkey_to_bech32_address(signer2().to_binary(), "cosmos".to_string());
    
    // try setting another record to ensure query works upon two or more records
    mint_and_set_record(
        &mut app,
        "bob",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    let AddressResponse { address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Address {
                name: "bob".to_string(),
                bech32_prefix: "cosmos".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        address,
        addr2.to_string()
    );

    // try getting unavailable address
    let err = app
        .wrap()
        .query_wasm_smart::<AddressResponse>(
            resolver_contract_addr.clone(),
            &QueryMsg::Address {
                name: "bob".to_string(),
                bech32_prefix: "juno".to_string(),
            },
        )
        .unwrap_err();

    assert_eq!(
        err,
        StdError::GenericErr {
            msg: "Querier contract error: alloc::string::String not found".to_string()
        }
    );
}

#[test]
fn query_names() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins, registrar.clone());

    let addr1 = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string());
    let addr2 = cosmos_pubkey_to_bech32_address(signer2().to_binary(), "cosmos".to_string());
    
    // try setting another record to ensure query works upon two or more records
    mint_and_set_record(
        &mut app,
        "bob",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    mint_and_set_record(
        &mut app,
        "charlie",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    // query addresses
    let NamesResponse { names, primary_name } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Names { address: addr1 },
        )
        .unwrap();

    assert_eq!(
        names,
        vec![
            "alice".to_string(),
        ]
    );
    // primary name should be set as the latest record that has been set for the bech32 address
    assert_eq!(
        primary_name,
        "alice".to_string()
    );
    // query addresses
    let NamesResponse { names, primary_name } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Names { address: addr2 },
        )
        .unwrap();


    assert_eq!(
        names,
        vec![
            "bob".to_string(),
            "charlie".to_string()
        ]
    );
    // primary name should be set as the latest record that has been set for the bech32 address
    assert_eq!(
        primary_name,
        "charlie".to_string()
    );
}

#[test]
fn query_icns_names() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins, registrar.clone());

    let addr1 = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string());
    let addr2 = cosmos_pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string());
    
    // try setting another record to ensure query works upon two or more records
    mint_and_set_record(
        &mut app,
        "bob",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    mint_and_set_record(
        &mut app,
        "charlie",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    // query addresses for alice
    let IcnsNamesResponse { names, primary_name } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::IcnsNames { address: addr1 },
        )
        .unwrap();
    assert_eq!(
        names,
        vec![
            "alice.cosmos".to_string(),
        ]
    );
    assert_eq!(
        primary_name,
        "alice.cosmos".to_string()
    );

    // query addresses for bob
    let IcnsNamesResponse { names, primary_name } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::IcnsNames { address: addr2 },
        )
        .unwrap();
    assert_eq!(
        names,
        vec![
            "bob.osmo".to_string(),
            "charlie.osmo".to_string()
        ]
    );
    assert_eq!(
        primary_name,
        "charlie.osmo".to_string()
    );
}

#[test]
fn query_address_by_icns() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins, registrar.clone());

    let addr1 = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string());
    let addr2 = cosmos_pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string());
    
    // try setting another record to ensure query works upon two or more records
    mint_and_set_record(
        &mut app,
        "bob",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    mint_and_set_record(
        &mut app,
        "charlie",
        addr2.clone(),
        &signer2(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    // query addresses for alice
    let AddressByIcnsResponse { bech32_address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::AddressByIcns { icns: "alice.cosmos".to_string() },
        )
        .unwrap();
    assert_eq!(
        bech32_address,
        addr1.to_string()
    );

    // query addresses for bob
    let AddressByIcnsResponse { bech32_address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::AddressByIcns { icns: "bob.osmo".to_string() },
        )
        .unwrap();
    assert_eq!(
        bech32_address,
        addr2.to_string()
    );

    let AddressByIcnsResponse { bech32_address } = app
        .wrap()
        .query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::AddressByIcns { icns: "bob.osmo".to_string() },
        )
        .unwrap();
    assert_eq!(
        bech32_address,
        addr2.to_string()
    );
}