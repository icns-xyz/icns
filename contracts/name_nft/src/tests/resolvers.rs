#![cfg(test)]

use crate::{
    msg::{ExecuteMsg, ICNSNameExecuteMsg, ResolversResponse},
    tests::helpers::{TestEnv, TestEnvBuilder},
    QueryMsg,
};

use cosmwasm_std::Addr;

use cw721_base::ContractError;
use cw_multi_test::{BasicApp, Executor};

#[test]
fn only_admin_can_update_resolvers() {
    let TestEnv {
        mut app,
        contract_addr,
        registrar,
        admin,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let resolvers = |app: &BasicApp| {
        let ResolversResponse { resolvers } = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Resolvers {})
            .unwrap();
        resolvers
    };

    let update_resolvers =
        |app: &mut BasicApp, sender: Addr, add: Vec<String>, remove: Vec<String>| {
            app.execute_contract(
                sender,
                contract_addr.clone(),
                &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::UpdateResolvers { add, remove }),
                &[],
            )
        };

    assert_eq!(resolvers(&app), vec![] as Vec<Addr>);

    // resolvers can't be updated by random person
    let random_person = Addr::unchecked("random_person");
    let err = update_resolvers(
        &mut app,
        random_person.clone(),
        vec![random_person.to_string()],
        vec![],
    )
    .unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert_eq!(resolvers(&app), vec![] as Vec<Addr>);

    // resolvers can't be updated by registrar
    let err =
        update_resolvers(&mut app, registrar, vec![random_person.to_string()], vec![]).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert_eq!(resolvers(&app), vec![] as Vec<Addr>);

    // resolvers can only be updated by admin
    update_resolvers(
        &mut app,
        admin.clone(),
        vec![random_person.to_string()],
        vec![],
    )
    .unwrap();
    assert_eq!(resolvers(&app), vec![random_person.clone()]);

    update_resolvers(&mut app, admin, vec![], vec![random_person.to_string()]).unwrap();
    assert_eq!(resolvers(&app), vec![] as Vec<Addr>);
}

#[test]
fn update_resolvers_add_and_remove() {
    let TestEnv {
        mut app,
        contract_addr,
        admin,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let resolvers = |app: &BasicApp| {
        let ResolversResponse { resolvers } = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Resolvers {})
            .unwrap();
        resolvers
    };

    let update_resolvers =
        |app: &mut BasicApp, sender: Addr, add: Vec<String>, remove: Vec<String>| {
            app.execute_contract(
                sender,
                contract_addr.clone(),
                &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::UpdateResolvers { add, remove }),
                &[],
            )
        };

    assert_eq!(resolvers(&app), vec![] as Vec<Addr>);

    // update with duplicate addresses
    let random_person = Addr::unchecked("random_person");
    update_resolvers(
        &mut app,
        admin.clone(),
        vec![random_person.to_string(), random_person.to_string()],
        vec![],
    )
    .unwrap();
    assert_eq!(resolvers(&app), vec![random_person.clone()]);

    // update with existing address
    update_resolvers(
        &mut app,
        admin.clone(),
        vec![random_person.to_string(), admin.to_string()],
        vec![],
    )
    .unwrap();
    assert_eq!(resolvers(&app), vec![random_person.clone(), admin.clone()]);

    // update with add/remove same address
    update_resolvers(
        &mut app,
        admin.clone(),
        vec![random_person.to_string(), admin.to_string()],
        vec![random_person.to_string()],
    )
    .unwrap();
    assert_eq!(resolvers(&app), vec![admin.clone()]);

    // remove more than it has
    update_resolvers(
        &mut app,
        admin.clone(),
        vec![],
        vec![random_person.to_string(), admin.to_string()],
    )
    .unwrap();
    assert_eq!(resolvers(&app), vec![] as Vec<Addr>);
}
