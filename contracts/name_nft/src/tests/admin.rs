#![cfg(test)]

use crate::{
    msg::{AdminResponse, ExecuteMsg, ICNSNameExecuteMsg},
    tests::helpers::{TestEnv, TestEnvBuilder},
    QueryMsg,
};

use cosmwasm_std::Addr;
use cw721_base::ContractError;
use cw_multi_test::{BasicApp, Executor};

#[test]
fn only_admin_can_set_new_admin() {
    let TestEnv {
        mut app,
        admin,
        contract_addr,
        registrar,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let get_admin = |app: &BasicApp| {
        let AdminResponse { admin } = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Admin {})
            .unwrap();

        admin
    };

    let set_admin = |app: &mut BasicApp, sender: Addr, admin: String| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetAdmin { admin }),
            &[],
        )
    };

    let new_admin = Addr::unchecked("new_admin");

    // set new admin by non admin should fail
    let err = set_admin(&mut app, new_admin.clone(), new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert_eq!(get_admin(&app), admin.to_string());

    // set admin by registrar should fail
    let err = set_admin(&mut app, registrar, new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert_eq!(get_admin(&app), admin.to_string());

    // set new admin by admin should succeed
    set_admin(&mut app, admin, new_admin.to_string()).unwrap();
    assert_eq!(get_admin(&app), new_admin.to_string());
}
