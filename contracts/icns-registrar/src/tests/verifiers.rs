#![cfg(test)]

use cosmwasm_std::{Addr, Binary, Decimal};
use cw_multi_test::{BasicApp, Executor};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, VerifierPubKeysResponse},
    ContractError,
};

use super::helpers::{fixtures::*, name_nft_contract, registrar_contract, ToBinary};

#[test]
fn only_admin_can_update_verifier() {
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

    // unauthorized if not admin
    let err = app
        .execute_contract(
            Addr::unchecked("random_guy"),
            registrar_contract_addr.clone(),
            &ExecuteMsg::UpdateVerifierPubkeys {
                add: vec![verifier1().to_binary()],
                remove: vec![verifier2().to_binary()],
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    let VerifierPubKeysResponse { verifier_pubkeys } = app
        .wrap()
        .query_wasm_smart(
            registrar_contract_addr.clone(),
            &QueryMsg::VerifierPubKeys {},
        )
        .unwrap();

    assert_eq!(verifier_pubkeys, vec![verifier2().to_binary()]);

    // authorized if admin
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        registrar_contract_addr.clone(),
        &ExecuteMsg::UpdateVerifierPubkeys {
            add: vec![verifier1().to_binary()],
            remove: vec![verifier2().to_binary()],
        },
        &[],
    )
    .unwrap();

    let VerifierPubKeysResponse { verifier_pubkeys } = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr, &QueryMsg::VerifierPubKeys {})
        .unwrap();

    assert_eq!(verifier_pubkeys, vec![verifier1().to_binary()]);
}

#[test]
fn update_verifier_must_keep_verifier_state_unique() {
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
                verifier_pubkeys: vec![],
                verification_threshold: Decimal::percent(50),
                fee: None,
            },
            &[],
            "registar",
            None,
        )
        .unwrap();

    // add duplicated pubkey should keeps state the same
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        registrar_contract_addr.clone(),
        &ExecuteMsg::UpdateVerifierPubkeys {
            add: vec![
                verifier1().to_binary(), // dup 1
                verifier1().to_binary(), // dup 1
                verifier2().to_binary(), // to remove 2
                verifier3().to_binary(), // -> kept
            ],
            remove: vec![verifier2().to_binary()], // remove 2
        },
        &[],
    )
    .unwrap();

    let VerifierPubKeysResponse { verifier_pubkeys } = app
        .wrap()
        .query_wasm_smart(
            registrar_contract_addr.clone(),
            &QueryMsg::VerifierPubKeys {},
        )
        .unwrap();

    assert_eq!(
        verifier_pubkeys,
        vec![verifier1().to_binary(), verifier3().to_binary()]
    );

    // add existing pubkey should not change anything
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        registrar_contract_addr.clone(),
        &ExecuteMsg::UpdateVerifierPubkeys {
            add: vec![verifier3().to_binary()],
            remove: vec![],
        },
        &[],
    )
    .unwrap();

    let VerifierPubKeysResponse { verifier_pubkeys } = app
        .wrap()
        .query_wasm_smart(registrar_contract_addr, &QueryMsg::VerifierPubKeys {})
        .unwrap();

    assert_eq!(
        verifier_pubkeys,
        vec![verifier1().to_binary(), verifier3().to_binary()]
    );
}

#[test]
#[ignore = "previous pubkey verification method depends on rust-crypto's crate which all depends on rand, need to find another way to verify pubkey"]
fn adding_invalid_pubkeys_is_not_allowed() {
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
                verifier_pubkeys: vec![],
                verification_threshold: Decimal::percent(50),
                fee: None,
            },
            &[],
            "registar",
            None,
        )
        .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked(admins[0].clone()),
            registrar_contract_addr,
            &ExecuteMsg::UpdateVerifierPubkeys {
                add: vec![Binary(vec![10, 2, 1])],
                remove: vec![verifier2().to_binary()],
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::InvalidPublicKeyFormat {}
    );
}
