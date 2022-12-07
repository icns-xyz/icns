#![cfg(test)]

use crate::{
    error::ContractError,
    msg::{AdminResponse, ExecuteMsg, ICNSNameExecuteMsg},
    tests::helpers::{TestEnv, TestEnvBuilder},
    QueryMsg,
};

use cosmwasm_std::Addr;
use cw_multi_test::{BasicApp, Executor};

#[test]
fn only_admin_can_add_remove_new_admin() {
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
            &ExecuteMsg::Extension {
                msg: ICNSNameExecuteMsg::AddAdmin { admin_address },
            },
            &[],
        )
    };

    let remove_admin = |app: &mut BasicApp, sender: Addr, admin_address: String| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::Extension {
                msg: ICNSNameExecuteMsg::RemoveAdmin { admin_address },
            },
            &[],
        )
    };
    let new_admin = Addr::unchecked("new_admin");

    // set new admin by non admin should fail
    let err = add_admin(&mut app, new_admin.clone(), new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );

    assert_eq!(get_admin(&app), admins_string);

    // set admin by registrar should fail
    let err = add_admin(&mut app, registrar.clone(), new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );
    assert_eq!(get_admin(&app), admins_string);

    // set new admin by admin should succeed
    add_admin(&mut app, admins[0].clone(), new_admin.to_string()).unwrap();
    admins_string.push(new_admin.to_string());
    assert_eq!(get_admin(&app), admins_string);

    // adding the same admin should fail
    add_admin(&mut app, admins[0].clone(), new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );

    // now we test removing admin

    // first try to remove admin by non admin should fail
    remove_admin(&mut app, registrar, new_admin.to_string()).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );

    // try removing a non admin, it should fail
    remove_admin(&mut app, admins[0].clone(), String::from("non-admin")).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );

    // remove admin by admin should succeed
    remove_admin(&mut app, admins[0].clone(), new_admin.to_string()).unwrap();

    // remove new_admin from the admins_string vec
    admins_string.retain(|x| x != &new_admin.to_string());
    assert_eq!(get_admin(&app), admins_string);
}
