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
fn only_admin_can_add_new_admin() {
    let TestEnv {
        mut app,
        admins,
        contract_addr,
        registrar,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let get_admin = |app: &BasicApp| {
        let AdminResponse { admins } = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Admin {})
            .unwrap();

        admins
    };

    // change admin acc vec to string vec
    let mut admins_string = Vec::new();
    for admin in &admins {
        admins_string.push(admin.to_string());
    }

    let add_admin = |app: &mut BasicApp, sender: Addr, admin_address: String| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::AddAdmin{ admin_address }),
            &[],
        )
    };

    let new_admin = Addr::unchecked("new_admin");
    admins_string.push(new_admin.to_string());

    // set new admin by non admin should fail
    let err = add_admin(&mut app, new_admin.clone(), new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    assert_eq!(get_admin(&app), admins_string);

    // set admin by registrar should fail
    let err = add_admin(&mut app, registrar, new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert_eq!(get_admin(&app), admins_string);

    // set new admin by admin should succeed
    add_admin(&mut app, admins[0].clone(), new_admin.to_string()).unwrap();
    assert_eq!(get_admin(&app), admins_string);
}
