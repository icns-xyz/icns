#![cfg(test)]

use crate::{
    msg::ExecuteMsg,
    tests::helpers::{TestEnv, TestEnvBuilder},
    QueryMsg,
};

use cosmwasm_std::{Addr, Empty, StdError, StdResult};
use cw721::OwnerOfResponse;
use cw721_base::{ContractError, ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};
use cw_multi_test::{BasicApp, Executor};

#[test]
fn only_registry_can_set_new_admin() {
    let TestEnv {
        mut app,
        admin,
        contract_addr,
        registry,
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
            &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: name,
                owner,
                token_uri: None,
                extension: None,
            })),
            &[],
        )
    };

    let not_found_err = StdError::GenericErr {
        msg: "Querier contract error: cw721_base::state::TokenInfo<core::option::Option<cosmwasm_std::results::empty::Empty>> not found".to_string()
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
        &ContractError::Unauthorized {}
    );

    assert_eq!(owner(&app, name.to_string()).unwrap_err(), not_found_err);

    // mint by admin should fail
    let err = mint(&mut app, admin, name.to_string(), random_person.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    assert_eq!(owner(&app, name.to_string()).unwrap_err(), not_found_err);

    // mint by registry should be allowed
    mint(
        &mut app,
        registry,
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
        admin,
        contract_addr,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let err = app
        .execute_contract(
            admin,
            contract_addr,
            &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Burn {
                token_id: "name".to_string(),
            }),
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
}
