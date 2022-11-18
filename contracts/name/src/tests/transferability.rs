#![cfg(test)]

use crate::{
    msg::{ExecuteMsg, ICNSNameExecuteMsg, TransferrableResponse},
    tests::helpers::{Env, EnvBuilder},
    QueryMsg,
};

use cosmwasm_std::{Addr, Empty};
use cw721::OwnerOfResponse;
use cw721_base::{ContractError, ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};
use cw_multi_test::{BasicApp, Executor};

#[test]
fn transferable_false_should_not_allow_transfer() {
    let Env {
        mut app,
        contract_addr,
        registry,
        ..
    } = EnvBuilder::default().with_transferable(false).build();

    let name_owner = Addr::unchecked("name_owner");
    let recipient = Addr::unchecked("recipient");
    let name = "alice";

    // registry mint test icns_name
    app.execute_contract(
        registry,
        contract_addr.clone(),
        &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: name.to_string(),
            owner: name_owner.to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    )
    .unwrap();

    // transfer must be unauthorized
    let err = app
        .execute_contract(
            name_owner,
            contract_addr,
            &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::TransferNft {
                recipient: recipient.to_string(),
                token_id: name.to_string(),
            }),
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
}

#[test]
fn transferable_true_should_allow_transfer() {
    let Env {
        mut app,
        contract_addr,
        registry,
        ..
    } = EnvBuilder::default().with_transferable(true).build();

    let name_owner = Addr::unchecked("name_owner");
    let recipient = Addr::unchecked("recipient");
    let name = "alice";

    // mint test name
    app.execute_contract(
        registry,
        contract_addr.clone(),
        &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: name.to_string(),
            owner: name_owner.to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    )
    .unwrap();

    // transfer to recipient should succeed
    app.execute_contract(
        name_owner,
        contract_addr.clone(),
        &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::TransferNft {
            recipient: recipient.to_string(),
            token_id: name.to_string(),
        }),
        &[],
    )
    .unwrap();

    // name is now owned by recipient
    let res: OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::OwnerOf {
                token_id: name.to_string(),
                include_expired: None,
            },
        )
        .unwrap();

    assert_eq!(res.owner, recipient.to_string());
}

#[test]
fn only_admin_can_set_transferable() {
    let Env {
        mut app,
        contract_addr,
        registry,
        admin,
        ..
    } = EnvBuilder::default().with_transferable(false).build();

    let transferrable = |app: &BasicApp| {
        let TransferrableResponse { transferrable } = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Transferrable {})
            .unwrap();
        transferrable
    };

    let set_transferrable = |app: &mut BasicApp, sender: Addr, transferrable: bool| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetTransferrable { transferrable }),
            &[],
        )
    };

    assert!(!transferrable(&app));

    // transferrable can't be set by random person
    let err = set_transferrable(&mut app, Addr::unchecked("random_person"), true).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert!(!transferrable(&app));

    // transferrable can't be set by registry
    let err = set_transferrable(&mut app, registry, true).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert!(!transferrable(&app));

    // transferrable can only be set by admin
    set_transferrable(&mut app, admin.clone(), true).unwrap();
    assert!(transferrable(&app));

    set_transferrable(&mut app, admin, false).unwrap();
    assert!(!transferrable(&app));
}
