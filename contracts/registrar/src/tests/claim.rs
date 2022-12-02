#![cfg(test)]

use crate::tests::helpers::{fixtures::*, ToBinary};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmwasm_std::{Addr, Binary, Decimal, StdError, StdResult};
use cw721::OwnerOfResponse;
use cw_multi_test::{BasicApp, Executor};
use icns_name_nft::msg::ICNSNameExecuteMsg;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, Verification},
    tests::helpers::{name_nft_contract, registrar_contract},
    ContractError,
};

#[test]
fn claim_name() {
    // setup contracts
    let mut app = BasicApp::default();
    let name_nft_code_id = app.store_code(name_nft_contract());
    let registrar_code_id = app.store_code(registrar_contract());
    let admins = vec!["admin1".to_string(), "admin2".to_string()];

    // setup name nft contract
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

    let owner = |app: &BasicApp, name: String| -> StdResult<_> {
        let OwnerOfResponse { owner, .. } = app.wrap().query_wasm_smart(
            name_nft_contract_addr.clone(),
            &icns_name_nft::QueryMsg::OwnerOf {
                token_id: name,
                include_expired: None,
            },
        )?;

        Ok(owner)
    };

    // set up verifiers
    let verify_all = |verifying_msg: &str, verifiers: Vec<SigningKey>| -> Vec<Verification> {
        verifiers
            .iter()
            .map(|verifier| Verification {
                public_key: verifier.to_binary(),
                signature: verifier.sign(verifying_msg.as_bytes()).unwrap().to_binary(),
            })
            .collect()
    };

    // set up reigistrar contract
    let registrar_contract_addr = app
        .instantiate_contract(
            registrar_code_id,
            Addr::unchecked(admins[0].clone()),
            &InstantiateMsg {
                name_nft_addr: name_nft_contract_addr.to_string(),
                verifier_pubkeys: vec![verifier1(), verifier2(), verifier3(), verifier4()]
                    .iter()
                    .map(|v| v.to_binary())
                    .collect(),
                verification_threshold: Decimal::percent(50),
            },
            &[],
            "registar",
            None,
        )
        .unwrap();

    // set registrar as name nft minter
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        name_nft_contract_addr.clone(),
        &icns_name_nft::msg::ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetMinter {
            minter_address: registrar_contract_addr.to_string(),
        }),
        &[],
    )
    .unwrap();

    let bob = Addr::unchecked("bobaddr");
    let bob_name = "bob";
    let multitest_chain_id = "cosmos-testnet-14002";
    let unique_twitter_id = "1234567890";

    // "bob" shouldn't be owned by anyone at first
    assert_eq!(
        owner(&app, bob_name.to_string()).unwrap_err(),
        StdError::GenericErr {
            msg: "Querier contract error: cw721_base::state::TokenInfo<core::option::Option<cosmwasm_std::results::empty::Empty>> not found".to_string()
        }
    );

    // execute claim with wrong verifying msg form
    let verifying_msg = format!(r#"{{"name": "{bob_name}"}}"#,);

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), verifier3()]),
                referral: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Std(StdError::ParseErr {
            target_type: "icns_registrar::msg::VerifyingMsg".to_string(),
            msg: "missing field `claimer`".to_string()
        })
    );

    // execute claim with wrong verifying msg info
    let verifying_msg = format!(
        r#"{{"name":"alice","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), verifier3()]),
                referral: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::InvalidVerifyingMessage {
            msg: "name mismatched: expected `bob` but got `alice`".to_string(),
        }
    );

    // execute claim with verification from non-verifier
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), non_verifier()]),
                referral: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::NotAVerifierPublicKey {
            public_key: Binary(non_verifier().public_key().to_bytes())
        }
    );

    // execute claim with non passing verification below threshold
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1()]),
                referral: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::ValidVerificationIsBelowThreshold {
            expected_over: Decimal::percent(50),
            actual: Decimal::percent(25)
        }
    );

    // execute claim with . in name
    let bob_name_with_dot = "bob.dylan";
    let verifying_msg = format!(
        r#"{{"name":"{bob_name_with_dot}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name_with_dot.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), verifier2()]),
                referral: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<icns_name_nft::error::ContractError>()
            .unwrap(),
        &icns_name_nft::error::ContractError::InvalidName {}
    );

    // execute claim with passing verification
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    app.execute_contract(
        bob.clone(),
        registrar_contract_addr.clone(),
        &ExecuteMsg::Claim {
            name: bob_name.to_string(),
            verifying_msg: verifying_msg.clone(),
            verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
            referral: None,
        },
        &[],
    )
    .unwrap();

    assert_eq!(owner(&app, bob_name.to_string()).unwrap(), bob);

    // execute claim with passing but same name -> should error
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob,
            registrar_contract_addr,
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
                referral: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<icns_name_nft::error::ContractError>()
            .unwrap(),
        &(cw721_base::ContractError::Claimed {}.into())
    );
}
