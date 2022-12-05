#![cfg(test)]

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::{BasicApp, Executor};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, VerificationThresholdResponse},
    ContractError,
};

use super::helpers::{fixtures::*, name_nft_contract, registrar_contract, ToBinary};

#[test]
fn only_admin_can_set_threshold() {
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
                fee: None,
            },
            &[],
            "registar",
            None,
        )
        .unwrap();
    let VerificationThresholdResponse {
        verification_threshold_percentage,
    } = app
        .wrap()
        .query_wasm_smart(
            registrar_contract_addr.clone(),
            &QueryMsg::VerificationThreshold {},
        )
        .unwrap();

    assert_eq!(verification_threshold_percentage, Decimal::percent(50));

    // unauthorized if not admin
    let err = app
        .execute_contract(
            Addr::unchecked("random_guy"),
            registrar_contract_addr.clone(),
            &ExecuteMsg::SetVerificationThreshold {
                threshold: Decimal::percent(10),
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
        &ExecuteMsg::SetVerificationThreshold {
            threshold: Decimal::percent(10),
        },
        &[],
    )
    .unwrap();

    let VerificationThresholdResponse {
        verification_threshold_percentage,
    } = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr, &QueryMsg::VerificationThreshold {})
        .unwrap();

    assert_eq!(verification_threshold_percentage, Decimal::percent(10));
}
