#![cfg(test)]

use cosmwasm_std::{Addr, Coin, Decimal};
use cw_multi_test::{BasicApp, Executor};

use crate::{
    msg::{ExecuteMsg, FeeResponse, InstantiateMsg, QueryMsg},
    ContractError,
};

use super::helpers::{fixtures::*, name_nft_contract, registrar_contract, ToBinary};

#[test]
fn only_admin_can_set_fee() {
    // setup contracts
    let mut app = BasicApp::default();
    let name_nft_code_id = app.store_code(name_nft_contract());
    let registrar_code_id = app.store_code(registrar_contract());
    let admins = vec!["admin1".to_string(), "admin2".to_string()];

    // setup contracts
    let name_nft_contract_addr = app
        .instantiate_contract(
            name_nft_code_id,
            Addr::unchecked(admins[0].clone()),
            &icns_name_nft::InstantiateMsg {
                admins: admins.clone(),
                transferrable: false,
            },
            &[],
            "name",
            None,
        )
        .unwrap();

    let registrar_contract_addr = app
        .instantiate_contract(
            registrar_code_id,
            Addr::unchecked(admins[0].clone()),
            &InstantiateMsg {
                name_nft_addr: name_nft_contract_addr.to_string(),
                verifier_pubkeys: vec![verifier2().to_binary()],
                verification_threshold: Decimal::percent(50),
            },
            &[],
            "registar",
            None,
        )
        .unwrap();

    let FeeResponse { fee } = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr.clone(), &QueryMsg::Fee {})
        .unwrap();

    assert_eq!(fee, Coin::new(0, "uosmo"));

    // unauthorized if not admin
    let err = app
        .execute_contract(
            Addr::unchecked("random_guy"),
            registrar_contract_addr.clone(),
            &ExecuteMsg::SetFee {
                fee: Coin::new(10000000, "uosmo"),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    // authorized if admin
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        registrar_contract_addr.clone(),
        &ExecuteMsg::SetFee {
            fee: Coin::new(999999999, "uosmo"),
        },
        &[],
    )
    .unwrap();

    let FeeResponse { fee } = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr, &QueryMsg::Fee {})
        .unwrap();

    assert_eq!(fee, Coin::new(999999999, "uosmo"));
}
