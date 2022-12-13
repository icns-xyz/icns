#![cfg(test)]

use crate::{
    crypto::{cosmos_pubkey_to_bech32_address, create_adr36_message, eth_pubkey_to_bech32_address},
    msg::{self, Adr36Info, Bech32Address, ExecuteMsg, NamesResponse},
    msg::{AddressesResponse, QueryMsg},
    tests::helpers::{mint_and_set_record, signer1, ToBinary},
    ContractError,
};

use cosmwasm_std::{Addr, Binary, Empty, StdResult, Uint128};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, MintMsg};

use cw_multi_test::{BasicApp, Executor};
use hex_literal::hex;
use icns_name_nft::msg::{ExecuteMsg as NameExecuteMsg, Metadata};
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
            Bech32Address {
                bech32_prefix: "cosmos".to_string(),
                address: "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string()
            },
            Bech32Address {
                bech32_prefix: "juno".to_string(),
                address: "juno1d2kh2xaen7c0zv3h7qnmghhwhsmmassqffq35s".to_string()
            },
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
        vec![Bech32Address {
            bech32_prefix: "cosmos".to_string(),
            address: signer_bech32_address.to_owned()
        }]
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
    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract,
        &NameExecuteMsg::Mint(MintMsg {
            token_id: "alice".to_string(),
            owner: addr1.to_string(),
            token_uri: None,
            extension: Metadata { referral: None },
        }),
        &[],
    )
    .unwrap();

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
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Bech32DecodingErr {
            addr: "osmo".to_string()
        }
    );

    // now try setting record with unmatching bech32 prefix and address
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            address_hash: msg::AddressHash::Cosmos,
            pub_key: pub_key,
            signature: signature,
            signature_salt: 1323124u128.into(),
        },
        bech32_prefix: "juno".to_string(),
    };

    let err = app
        .execute_contract(
            Addr::unchecked(addr1),
            resolver_contract_addr,
            &record_msg,
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Bech32PrefixMismatch {
            prefix: "juno".to_string(),
            addr: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string()
        }
    );
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
        hex_decode("04ec18c82501c5088119251679b538e9cf8eae502956cc862c7778aa148365e886fb4a05d4685b9c9e16032bd41db1c41e16f6ffe5115462725737b2e995697b3e")
        .unwrap();
    let pub_key_binary = Binary::from(pub_key_bytes);

    let sender_pub_key_bytes =
        hex_decode("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc").unwrap();

    // first check using cosmos_pubkey_to_bech32_address method
    let sender_pub_key_binary = Binary::from(sender_pub_key_bytes);

    let addr = cosmos_pubkey_to_bech32_address(sender_pub_key_binary, "osmo".to_string());

    let original_signature_bytes = hex!("87365a0f80671d16bb136094135e57473a59b8dbc2e514b9f03f67453db17a881b53b16e4eadfef092998d188afe79cfd63dd6be1a77be5058c2801094b7932f");
    let signature = Binary::from(original_signature_bytes);

    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract,
        &NameExecuteMsg::Mint(MintMsg {
            token_id: "alice".to_string(),
            owner: addr.to_string(),
            token_uri: None,
            extension: Metadata { referral: None },
        }),
        &[],
    )
    .unwrap();

    // now set record
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        bech32_prefix: "evmos".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: "evmos1puzp8aevdnjngsuwv6qk3855syanpf4tmvmd0e".to_string(),
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

    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract,
        &NameExecuteMsg::Mint(MintMsg {
            token_id: "alice".to_string(),
            owner: addr.to_string(),
            token_uri: None,
            extension: Metadata { referral: None },
        }),
        &[],
    )
    .unwrap();

    // use address with different bech32 prefix
    let different_bech32_prefix_address =
        cosmos_pubkey_to_bech32_address(signer1().to_binary(), "cosmos".to_string());
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        bech32_prefix: "cosmos".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: different_bech32_prefix_address.clone(),
            address_hash: msg::AddressHash::Ethereum,
            pub_key,
            signature,
            signature_salt: 12313u128.into(),
        },
    };

    app.execute_contract(
        Addr::unchecked(addr.clone()),
        resolver_contract_addr.clone(),
        &record_msg,
        &[],
    )
    .unwrap_err();

    let pub_key = Binary::from(pub_key_bytes);
    let record_msg = ExecuteMsg::SetRecord {
        name: "alice".to_string(),
        bech32_prefix: "cosmos".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: different_bech32_prefix_address,
            address_hash: msg::AddressHash::Ethereum,
            pub_key,
            signature: Binary::default(),
            signature_salt: Uint128::new(0),
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

// TODO: add test for testing withg different pub key and sender with secp256k1
#[test]
fn same_pubkey_invalid_bech_32() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) =
        default_setting(admins, registrar.clone());

    // create osmo address
    let addr = cosmos_pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let multitest_chain_id = "cosmos-testnet-14002";

    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract,
        &CW721BaseExecuteMsg::<Metadata, Empty>::Mint(MintMsg {
            token_id: "bob".to_string(),
            owner: addr.to_string(),
            token_uri: None,
            extension: Metadata { referral: None },
        }),
        &[],
    )
    .unwrap();

    let msg = create_adr36_message(
        "bob".to_string(),
        // bech32_prefix.clone(),
        "osmo".to_string(),
        addr.to_string(),
        addr.to_string(),
        multitest_chain_id.to_string(),
        resolver_contract_addr.to_string(),
        12313,
    );
    let signature = signer1().sign(msg.as_bytes()).unwrap().to_binary();

    // this should fail because the bech32 prefix is different
    let msg = ExecuteMsg::SetRecord {
        name: "bob".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: addr.to_string(),
            address_hash: msg::AddressHash::Cosmos,
            pub_key: signer1().to_binary(),
            signature,
            signature_salt: 12313u128.into(),
        },
        bech32_prefix: "cosmos".to_string(),
    };
    app.execute_contract(
        Addr::unchecked(addr),
        resolver_contract_addr.clone(),
        &msg,
        &[],
    )
    .unwrap_err();

    // now try this with eth address
    let eth_addr = eth_pubkey_to_bech32_address(signer1().to_binary(), "evmos".to_string());
    let msg = create_adr36_message(
        "bob".to_string(),
        // bech32_prefix.clone(),
        "evmos".to_string(),
        eth_addr.to_string(),
        eth_addr.to_string(),
        multitest_chain_id.to_string(),
        resolver_contract_addr.to_string(),
        12313,
    );
    let signature = signer1().sign(msg.as_bytes()).unwrap().to_binary();
    let msg = ExecuteMsg::SetRecord {
        name: "bob".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: eth_addr.to_string(),
            address_hash: msg::AddressHash::Cosmos,
            pub_key: signer1().to_binary(),
            signature,
            signature_salt: 12313u128.into(),
        },
        bech32_prefix: "eth".to_string(),
    };
    app.execute_contract(Addr::unchecked(eth_addr), resolver_contract_addr, &msg, &[])
        .unwrap_err();
}

#[test]
fn set_record_with_different_signer_and_signature_owner_wrong_signature() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) =
        default_setting(admins, registrar.clone());

    // create osmo address
    let pub_key_bytes =
        hex_decode("0322b7d0ab1ec915bf3902bd4d3a1dde5d0add15865f951d7ac3fb206e9e898f2d").unwrap();
    let pub_key_binary = Binary::from(pub_key_bytes);
    let addr = cosmos_pubkey_to_bech32_address(pub_key_binary.clone(), "juno".to_string());

    assert_eq!(addr, "juno1c8qw55n7vl6j0yvct7gmyg3hlmx026ek8r55g9",);

    let wrong_original_signature_vec = hex!("65b953369240beddbd0c3df5d7fe0e2519312daacd912828a93ed44d801d492b0720003d23fe4b7958dfa6126c5213d61f4f693bbc00aeabe71253d81960a778");
    let wrong_signature = Binary::from(wrong_original_signature_vec);

    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract,
        &CW721BaseExecuteMsg::<Metadata, Empty>::Mint(MintMsg {
            token_id: "carol".to_string(),
            owner: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            token_uri: None,
            extension: Metadata { referral: None },
        }),
        &[],
    )
    .unwrap();

    let msg = ExecuteMsg::SetRecord {
        name: "carol".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: addr,
            address_hash: msg::AddressHash::Cosmos,
            pub_key: pub_key_binary,
            signature: wrong_signature,
            signature_salt: 633234u128.into(),
        },
        bech32_prefix: "juno".to_string(),
    };

    let err = app
        .execute_contract(
            Addr::unchecked("osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string()),
            resolver_contract_addr,
            &msg,
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::SignatureMisMatch {}
    );
}

#[test]
fn set_record_with_different_signer_and_signature_owner() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) =
        default_setting(admins, registrar.clone());

    // create osmo address
    let pub_key_bytes =
        hex_decode("0322b7d0ab1ec915bf3902bd4d3a1dde5d0add15865f951d7ac3fb206e9e898f2d").unwrap();
    let pub_key_binary = Binary::from(pub_key_bytes);
    let addr = cosmos_pubkey_to_bech32_address(pub_key_binary.clone(), "juno".to_string());

    assert_eq!(addr, "juno1c8qw55n7vl6j0yvct7gmyg3hlmx026ek8r55g9",);

    app.execute_contract(
        Addr::unchecked(registrar),
        name_nft_contract,
        &CW721BaseExecuteMsg::<Metadata, Empty>::Mint(MintMsg {
            token_id: "carol".to_string(),
            owner: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            token_uri: None,
            extension: Metadata { referral: None },
        }),
        &[],
    )
    .unwrap();

    let original_signature_vec = hex!("65b953369240beddbd0c3df5d7fe0e2519312daacd912828a93ed44d801d492b0720003d23fe4b7958dfa6126c5213d61f4f693bbc00aeabe71253d81960a779");
    let signature = Binary::from(original_signature_vec);

    // this should fail because the bech32 prefix is different
    let msg = ExecuteMsg::SetRecord {
        name: "carol".to_string(),
        adr36_info: Adr36Info {
            signer_bech32_address: addr,
            address_hash: msg::AddressHash::Cosmos,
            pub_key: pub_key_binary,
            signature,
            signature_salt: 633234u128.into(),
        },
        bech32_prefix: "juno".to_string(),
    };
    app.execute_contract(
        Addr::unchecked("osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697"),
        resolver_contract_addr,
        &msg,
        &[],
    )
    .unwrap();
}
