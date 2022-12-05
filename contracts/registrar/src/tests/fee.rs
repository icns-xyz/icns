#![cfg(test)]

use std::fmt::Debug;

use cosmwasm_std::{Addr, Coin, Decimal};
use cw_multi_test::{BasicApp, Executor};
use serde::de::DeserializeOwned;

use crate::{
    msg::{ExecuteMsg, FeeResponse, InstantiateMsg, QueryMsg},
    ContractError,
};

use super::helpers::{fixtures::*, name_nft_contract, registrar_contract, ToBinary};

fn test_only_admin<T>(execute_msg: ExecuteMsg, query_msg: QueryMsg, initial: T, updated: T)
where
    T: DeserializeOwned + PartialEq + Debug,
{
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

    let response: T = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr.clone(), &query_msg)
        .unwrap();

    assert_eq!(response, initial);

    // unauthorized if not admin
    let err = app
        .execute_contract(
            Addr::unchecked("random_guy"),
            registrar_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    let response: T = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr.clone(), &query_msg)
        .unwrap();

    assert_eq!(response, initial);

    // authorized if admin
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        registrar_contract_addr.clone(),
        &execute_msg,
        &[],
    )
    .unwrap();

    let response: T = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr, &query_msg)
        .unwrap();

    assert_eq!(response, updated);
}

#[test]
fn only_admin_can_set_fee() {
    test_only_admin(
        ExecuteMsg::SetFee {
            fee: Some(Coin::new(999999999, "uosmo")),
        },
        QueryMsg::Fee {},
        FeeResponse { fee: None },
        FeeResponse {
            fee: Some(Coin::new(999999999, "uosmo")),
        },
    );
}

// #[test]
// fn only_admin_can_set_fee() {
//     test_only_admin(
//         ExecuteMsg::SetFeeCollector {
//             fee_collector: "collector".to_string(),
//         },
//         QueryMsg::FeeCollector {},
//         FeeCollectorResponse {
//             fee_collector: "collector".to_string(),
//         },
//         FeeCollectorResponse {
//             fee_collector: "collector".to_string(),
//         },
//     );
// }
