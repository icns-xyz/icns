#![cfg(test)]

use crate::{
    msg::{QueryMsg, GetAddressesResponse},
    msg::{AdminResponse, ExecuteMsg},
    ContractError, contract::is_admin,
};

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

    // try setting record with non admin, should fail
    let err = app
    .execute_contract(
        Addr::unchecked("non_admin".to_string()), 
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                ],
            }, 
        &[],
    ).unwrap_err();
    assert_eq!(err.downcast::<ContractError>().unwrap(), ContractError::Unauthorized {});

    // try setting record with admin, should be allowed
    app
    .execute_contract(
        Addr::unchecked(admin1.clone()), 
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                ],
            }, 
        &[],
    ).unwrap();

    // now check if record is set properly in store
    let addresses = addresses(&app, "bob".to_string()).unwrap();
    assert_eq!(addresses, vec![("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string())]);
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
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                ],
            }, 
        &[],
    ).unwrap_err();
    assert_eq!(err.downcast::<ContractError>().unwrap(), ContractError::Unauthorized {});

    // try setting record with owner, should be allowed
    app
    .execute_contract(
        Addr::unchecked(admin1.clone()), 
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                ],
            }, 
        &[],
    ).unwrap();
}
