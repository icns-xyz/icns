#![cfg(test)]

use crate::{
    msg::{NameByTwitterIdResponse, QueryMsg},
    tests::helpers::{fixtures::*, ToBinary, default_contracts_setup},
};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmwasm_std::{Addr, Binary, Coin, Decimal, StdError, StdResult};
use cw721::{NftInfoResponse, OwnerOfResponse};
use cw_multi_test::{AppBuilder, BasicApp, Executor};
use icns_name_nft::msg::{ICNSNameExecuteMsg, Metadata};

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
    let (name_nft_contract_addr, registrar_contract_addr) = default_contracts_setup(
        &mut app,
        name_nft_code_id,
        registrar_code_id,
        admins.clone(),
        None
    );

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

    let name_by_twitter_id = |app: &BasicApp, twitter_id: String| -> StdResult<_> {
        let NameByTwitterIdResponse { name } = app.wrap().query_wasm_smart(
            registrar_contract_addr.clone(),
            &QueryMsg::NameByTwitterId { twitter_id },
        )?;

        Ok(name)
    };

    let bob = Addr::unchecked("bobaddr");
    let bob_name = "bob";
    let multitest_chain_id = "cosmos-testnet-14002";
    let unique_twitter_id = "1234567890";

    // "bob" shouldn't be owned by anyone at first
    assert_eq!(
        owner(&app, bob_name.to_string()).unwrap_err(),
        StdError::GenericErr {
            msg: "Querier contract error: cw721_base::state::TokenInfo<icns_name_nft::msg::Metadata> not found".to_string()
        }
    );

    // unique_twitter_id has not been used yet
    assert_eq!(
        name_by_twitter_id(&app, unique_twitter_id.to_string()).unwrap_err(),
        StdError::GenericErr {
            msg: "Querier contract error: alloc::string::String not found".to_string()
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
    assert_eq!(
        name_by_twitter_id(&app, unique_twitter_id.to_string()).unwrap(),
        bob_name
    );

    // execute claim with passing but same name -> should error
    let new_twitter_id = "11111";
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{new_twitter_id}"}}"#,
    );

    let err = app
        .execute_contract(
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
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<icns_name_nft::error::ContractError>()
            .unwrap(),
        &(cw721_base::ContractError::Claimed {}.into())
    );

    // execute claim with passing(different name) but with same unique twitter id -> should error
    let new_name = "new_name".to_string();
    let verifying_msg = format!(
        r#"{{"name":"{new_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob,
            registrar_contract_addr,
            &ExecuteMsg::Claim {
                name: new_name,
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), verifier2()]),
                referral: None,
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::DuplicatedTwitterId {
            msg: format!("unique twitter id `{}` is already used", unique_twitter_id)
        }
    );
}

#[test]
fn admin_bypass_verifications() {
     // setup contracts
     let mut app = BasicApp::default();
     let name_nft_code_id = app.store_code(name_nft_contract());
     let registrar_code_id = app.store_code(registrar_contract());
     let admins = vec!["admin1".to_string(), "admin2".to_string()];
 
    // setup name nft contract
    let (_name_nft_contract_addr, registrar_contract_addr) = default_contracts_setup(
        &mut app,
        name_nft_code_id,
        registrar_code_id,
        admins.clone(),
        None,
    );

    // execute claim with passing verification
    let bob = Addr::unchecked("bobaddr");
    let bob_name = "bob";
    let multitest_chain_id = "cosmos-testnet-14002";
    let unique_twitter_id = "1234567890";
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    // try claiming with non admin with no verifications, this should error
    app.execute_contract(
        bob.clone(),
        registrar_contract_addr.clone(),
        &ExecuteMsg::Claim {
            name: bob_name.to_string(),
            verifying_msg: verifying_msg.clone(),
            verifications: Vec::new(),
            referral: None,
        },
        &[],
    )
    .unwrap_err();

    // now try claiming with admin with no verifications, this should work
    app.execute_contract(
        Addr::unchecked(admins[0].clone()),
        registrar_contract_addr.clone(),
        &ExecuteMsg::Claim {
            name: bob_name.to_string(),
            verifying_msg: verifying_msg.clone(),
            verifications: Vec::new(),
            referral: None,
        },
        &[],
    )
    .unwrap();
}

#[test]
fn claim_name_with_fee() {
    let bob = Addr::unchecked("bobaddr");
    // setup contracts
    let mut app = AppBuilder::default().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &bob,
                vec![
                    Coin::new(100_000_000_000, "uosmo"),
                    Coin::new(100_000_000_000, "uion"),
                ],
            )
            .unwrap();
    });
    let name_nft_code_id = app.store_code(name_nft_contract());
    let registrar_code_id = app.store_code(registrar_contract());
    let admins = vec!["admin1".to_string(), "admin2".to_string()];
    let fee = Coin::new(1_000_000_000, "uosmo");

    // setup name nft contract
    let (name_nft_contract_addr, registrar_contract_addr) = default_contracts_setup(
        &mut app,
        name_nft_code_id,
        registrar_code_id,
        admins.clone(),
        Some(fee.clone())
    );

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

    let bob_name = "bob";
    let multitest_chain_id = "cosmos-testnet-14002";
    let unique_twitter_id = "1234567890";

    // "bob" shouldn't be owned by anyone at first
    assert_eq!(
        owner(&app, bob_name.to_string()).unwrap_err(),
        StdError::GenericErr {
            msg: "Querier contract error: cw721_base::state::TokenInfo<icns_name_nft::msg::Metadata> not found".to_string()
        }
    );

    // execute claim invalid fee
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    // no funds
    let err = app
        .execute_contract(
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
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::InvalidFee {
            fee_required: fee.clone()
        }
    );

    // wrong denom
    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
                referral: None,
            },
            &[Coin::new(100, "uion")],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::InvalidFee {
            fee_required: fee.clone()
        }
    );

    // over paid
    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
                referral: None,
            },
            &[Coin::new(100_000_000_000, "uosmo")],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::InvalidFee {
            fee_required: fee.clone()
        }
    );

    // under paid
    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
                referral: None,
            },
            &[Coin::new(1, "uosmo")],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::InvalidFee {
            fee_required: fee.clone()
        }
    );

    // exact fee
    app.execute_contract(
        bob.clone(),
        registrar_contract_addr,
        &ExecuteMsg::Claim {
            name: bob_name.to_string(),
            verifying_msg: verifying_msg.clone(),
            verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
            referral: None,
        },
        &[fee],
    )
    .unwrap();

    assert_eq!(owner(&app, bob_name.to_string()).unwrap(), bob);
}

#[test]
fn claim_name_with_referral() {
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

    let metadata = |app: &BasicApp, name: String| -> StdResult<_> {
        let NftInfoResponse { extension, .. }: NftInfoResponse<Metadata> =
            app.wrap().query_wasm_smart(
                name_nft_contract_addr.clone(),
                &icns_name_nft::QueryMsg::NftInfo { token_id: name },
            )?;

        Ok(extension)
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
                fee: None,
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
        &icns_name_nft::msg::ExecuteMsg::Extension {
            msg: ICNSNameExecuteMsg::SetMinter {
                minter_address: registrar_contract_addr.to_string(),
            },
        },
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
            msg: "Querier contract error: cw721_base::state::TokenInfo<icns_name_nft::msg::Metadata> not found".to_string()
        }
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
            referral: Some("referral".to_string()),
        },
        &[],
    )
    .unwrap();

    assert_eq!(owner(&app, bob_name.to_string()).unwrap(), bob);
    assert_eq!(
        metadata(&app, bob_name.to_string()).unwrap(),
        Metadata {
            referral: Some("referral".to_string())
        }
    );
}

#[test]
fn try_claiming_with_unpassed_threshold() {
    // setup contracts
    let mut app = BasicApp::default();
    let name_nft_code_id = app.store_code(name_nft_contract());
    let registrar_code_id = app.store_code(registrar_contract());
    let admins = vec!["admin1".to_string(), "admin2".to_string()];

    // setup name nft contract
    let (_name_nft_contract_addr, registrar_contract_addr) = default_contracts_setup(
        &mut app,
        name_nft_code_id,
        registrar_code_id,
        admins.clone(),
        None
    );

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
    let bob = Addr::unchecked("bobaddr");
    let bob_name = "bob";
    let multitest_chain_id = "cosmos-testnet-14002";
    let unique_twitter_id = "1234567890";

    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    let err = app.execute_contract(
        bob.clone(),
        registrar_contract_addr.clone(),
        &ExecuteMsg::Claim {
            name: bob_name.to_string(),
            verifying_msg: verifying_msg.clone(),
            verifications: verify_all(&verifying_msg, vec![verifier3()]),
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
}