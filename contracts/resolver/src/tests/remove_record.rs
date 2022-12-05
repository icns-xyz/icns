#![cfg(test)]

use crate::{
    crypto::{create_adr36_message, pubkey_to_bech32_address},
    msg::{self, Adr36Info, ExecuteMsg, AddressesResponse},
    msg::{PrimaryNameResponse, QueryMsg},
    tests::helpers::{instantiate_name_nft, 
        instantiate_resolver_with_name_nft, signer2,
        mint_and_set_record, primary_name, addresses},
    ContractError,
};

use cosmrs::crypto::secp256k1::SigningKey;
use cw721_base::{Extension, MintMsg};
use icns_name_nft::CW721BaseExecuteMsg;

use cosmwasm_std::{Addr, Empty, StdResult};

use cw_multi_test::{BasicApp, Executor};

use super::helpers::{signer1, ToBinary};


#[test]
fn remove_and_replace_with_single_name_for_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(&mut app, "isabel", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());

    app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel".to_string(),
            bech32_address: signer_bech32_address.clone().to_string(),
            replace_primary_name: None,
        },
        &[],
    )
    .unwrap();

    // now check primary name and addresses
    assert_eq!(
        primary_name(
            &app,
            addr1.clone(),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        "".to_string()
    );

    // should have nothing as addresses
    assert_eq!(
        addresses(
            &app,
            "isabel".to_string(),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        Vec::<(String, String)>::new()
    );
}

#[test]
fn remove_and_replace_with_two_names_for_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(&mut app, "isabel", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isabel2", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());

    app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel".to_string(),
            bech32_address: signer_bech32_address.clone().to_string(),
            replace_primary_name: Some("isabel2".to_string())
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
        "isabel2".to_string()
    );

    assert_eq!(
        addresses(
            &app,
            "isabel".to_string(),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        Vec::<(String, String)>::new()
    );
    assert_eq!(
        addresses(
            &app,
            "isabel2".to_string(),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        vec![("cosmos".to_string(), signer_bech32_address.clone())]
    );
}


#[test]
fn remove_non_primary_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(&mut app, "isabel", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isabel2", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isabel3", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    
    app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel2".to_string(),
            bech32_address: signer_bech32_address.clone().to_string(),
            replace_primary_name: Some("isabel3".to_string())
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
        "isabel".to_string()
    );

    assert_eq!(
        addresses(
            &app,
            "isabel2".to_string(),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        Vec::<(String, String)>::new()
    );
}

#[test]
fn remove_with_non_existent_name_as_replacement() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(&mut app, "isabel", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isabel2", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());

    
    let err = app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel".to_string(),
            bech32_address: signer_bech32_address.clone().to_string(),
            replace_primary_name: Some("non existent name".to_string())
        },
        &[],
    )
    .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::ReplacePrimaryAddressNotSet { name: "isabel".to_string(), address: signer_bech32_address.clone() }
    );
}

#[test]
fn remove_primary_with_no_replacement_name() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(&mut app, "isabel", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isabel2", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());

    
    let err = app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel".to_string(),
            bech32_address: signer_bech32_address.clone(),
            replace_primary_name: None
        },
        &[],
    )
    .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::ReplacePrimaryAddressNotSet { name: "isabel".to_string(), address: signer_bech32_address.clone() }
    );
}

#[test]
fn remove_as_non_owner() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let addr2 = pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string());
    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // make sure primary name is correctly set
    mint_and_set_record(&mut app, "isabel", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isabel2", signer_bech32_address.clone(), &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());

    
    let err = app.execute_contract(
        Addr::unchecked(addr2.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::RemoveRecord {
            name: "isabel".to_string(),
            bech32_address: signer_bech32_address.clone(),
            replace_primary_name: None
        },
        &[],
    )
    .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {  }
    );
}