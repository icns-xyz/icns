#![cfg(test)]

use crate::{
    msg::{AdminResponse, ExecuteMsg, ICNSNameExecuteMsg},
    tests::helpers::{Env, EnvBuilder},
    QueryMsg,
};

use cosmwasm_std::Addr;
use cw721_base::ContractError;
use cw_multi_test::Executor;

#[test]
fn only_admin_can_set_new_admin() {
    let Env {
        mut app,
        admin,
        contract_addr,
        ..
    } = EnvBuilder::default().with_transferable(false).build();

    let new_admin = Addr::unchecked("new_admin");

    // set new admin by non admin should fail
    let err = app
        .execute_contract(
            new_admin.clone(),
            contract_addr.clone(),
            &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetAdmin {
                admin: new_admin.to_string(),
            }),
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    // admin should not be changed
    let res: AdminResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::Admin {})
        .unwrap();

    assert_eq!(res.admin, admin.to_string());

    // set new admin by admin should succeed
    app.execute_contract(
        admin,
        contract_addr.clone(),
        &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetAdmin {
            admin: new_admin.to_string(),
        }),
        &[],
    )
    .unwrap();

    let res: AdminResponse = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryMsg::Admin {})
        .unwrap();

    assert_eq!(res.admin, new_admin.to_string());
}
