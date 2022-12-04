#![cfg(test)]

use crate::{
    crypto::{create_adr36_message, pubkey_to_bech32_address},
    msg::{self, Adr36Info, ExecuteMsg},
    msg::{PrimaryNameResponse, QueryMsg},
    tests::helpers::{instantiate_name_nft, instantiate_resolver_with_name_nft, signer2},
    ContractError,
};

use cosmrs::crypto::secp256k1::SigningKey;
use cw721_base::{Extension, MintMsg};
use icns_name_nft::CW721BaseExecuteMsg;

use cosmwasm_std::{Addr, Empty, StdResult};

use cw_multi_test::{BasicApp, Executor};

use super::helpers::{signer1, ToBinary};

#[test]
fn replace_primary_if_exists() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins.clone(), registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let primary_name = |app: &BasicApp, address: String| -> StdResult<_> {
        let PrimaryNameResponse { name } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::PrimaryName { address },
        )?;

        Ok(name)
    };

    let mint_and_set_record =
        |app: &mut BasicApp, name: &str, signer: &SigningKey, replace_primary_if_exists: bool| {
            let addr = pubkey_to_bech32_address(signer.to_binary(), "osmo".to_string());

            app.execute_contract(
                Addr::unchecked(registrar.clone()),
                name_nft_contract.clone(),
                &CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                    token_id: name.to_string(),
                    owner: addr.to_string(),
                    token_uri: None,
                    extension: None,
                }),
                &[],
            )
            .unwrap();

            let multitest_chain_id = "cosmos-testnet-14002";

            let msg = create_adr36_message(
                name.to_string(),
                "osmo".to_string(),
                addr.to_string(),
                multitest_chain_id.to_string(),
                resolver_contract_addr.to_string(),
                12313,
            );

            let signature = signer.sign(msg.as_bytes()).unwrap().to_binary();

            let msg = ExecuteMsg::SetRecord {
                user_name: name.to_string(),
                adr36_info: Adr36Info {
                    bech32_address: addr.to_string(),
                    address_hash: msg::AddressHash::SHA256,
                    pub_key: signer.to_binary(),
                    signature,
                },
                bech32_prefix: "osmo".to_string(),
                replace_primary_if_exists,
                signature_salt: 12313,
            };

            app.execute_contract(
                Addr::unchecked(addr),
                resolver_contract_addr.clone(),
                &msg,
                &[],
            )
            .unwrap();
        };

    // if there is single name for an address, then that's primary
    mint_and_set_record(&mut app, "isabel", &signer1(), false);
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string())
        )
        .unwrap(),
        "isabel".to_string()
    );

    // if there is single name for an address, then that's primary even with replace_primary_if_exists = false
    mint_and_set_record(&mut app, "marley", &signer2(), true);
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string())
        )
        .unwrap(),
        "marley".to_string()
    );

    // does not change primary if replace_primary_if_exists is false on existing address
    mint_and_set_record(&mut app, "isakaya", &signer1(), false);
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string())
        )
        .unwrap(),
        "isabel".to_string()
    );

    // change primary if replace_primary_if_exists is true on existing address
    mint_and_set_record(&mut app, "isann", &signer1(), true);
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string())
        )
        .unwrap(),
        "isann".to_string()
    );
}

#[test]
fn set_primary() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins.clone(), registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let primary_name = |app: &BasicApp, address: String| -> StdResult<_> {
        let PrimaryNameResponse { name } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::PrimaryName { address },
        )?;

        Ok(name)
    };

    let mint_and_set_record =
        |app: &mut BasicApp, name: &str, signer: &SigningKey, replace_primary_if_exists: bool| {
            let addr = pubkey_to_bech32_address(signer.to_binary(), "osmo".to_string());

            app.execute_contract(
                Addr::unchecked(registrar.clone()),
                name_nft_contract.clone(),
                &CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                    token_id: name.to_string(),
                    owner: addr.to_string(),
                    token_uri: None,
                    extension: None,
                }),
                &[],
            )
            .unwrap();

            let multitest_chain_id = "cosmos-testnet-14002";

            let msg = create_adr36_message(
                name.to_string(),
                "osmo".to_string(),
                addr.to_string(),
                multitest_chain_id.to_string(),
                resolver_contract_addr.to_string(),
                12313,
            );

            let signature = signer.sign(msg.as_bytes()).unwrap().to_binary();

            let msg = ExecuteMsg::SetRecord {
                user_name: name.to_string(),
                adr36_info: Adr36Info {
                    bech32_address: addr.to_string(),
                    address_hash: msg::AddressHash::SHA256,
                    pub_key: signer.to_binary(),
                    signature,
                },
                bech32_prefix: "osmo".to_string(),
                replace_primary_if_exists,
                signature_salt: 12313,
            };

            app.execute_contract(
                Addr::unchecked(addr),
                resolver_contract_addr.clone(),
                &msg,
                &[],
            )
            .unwrap();
        };

    mint_and_set_record(&mut app, "isabel", &signer1(), false);
    mint_and_set_record(&mut app, "isakaya", &signer1(), false);
    mint_and_set_record(&mut app, "isann", &signer1(), false);

    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string())
        )
        .unwrap(),
        "isabel".to_string()
    );

    let addr1 = pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string());
    let addr2 = pubkey_to_bech32_address(signer2().to_binary(), "osmo".to_string());

    // non-owner can't set primary
    let err = app
        .execute_contract(
            Addr::unchecked(addr2),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetPrimary {
                name: "isann".to_string(),
            },
            &[],
        )
        .unwrap_err();

    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    // set primary with name that they do not own is not allowed
    app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "others_name".to_string(),
            owner: "someone_else".to_string(),
            token_uri: None,
            extension: None,
        }),
        &[],
    )
    .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked(addr1.clone()),
            resolver_contract_addr.clone(),
            &ExecuteMsg::SetPrimary {
                name: "others_name".to_string(),
            },
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );

    // only owner can set primary
    app.execute_contract(
        Addr::unchecked(addr1),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetPrimary {
            name: "isann".to_string(),
        },
        &[],
    )
    .unwrap();

    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string())
        )
        .unwrap(),
        "isann".to_string()
    );
}
