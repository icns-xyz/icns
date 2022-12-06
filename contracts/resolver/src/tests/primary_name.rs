#![cfg(test)]

use crate::{
    crypto::pubkey_to_bech32_address,
    msg::ExecuteMsg,
    tests::helpers::{
        instantiate_name_nft, instantiate_resolver_with_name_nft, mint_and_set_record,
        primary_name, signer2,
    },
    ContractError,
};

use cw721_base::{Extension, MintMsg};
use icns_name_nft::CW721BaseExecuteMsg;

use cosmwasm_std::{Addr, Empty};

use cw_multi_test::Executor;

use super::helpers::{signer1, ToBinary};

#[test]
fn set_primary_name_on_set_first_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    // if there is single name for an address, then that's primary
    mint_and_set_record(
        &mut app,
        "alice",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string()),
            resolver_contract_addr.clone(),
        )
        .unwrap(),
        "alice".to_string()
    );

    // does not change primary if there are existing address(es)
    mint_and_set_record(
        &mut app,
        "isakaya",
        signer_bech32_address,
        &signer1(),
        registrar,
        name_nft_contract,
        resolver_contract_addr.clone(),
    );
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string()),
            resolver_contract_addr
        )
        .unwrap(),
        "alice".to_string()
    );
}

#[test]
fn set_primary() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
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
        "isakaya",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );
    mint_and_set_record(
        &mut app,
        "isann",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string()),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        "isabel".to_string()
    );

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let addr2 = pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string());

    // non-owner can't set primary
    let err = app
        .execute_contract(
            Addr::unchecked(addr2.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetPrimary {
                name: "isann".to_string(),
                bech32_address: addr1.clone(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    // set primary with name that they do not own is not allowed
    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract,
        &CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "others_name".to_string(),
            owner: "someone_else".to_string(),
            token_uri: None,
            extension: None,
        }),
        &[],
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked(addr1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetPrimary {
                name: "others_name".to_string(),
                bech32_address: addr1.clone(),
            },
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    let err = app
        .execute_contract(
            Addr::unchecked(addr1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetPrimary {
                name: "isann".to_string(),
                bech32_address: addr2.clone(),
            },
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Bech32AddressNotSet {
            name: "isann".to_string(),
            address: addr2
        }
    );

    // only owner can set primary
    app.execute_contract(
        Addr::unchecked(addr1),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetPrimary {
            name: "isann".to_string(),
            bech32_address: signer_bech32_address,
        },
        &[],
    )
    .unwrap();

    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string()),
            resolver_contract_addr
        )
        .unwrap(),
        "isann".to_string()
    );
}
