#![cfg(test)]

use crate::{
    crypto::{create_adr36_message, pubkey_to_bech32_address},
    msg::{self, Adr36Info, ExecuteMsg, AddressesResponse},
    msg::{PrimaryNameResponse, QueryMsg},
    tests::helpers::{instantiate_name_nft, mint_and_set_record, instantiate_resolver_with_name_nft, signer2, primary_name},
    ContractError,
};

use cosmrs::crypto::secp256k1::SigningKey;
use cw721_base::{Extension, MintMsg};
use icns_name_nft::CW721BaseExecuteMsg;

use cosmwasm_std::{Addr, Empty, StdResult};

use cw_multi_test::{BasicApp, Executor};

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

    // if there is single name for an address, then that's primary
    mint_and_set_record(&mut app, "isabel", &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string()),
            resolver_contract_addr.clone(),
        )
        .unwrap(),
        "isabel".to_string()
    );

    // does not change primary if there are existing address(es)
    mint_and_set_record(&mut app, "isakaya", &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string()),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        "isabel".to_string()
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

    mint_and_set_record(&mut app, "isabel", &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isakaya", &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());
    mint_and_set_record(&mut app, "isann", &signer1(), registrar.clone(), name_nft_contract.clone(), resolver_contract_addr.clone());

    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string()),
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
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
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

    let err = app.execute_contract(
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
        &ContractError::Bech32AddressNotSet { name: "isann".to_string(), address: addr2.clone() }
    );

    // only owner can set primary
    app.execute_contract(
        Addr::unchecked(addr1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetPrimary {
            name: "isann".to_string(),
            bech32_address: addr1.clone(),
        },
        &[],
    )
    .unwrap();

    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string()),
            resolver_contract_addr.clone()
        )
        .unwrap(),
        "isann".to_string()
    );
}
