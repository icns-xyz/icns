#![cfg(test)]

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, ICNSNameExecuteMsg, Metadata},
    tests::helpers::{TestEnv, TestEnvBuilder},
    QueryMsg,
};

use cosmwasm_std::{Addr, StdError, StdResult};
use cw721::OwnerOfResponse;
use cw721_base::MintMsg;
use cw_multi_test::{BasicApp, Executor};

#[test]
fn can_not_mint_until_minter_is_set() {
    let TestEnv {
        mut app,
        admins,
        contract_addr,
        registrar,
        ..
    } = TestEnvBuilder::default()
        .with_no_minter()
        .with_transferrable(false)
        .build();

    let owner = |app: &BasicApp, name: String| -> StdResult<_> {
        let OwnerOfResponse { owner, .. } = app.wrap().query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::OwnerOf {
                token_id: name,
                include_expired: None,
            },
        )?;

        Ok(owner)
    };

    let mint = |app: &mut BasicApp, sender: Addr, name: String, owner: String| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name,
                owner,
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
    };

    let random_person = Addr::unchecked("random_person");
    let name = "bob";

    // mint without setting minter should error
    let err = mint(
        &mut app,
        registrar.clone(),
        name.to_string(),
        random_person.to_string(),
    )
    .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::CW721Base(
            StdError::NotFound {
                kind: "cosmwasm_std::addresses::Addr".to_string()
            }
            .into()
        )
    );

    // non-admin can't set minter
    let err = app
        .execute_contract(
            random_person.clone(),
            contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: ICNSNameExecuteMsg::SetMinter {
                    minter_address: registrar.to_string(),
                },
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );

    // set minter to registrar
    app.execute_contract(
        admins[0].clone(),
        contract_addr.clone(),
        &ExecuteMsg::Extension {
            msg: ICNSNameExecuteMsg::SetMinter {
                minter_address: registrar.to_string(),
            },
        },
        &[],
    )
    .unwrap();

    // mint again
    mint(
        &mut app,
        registrar,
        name.to_string(),
        random_person.to_string(),
    )
    .unwrap();
    assert_eq!(
        owner(&app, name.to_string()).unwrap(),
        random_person.to_string()
    );
}

#[test]
fn can_not_name_with_dot() {
    let TestEnv {
        mut app,
        admins,
        contract_addr,
        registrar,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let mint = |app: &mut BasicApp, sender: Addr, name: String, owner: String| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name,
                owner,
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
    };

    let random_person = Addr::unchecked("random_person");
    let name = "bob.dylan";

    // set minter to registrar
    app.execute_contract(
        admins[0].clone(),
        contract_addr.clone(),
        &ExecuteMsg::Extension {
            msg: ICNSNameExecuteMsg::SetMinter {
                minter_address: registrar.to_string(),
            },
        },
        &[],
    )
    .unwrap();

    let err = mint(
        &mut app,
        registrar,
        name.to_string(),
        random_person.to_string(),
    )
    .unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::InvalidName {}
    );
}
#[test]
fn only_registrar_can_mint() {
    let TestEnv {
        mut app,
        admins,
        contract_addr,
        registrar,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let owner = |app: &BasicApp, name: String| -> StdResult<_> {
        let OwnerOfResponse { owner, .. } = app.wrap().query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::OwnerOf {
                token_id: name,
                include_expired: None,
            },
        )?;

        Ok(owner)
    };

    let mint = |app: &mut BasicApp, sender: Addr, name: String, owner: String| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name,
                owner,
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
    };

    let not_found_err = StdError::GenericErr {
        msg: "Querier contract error: cw721_base::state::TokenInfo<icns_name_nft::msg::Metadata> not found".to_string()
    };

    let random_person = Addr::unchecked("random_person");
    let name = "bob";

    // mint by random person should fail
    let err = mint(
        &mut app,
        random_person.clone(),
        name.to_string(),
        random_person.to_string(),
    )
    .unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );

    assert_eq!(owner(&app, name.to_string()).unwrap_err(), not_found_err);

    // mint by admin should fail
    let err = mint(
        &mut app,
        admins[0].clone(),
        name.to_string(),
        random_person.to_string(),
    )
    .unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );

    assert_eq!(owner(&app, name.to_string()).unwrap_err(), not_found_err);

    // mint by registarar should be allowed
    mint(
        &mut app,
        registrar,
        name.to_string(),
        random_person.to_string(),
    )
    .unwrap();
    assert_eq!(
        owner(&app, name.to_string()).unwrap(),
        random_person.to_string()
    );
}

#[test]
fn burning_is_not_allowed() {
    let TestEnv {
        mut app,
        admins,
        contract_addr,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let err = app
        .execute_contract(
            admins[0].clone(),
            contract_addr,
            &ExecuteMsg::Burn {
                token_id: "name".to_string(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );
}
