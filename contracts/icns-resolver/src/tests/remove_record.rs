#![cfg(test)]

use crate::{
    crypto::cosmos_pubkey_to_bech32_address,
    msg::{ExecuteMsg, Bech32Address},
    tests::helpers::{
        addresses, instantiate_name_nft, instantiate_resolver_with_name_nft, mint_and_set_record,
        primary_name, signer2,
    },
    ContractError,
};

use cosmwasm_std::{Addr, StdError};

use cw_multi_test::Executor;

use super::helpers::{signer1, ToBinary};

#[test]
fn remove_with_single_name_for_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(
        &mut app,
        "isabel",
        signer_bech32_address.clone(),
        &signer1(),
        registrar,
        name_nft_contract,
        resolver_contract_addr.clone(),
    );

    app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel".to_string(),
            bech32_address: signer_bech32_address,
        },
        &[],
    )
    .unwrap();

    // now check primary name and addresses
    assert_eq!(
        primary_name(&app, addr1, resolver_contract_addr.clone()).unwrap(),
        "".to_string()
    );

    // should have nothing as addresses
    assert_eq!(
        addresses(&app, "isabel".to_string(), resolver_contract_addr).unwrap(),
        Vec::<Bech32Address>::new()
    );
}

#[test]
fn remove_primary_and_non_primary_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(
        &mut app,
        "isabel",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    mint_and_set_record(
        &mut app,
        "isabel2",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    mint_and_set_record(
        &mut app,
        "isabel3",
        signer_bech32_address.clone(),
        &signer1(),
        registrar,
        name_nft_contract,
        resolver_contract_addr.clone(),
    );

    // when address has more than 1 name, that address can't remove primary name
    // need to set primary name to another name first
    let err = app
        .execute_contract(
            Addr::unchecked(addr1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::RemoveRecord {
                name: "isabel3".to_string(),
                bech32_address: signer_bech32_address.clone(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::RemovingPrimaryAddressNotAllowed {}
    );

    app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel2".to_string(),
            bech32_address: signer_bech32_address.clone(),
        },
        &[],
    )
    .unwrap();

    assert_eq!(
        primary_name(
            &app,
            signer_bech32_address.clone(),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        "isabel3".to_string()
    );

    assert_eq!(
        addresses(&app, "isabel2".to_string(), resolver_contract_addr.clone()).unwrap(),
        Vec::<Bech32Address>::new()
    );

    // when there it is the last record for this address, removing record with primary name is allowed
    app.execute_contract(
        Addr::unchecked(addr1),
        resolver_contract_addr,
        &ExecuteMsg::RemoveRecord {
            name: "isabel".to_string(),
            bech32_address: signer_bech32_address,
        },
        &[],
    )
    .unwrap();
}

#[test]
fn remove_by_admin() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr2 = cosmos_pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(
        &mut app,
        "isabel",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    mint_and_set_record(
        &mut app,
        "isabel2",
        signer_bech32_address.clone(),
        &signer1(),
        registrar,
        name_nft_contract,
        resolver_contract_addr.clone(),
    );

    let err = app
        .execute_contract(
            Addr::unchecked(addr2),
            resolver_contract_addr,
            &ExecuteMsg::RemoveRecord {
                name: "isabel".to_string(),
                bech32_address: signer_bech32_address,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
}
