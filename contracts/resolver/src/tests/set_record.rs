#![cfg(test)]

use crate::{
    msg::QueryMsg,
    msg::{AdminResponse, ExecuteMsg},
    ContractError,
};

use cosmwasm_std::Addr;
use cw_multi_test::{BasicApp, Executor};

use super::helpers::{
    instantiate_name_nft_with_admins_and_new_app, instantiate_resolver_with_name_nft, TestEnv,
    TestEnvBuilder,
};

#[test]
fn only_admin_can_set_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft_with_admins_and_new_app(admins);

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let set_record =
        |app: &mut BasicApp, sender: Addr, user_name: String, addresses: Vec<(String, String)>| {
            let msg = ExecuteMsg::SetRecord {
                user_name,
                addresses,
            };

            app.execute_contract(resolver_contract_addr.clone(), sender, &msg, &[])
        };

    // query admin
    let msg = QueryMsg::Admin {};

    // change from `let res: Vec<String> to this:
    let AdminResponse { admins } = app
        .wrap()
        .query_wasm_smart(resolver_contract_addr.clone(), &msg)
        .unwrap();

    dbg!(admins);

    // // first try executing set record with non admin
    // let non_admin = Addr::unchecked("non_admin");
    // let admin = Addr::unchecked(admin1);
    // let user_name = String::from("bob");
    // let addresses = vec![(String::from("osmo"), String::from("osmo1xxxxx")), (String::from("juno"), String::from("juno1xxxxx"))];

    // let err = set_record(
    //     &mut app,
    //     admin,
    //     user_name.clone(),
    //     addresses.clone())
    // .is_err();

    // println!("err: {:?}", err);
    // // assert_eq!(
    // //     err.downcast_ref::<ContractError>().unwrap(),
    // //     &ContractError::Unauthorized {},
    // // )
}
