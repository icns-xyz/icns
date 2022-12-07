#![cfg(test)]

use crate::{
    crypto::cosmos_pubkey_to_bech32_address,
    msg::{self, Adr36Info, ExecuteMsg, NamesResponse},
    msg::{AddressesResponse, QueryMsg},
    tests::helpers::{mint_and_set_record, signer1, ToBinary},
};

use cosmwasm_std::{Addr, Binary, Empty, StdResult};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};

use cw_multi_test::{BasicApp, Executor};
use hex_literal::hex;
use icns_name_nft::msg::ExecuteMsg as NameExecuteMsg;
use subtle_encoding::hex::decode as hex_decode;

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
        let NamesResponse {
            names,
            primary_name,
        } = app
            .wrap()
            .query_wasm_smart(resolver_contract_addr.clone(), &QueryMsg::Names { address })?;

        Ok((names, primary_name))
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
        (vec!["alice".to_string()], "alice".to_string())
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
        registrar,
        name_nft_contract,
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
        let NamesResponse {
            names,
            primary_name,
        } = app
            .wrap()
            .query_wasm_smart(resolver_contract_addr.clone(), &QueryMsg::Names { address })?;

        Ok((names, primary_name))
    };

    // now get record
    let addresses = addresses(&app, "alice".to_string()).unwrap();
    assert_eq!(
        addresses,
        vec![("cosmos".to_string(), signer_bech32_address.to_owned()),]
    );

    assert_eq!(
        names(&app, signer_bech32_address).unwrap(),
        (
            vec!["alice".to_string(), "alice_in_wonderland".to_string()],
            "alice_in_wonderland".to_string()
        )
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

    let addr1 = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
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
            signature_salt: 1323124u128.into(),
        },
        bech32_prefix: "osmo".to_string(),
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
            signature_salt: 1323124u128.into(),
        },
        bech32_prefix: "juno".to_string(),
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
            signature_salt: 12313u128.into(),
        },
        bech32_prefix: "cosmos".to_string(),
    };
    app.execute_contract(
        Addr::unchecked(addr1),
        resolver_contract_addr,
        &record_msg,
        &[],
    )
    .unwrap();

    // println!("err: {}", err.downcast_ref::<ContractError>().unwrap());
    // assert_eq!(err, false);
}

#[test]
fn eth_address_set_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let pub_key_bytes =
        hex_decode("0422b7d0ab1ec915bf3902bd4d3a1dde5d0add15865f951d7ac3fb206e9e898f2d2cd59418a2a27b98eb1e39fc33c55faeed8e550dbf9226a594203c0c2430b0d7")
        .unwrap();
    let pub_key_binary = Binary::from(pub_key_bytes);

    let sender_pub_key_bytes =
        hex_decode("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc").unwrap();

    // first check using cosmos_pubkey_to_bech32_address method
    let sender_pub_key_binary = Binary::from(sender_pub_key_bytes);

    let addr = cosmos_pubkey_to_bech32_address(sender_pub_key_binary, "osmo".to_string());

    let original_signature_bytes = hex!("d67d5dc9f33f2a680c635bdae898c1c6a9ee39cd946ae9e2df827dd25eb50d6f6d7adc2926741d9adc84780f5a06bae226c30cd110af91f4092b45e3e521445c");
    let signature = Binary::from(original_signature_bytes);

    let mint = app
        .execute_contract(
            Addr::unchecked(registrar),
            name_nft_contract,
            &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: "alice".to_string(),
                owner: addr.to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // now set record
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        bech32_prefix: "evmos".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: "evmos16wx7ye3ce060tjvmmpu8lm0ak5xr7gm238xyss".to_string(),
            address_hash: msg::AddressHash::Ethereum,
            pub_key: pub_key_binary,
            signature,
            signature_salt: 12313u128.into(),
        },
    };

    app.execute_contract(
        Addr::unchecked(addr),
        resolver_contract_addr,
        &record_msg,
        &[],
    )
    .unwrap();
}

#[test]
fn adr36_verification_bypass() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addr = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());

    // use invalid pub key and signature
    let pub_key_bytes = hex!("aaaa");
    let signature_bytes = hex!("bbbb");
    let pub_key = Binary::from(pub_key_bytes);
    let signature = Binary::from(signature_bytes);

    let mint = app
        .execute_contract(
            Addr::unchecked(registrar),
            name_nft_contract,
            &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: "alice".to_string(),
                owner: addr.to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // use address with different bech32 prefix
    let different_bech32_prefix_address =
        cosmos_pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string());
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        bech32_prefix: "cosmos".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: different_bech32_prefix_address,
            address_hash: msg::AddressHash::Ethereum,
            pub_key: pub_key,
            signature,
            signature_salt: 12313u128.into(),
        },
    };

    app.execute_contract(
        Addr::unchecked(addr),
        resolver_contract_addr,
        &record_msg,
        &[],
    )
    .unwrap();
}
