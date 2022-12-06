#![cfg(test)]

use crate::{
    crypto::pubkey_to_bech32_address,
    msg::{self, Adr36Info, ExecuteMsg, NamesResponse},
    msg::{AddressesResponse, QueryMsg},
    tests::helpers::{mint_and_set_record, signer1, ToBinary},
    ContractError,
};

use cosmwasm_std::{Addr, Binary, Empty, StdResult};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};

use cw_multi_test::{BasicApp, Executor};
use hex_literal::hex;
use icns_name_nft::msg::ExecuteMsg as NameExecuteMsg;

use super::helpers::{default_setting, instantiate_name_nft, instantiate_resolver_with_name_nft};

#[test]
fn set_get_single_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (_, resolver_contract_addr, app) = default_setting(admins, registrar);
    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let AddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Addresses { name },
        )?;

        Ok(addresses)
    };

    let names = |app: &BasicApp, address: String| -> StdResult<_> {
        let NamesResponse { names } = app
            .wrap()
            .query_wasm_smart(resolver_contract_addr.clone(), &QueryMsg::Names { address })?;

        Ok(names)
    };

    // now get record
    let addresses = addresses(&app, "alice".to_string()).unwrap();
    assert_eq!(
        addresses,
        vec![
            (
                "cosmos".to_string(),
                "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string()
            ),
            (
                "juno".to_string(),
                "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s".to_string()
            )
        ]
    );
    assert_eq!(
        names(
            &app,
            "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string()
        )
        .unwrap(),
        vec!["alice"]
    );
}

#[test]
fn set_get_multiple_name_on_one_address() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let signer_bech32_address = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();

    mint_and_set_record(
        &mut app,
        "alice",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    mint_and_set_record(
        &mut app,
        "alice_in_wonderland",
        signer_bech32_address.clone(),
        &signer1(),
        registrar.clone(),
        name_nft_contract.clone(),
        resolver_contract_addr.clone(),
    );

    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let AddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::Addresses { name },
        )?;

        Ok(addresses)
    };

    let names = |app: &BasicApp, address: String| -> StdResult<_> {
        let NamesResponse { names } = app
            .wrap()
            .query_wasm_smart(resolver_contract_addr.clone(), &QueryMsg::Names { address })?;

        Ok(names)
    };

    // now get record
    let addresses = addresses(&app, "alice".to_string()).unwrap();
    assert_eq!(
        addresses,
        vec![("cosmos".to_string(), signer_bech32_address.to_owned()),]
    );

    assert_eq!(
        names(&app, signer_bech32_address).unwrap(),
        vec!["alice", "alice_in_wonderland"]
    );
}

#[test]
fn bech32_verification() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    // mint name nft to alice
    let mint = app
        .execute_contract(
            Addr::unchecked(registrar),
            name_nft_contract,
            &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: "alice".to_string(),
                owner: addr1.to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // now set record, first try setting invalid bech32 address
    let original_signature_vec = hex!("624fcd052ed8333fe643140ab5fde6fa308dd02c95cb61dd490ab53afa622db12a79ba2826b7da85d56c53bd4e53947b069cc3fb6fb091ca938f8d1952dfdf50");
    let pub_key = signer1().to_binary();
    let signature = Binary::from(original_signature_vec);
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        adr36_info: Adr36Info {
            // invalid address
            signer_bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs699".to_string(),
            address_hash: msg::AddressHash::Cosmos,
            pub_key: pub_key.clone(),
            signature: signature.clone(),
        },
        bech32_prefix: "osmo".to_string(),
        signature_salt: 1323124u128.into(),
    };

    let err = app
        .execute_contract(
            Addr::unchecked(addr1.clone()),
            resolver_contract_addr.clone(),
            &record_msg,
            &[],
        )
        .is_err();
    assert_eq!(err, true);

    // now try setting record with unmatching bech32 prefix and address
    let record_msg = ExecuteMsg::SetRecord {
        name: "tony".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            address_hash: msg::AddressHash::Cosmos,
            pub_key: pub_key.clone(),
            signature: signature.clone(),
        },
        bech32_prefix: "juno".to_string(),
        signature_salt: 1323124u128.into(),
    };
    let err = app
        .execute_contract(
            Addr::unchecked(addr1.clone()),
            resolver_contract_addr.clone(),
            &record_msg,
            &[],
        )
        .is_err();
    assert_eq!(err, true);

    // now set record with valid bech32 prefix and addresses, this should succeed
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        adr36_info: Adr36Info {
            // invalid address
            signer_bech32_address: "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string(),
            address_hash: msg::AddressHash::Cosmos,
            pub_key,
            signature,
        },
        bech32_prefix: "cosmos".to_string(),
        signature_salt: 12313u128.into(),
    };
    let err = app
        .execute_contract(
            Addr::unchecked(addr1),
            resolver_contract_addr,
            &record_msg,
            &[],
        )
        .unwrap_err();

    println!("err: {}", err.downcast_ref::<ContractError>().unwrap());
    // assert_eq!(err, false);
}
