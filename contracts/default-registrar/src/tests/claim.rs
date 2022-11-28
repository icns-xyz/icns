#![cfg(test)]

use cosmrs::{bip32, crypto::secp256k1::SigningKey, tendermint::signature::Secp256k1Signature};
use cosmwasm_std::{Addr, Binary, Decimal, Empty, StdError, StdResult};
use cw721::OwnerOfResponse;
use cw_multi_test::{BasicApp, Contract, ContractWrapper, Executor};
use icns_name_nft::msg::ICNSNameExecuteMsg;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, Verification},
    ContractError,
};

pub fn name_nft_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        icns_name_nft::entry::execute,
        icns_name_nft::entry::instantiate,
        icns_name_nft::entry::query,
    );
    Box::new(contract)
}

pub fn registrar_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

fn from_mnemonic(phrase: &str, derivation_path: &str) -> SigningKey {
    let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse().unwrap()).unwrap();
    xprv.into()
}

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
    let derivation_path = "m/44'/118'/0'/0/0";
    let verifier1 = || {
        from_mnemonic("notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius", derivation_path)
    };
    let verifier2 = || {
        from_mnemonic("quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty", derivation_path)
    };
    let verifier3 = || {
        from_mnemonic("symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb", derivation_path)
    };
    let verifier4 = || {
        from_mnemonic("bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty", derivation_path)
    };
    let non_verifier = || {
        from_mnemonic("prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder", derivation_path)
    };

    let base64_pubkey =
        |verifier: &SigningKey| Binary(verifier.public_key().to_bytes()).to_base64();

    let base64_signature =
        |signature: &Secp256k1Signature| Binary(signature.to_der().as_bytes().to_vec()).to_base64();

    let verify_all = |verifying_msg: &str, verifiers: Vec<SigningKey>| -> Vec<Verification> {
        verifiers
            .iter()
            .map(|verifier| Verification {
                public_key: base64_pubkey(verifier),
                signature: base64_signature(&verifier.sign(verifying_msg.as_bytes()).unwrap()),
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
                    .map(base64_pubkey)
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
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Std(StdError::ParseErr {
            target_type: "default_registrar::msg::VerifyingMsg".to_string(),
            msg: "missing field `claimer`".to_string()
        })
    );

    // execute claim with wrong verifying msg info
    let verifying_msg = format!(
        r#"{{"name":"alice","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), verifier3()]),
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
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), non_verifier()]),
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
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1()]),
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
        r#"{{"name":"{bob_name_with_dot}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob.clone(),
            registrar_contract_addr.clone(),
            &ExecuteMsg::Claim {
                name: bob_name_with_dot.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier1(), verifier2()]),
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
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}"}}"#,
    );

    app.execute_contract(
        bob.clone(),
        registrar_contract_addr.clone(),
        &ExecuteMsg::Claim {
            name: bob_name.to_string(),
            verifying_msg: verifying_msg.clone(),
            verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
        },
        &[],
    )
    .unwrap();

    assert_eq!(owner(&app, bob_name.to_string()).unwrap(), bob);

    // execute claim with passing but same name -> should error
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}"}}"#,
    );

    let err = app
        .execute_contract(
            bob,
            registrar_contract_addr,
            &ExecuteMsg::Claim {
                name: bob_name.to_string(),
                verifying_msg: verifying_msg.clone(),
                verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
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
