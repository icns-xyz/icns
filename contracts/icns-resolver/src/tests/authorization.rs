#![cfg(test)]

use crate::{
    msg::{AddressesResponse, QueryMsg},
    tests::helpers::default_osmo_set_record_msg,
    ContractError,
};

use cosmwasm_std::{Addr, StdResult};
use cw721_base::MintMsg;
use cw_multi_test::{BasicApp, Executor};
use icns_name_nft::msg::{ExecuteMsg as NameExecuteMsg, Metadata};

use super::helpers::{instantiate_name_nft, instantiate_resolver_with_name_nft};

#[test]
fn only_admin_can_set_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar);

    // now instantiate resolver using name nft contract
    let resolver_contract_addr = instantiate_resolver_with_name_nft(&mut app, name_nft_contract);

    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let AddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Addresses { name },
        )?;

        Ok(addresses)
    };

    // try setting record with non admin, should fail
    let err = app
        .execute_contract(
            Addr::unchecked("non_admin".to_string()),
            resolver_contract_addr.clone(),
            &default_osmo_set_record_msg(),
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    // try setting record with admin, should be allowed
    app.execute_contract(
        Addr::unchecked(admin1),
        resolver_contract_addr.clone(),
        &default_osmo_set_record_msg(),
        &[],
    )
    .unwrap();

    // now check if record is set properly in store
    let addresses = addresses(&app, "alice".to_string()).unwrap();
    assert_eq!(
        addresses,
        vec![(
            "cosmos".to_string(),
            "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string()
        )]
    )
}

#[test]
fn only_owner_can_set_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    // mint name nft to tony
    let mint = app
        .execute_contract(
            Addr::unchecked(registrar),
            name_nft_contract,
            &NameExecuteMsg::Mint(MintMsg {
                token_id: "tony".to_string(),
                owner: "tony".to_string(),
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // try setting record with non owner, should fail
    let err = app
        .execute_contract(
            Addr::unchecked("non_owner".to_string()),
            resolver_contract_addr.clone(),
            &default_osmo_set_record_msg(),
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast::<ContractError>().unwrap(),
        ContractError::Unauthorized {}
    );

    // try setting record with owner, should be allowed
    app.execute_contract(
        Addr::unchecked(admin1),
        resolver_contract_addr,
        &default_osmo_set_record_msg(),
        &[],
    )
    .unwrap();
}
