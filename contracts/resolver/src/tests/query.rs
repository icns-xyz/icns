#![cfg(test)]

use crate::{
    msg::{QueryMsg, GetAddressesResponse},
    msg::{AdminResponse, ExecuteMsg},
    ContractError, contract::is_admin,
};

use cosmwasm_std::{Addr, Empty, StdResult};
use cw_multi_test::{BasicApp, Executor};
use icns_name_nft::{msg::ExecuteMsg as NameExecuteMsg, msg::QueryMsg as NameQueryMsg};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};


use super::helpers::{
    instantiate_name_nft, instantiate_resolver_with_name_nft, TestEnv,
    TestEnvBuilder,
};

#[test]
fn query_admins_asd() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    // query admin
    let msg = QueryMsg::Admin {};

    // change from `let res: Vec<String> to this:
    let AdminResponse { admins } = app
        .wrap()
        .query_wasm_smart(resolver_contract_addr.clone(), &msg)
        .unwrap();
    
    assert_eq!(admins, vec![admin1, admin2]);
}

