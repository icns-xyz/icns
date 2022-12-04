#![cfg(test)]

use crate::{
    msg::{self, Adr36Info, ExecuteMsg},
    msg::{AddressesResponse, QueryMsg},
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
            &QueryMsg::Addresses { name: name },
        )?;

        Ok(addresses)
    };

    // now get record
    let addresses = addresses(&app, "tony".to_string()).unwrap();
    assert_eq!(addresses.len(), 2);
    assert_eq!(addresses[0].0, "juno");
    assert_eq!(
        addresses[0].1,
        "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s"
    );
    assert_eq!(addresses[1].0, "osmo");
    assert_eq!(
        addresses[1].1,
        "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697"
    );
}

#[test]
fn bech32_verification() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    // mint name nft to bob
    let mint = app
        .execute_contract(
            Addr::unchecked(registrar),
            name_nft_contract,
            &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: "bob".to_string(),
                owner: "bob".to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .is_err();
    assert_eq!(mint, false);

    // now set record, first try setting invalid bech32 address
    let original_pubkey_vec =
        hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");
    let original_signature_vec = hex!("74331c35c9dd49eb3d39f693afc363e77e5541d94839639b7c71e2f18b001295561f123cb169128a34aedb15dddd1caa42e3cbc39104cb07a32658e9de5707a1");
    let pub_key = Binary::from(original_pubkey_vec);
    let signature = Binary::from(original_signature_vec);
    let record_msg = ExecuteMsg::SetRecord {
        name: "tony".to_string(),
        adr36_info: Adr36Info {
            // invalid address
            bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs699".to_string(),
            address_hash: msg::AddressHash::SHA256,
            pub_key: pub_key.clone(),
            signature: signature.clone(),
        },
        bech32_prefix: "osmo".to_string(),
        signature_salt: 1323124u128.into(),
    };

    let err = app
        .execute_contract(
            Addr::unchecked(admin1.clone()),
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
            bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            address_hash: msg::AddressHash::SHA256,
            pub_key: pub_key.clone(),
            signature: signature.clone(),
        },
        bech32_prefix: "juno".to_string(),
        signature_salt: 1323124u128.into(),
    };
    let err = app
        .execute_contract(
            Addr::unchecked(admin1.clone()),
            resolver_contract_addr.clone(),
            &record_msg,
            &[],
        )
        .is_err();
    assert_eq!(err, true);

    // now set record with valid bech32 prefix and addresses, this should succeed
    let record_msg = ExecuteMsg::SetRecord {
        name: "tony".to_string(),
        adr36_info: Adr36Info {
            // invalid address
            bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            address_hash: msg::AddressHash::SHA256,
            pub_key,
            signature,
        },
        bech32_prefix: "osmo".to_string(),
        signature_salt: 1323124u128.into(),
    };
    let err = app
        .execute_contract(
            Addr::unchecked(admin1),
            resolver_contract_addr,
            &record_msg,
            &[],
        )
        .is_err();
    assert_eq!(err, false);
}
