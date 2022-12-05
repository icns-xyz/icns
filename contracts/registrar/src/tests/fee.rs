#![cfg(test)]

use super::helpers::test_only_admin;
use crate::{
    msg::{ExecuteMsg, FeeResponse, InstantiateMsg, QueryMsg, Verification},
    tests::helpers::{
        fixtures::{verifier3, verifier4},
        name_nft_contract, registrar_contract, ToBinary,
    },
    ContractError,
};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmwasm_std::{Addr, AllBalanceResponse, BankQuery, Coin, Decimal, QueryRequest, StdResult};
use cw721::OwnerOfResponse;
use cw_multi_test::{AppBuilder, BasicApp, Executor};
use icns_name_nft::msg::ICNSNameExecuteMsg;

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

#[test]
fn only_admin_can_withdraw_collected_fees() {
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

    let fee = Coin::new(1_000_000_000, "uosmo");

    // set up reigistrar contract
    let registrar_contract_addr = app
        .instantiate_contract(
            registrar_code_id,
            Addr::unchecked(admins[0].clone()),
            &InstantiateMsg {
                name_nft_addr: name_nft_contract_addr.to_string(),
                verifier_pubkeys: vec![verifier3(), verifier3(), verifier3(), verifier4()]
                    .iter()
                    .map(|v| v.to_binary())
                    .collect(),
                verification_threshold: Decimal::percent(50),
                fee: Some(fee.clone()),
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

    let bob_name = "bob";
    let multitest_chain_id = "cosmos-testnet-14002";
    let unique_twitter_id = "1234567890";

    // execute claim invalid fee
    let verifying_msg = format!(
        r#"{{"name":"{bob_name}","claimer":"{bob}","contract_address":"{registrar_contract_addr}","chain_id":"{multitest_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
    );

    // at first, contract should have no funds
    assert_eq!(
        app.wrap()
            .query::<AllBalanceResponse>(&QueryRequest::Bank(BankQuery::AllBalances {
                address: registrar_contract_addr.to_string(),
            }))
            .unwrap()
            .amount,
        vec![]
    );

    // exact fee
    app.execute_contract(
        bob.clone(),
        registrar_contract_addr.clone(),
        &ExecuteMsg::Claim {
            name: bob_name.to_string(),
            verifying_msg: verifying_msg.clone(),
            verifications: verify_all(&verifying_msg, vec![verifier4(), verifier3()]),
            referral: None,
        },
        &[fee.clone()],
    )
    .unwrap();

    assert_eq!(owner(&app, bob_name.to_string()).unwrap(), bob);

    // contract should now have funds = fee
    assert_eq!(
        app.wrap()
            .query::<AllBalanceResponse>(&QueryRequest::Bank(BankQuery::AllBalances {
                address: registrar_contract_addr.to_string(),
            }))
            .unwrap()
            .amount,
        vec![fee.clone()]
    );

    // bob should not be able to withdraw
    let err = app
        .execute_contract(
            bob,
            registrar_contract_addr.clone(),
            &ExecuteMsg::WithdrawFunds {
                amount: vec![fee.clone()],
                to_address: admins[0].clone(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    // admin should be able to withdraw
    app.execute_contract(
        Addr::unchecked(&admins[0]),
        registrar_contract_addr,
        &ExecuteMsg::WithdrawFunds {
            amount: vec![fee.clone()],
            to_address: admins[0].clone(),
        },
        &[],
    )
    .unwrap();

    // destined addr should get funds
    assert_eq!(
        app.wrap()
            .query::<AllBalanceResponse>(&QueryRequest::Bank(BankQuery::AllBalances {
                address: admins[0].to_string()
            }))
            .unwrap()
            .amount,
        vec![fee]
    );
}
