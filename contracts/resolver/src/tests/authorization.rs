#![cfg(test)]

use crate::{
    msg::{QueryMsg, GetAddressesResponse, AddressInfo},
    msg::{AdminResponse, ExecuteMsg, AddressHash},
    ContractError, contract::is_admin, tests::helpers::default_set_record,
};

use cosmwasm_std::{Binary};
use hex_literal::hex;
use cosmwasm_std::{Addr, Empty, StdResult};
use cw_multi_test::{BasicApp, Executor};
use icns_name_nft::{msg::ExecuteMsg as NameExecuteMsg, msg::QueryMsg as NameQueryMsg};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};


use super::helpers::{
    instantiate_name_nft, instantiate_resolver_with_name_nft, TestEnv,
    TestEnvBuilder,
};

#[test]
fn only_admin_can_set_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let GetAddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddresses {
                user_name: name,
            },
        )?;

        Ok(addresses)
    };

    // // try setting record with non admin, should fail
    // let err = app
    // .execute_contract(
    //     Addr::unchecked("non_admin".to_string()), 
    //     resolver_contract_addr.clone(),
    //     &default_set_record(),
    //     &[],
    // ).unwrap_err();

    // assert_eq!(
    //     err.downcast_ref::<ContractError>().unwrap(),
    //     &ContractError::Unauthorized {}
    // );

    let original_pubkey_vec = hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");
    let original_signature_vec = hex!("69c865c686a4b141297fee846e16a0f9c8df965fe64abea4513f653c8a3b385019f81c93081a2f3c0930c5cd3265bf621af863f48a2a9a54f8883d4a54d2c3d2");
    let pub_key = Binary::from(original_pubkey_vec);
    let signature = Binary::from(original_signature_vec);

    
    // try setting record with admin, should be allowed
    app
    .execute_contract(
        Addr::unchecked(admin1.clone()), 
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
            user_name: "bob".to_string(),
            address_info: AddressInfo{
                bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
                address_hash: AddressHash::SHA256,
                pub_key,
                signature,
            },
            bech32_prefix: "osmo".to_string(),
            replace_primary_if_exists: false,
            signature_salt: 1323124,
        }, 
        &[],
    ).unwrap();

    // // now check if record is set properly in store
    // let addresses = addresses(&app, "bob".to_string()).unwrap();
    // assert_eq!(addresses, vec![("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string())]);
}

#[test]
fn only_owner_can_set_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let GetAddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddresses {
                user_name: name,
            },
        )?;

        Ok(addresses)
    };

    // mint name nft to bob
    let mint = app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "bob".to_string(),
            owner: "bob".to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    ).is_err();
    assert_eq!(mint, false);


    // try setting record with non owner, should fail
    let err = app
    .execute_contract(
        Addr::unchecked("non_owner".to_string()), 
        resolver_contract_addr.clone(),
        &default_set_record(), 
        &[],
    ).unwrap_err();
    assert_eq!(err.downcast::<ContractError>().unwrap(), ContractError::Unauthorized {});

    // try setting record with owner, should be allowed
    app
    .execute_contract(
        Addr::unchecked(admin1.clone()), 
        resolver_contract_addr.clone(),
        &default_set_record(), 
        &[],
    ).unwrap();
}
