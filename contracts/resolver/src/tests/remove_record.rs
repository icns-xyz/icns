#![cfg(test)]

use crate::{
    crypto::{create_adr36_message, pubkey_to_bech32_address},
    msg::{self, Adr36Info, ExecuteMsg, AddressesResponse},
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
fn set_primary_name_on_set_first_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1, admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let primary_name = |app: &BasicApp, address: String| -> StdResult<_> {
        let PrimaryNameResponse { name } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::PrimaryName { address },
        )?;

        Ok(name)
    };

    let mint_and_set_record = |app: &mut BasicApp, name: &str, signer: &SigningKey| {
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
            name: name.to_string(),
            adr36_info: Adr36Info {
                bech32_address: addr.to_string(),
                address_hash: msg::AddressHash::SHA256,
                pub_key: signer.to_binary(),
                signature,
            },
            bech32_prefix: "osmo".to_string(),
            signature_salt: 12313u128.into(),
        };

        app.execute_contract(
            Addr::unchecked(addr),
            resolver_contract_addr.clone(),
            &msg,
            &[],
        )
        .unwrap();
    };

    // make sure primary name is correctly set
    mint_and_set_record(&mut app, "isabel", &signer1());
    assert_eq!(
        primary_name(
            &app,
            pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string())
        )
        .unwrap(),
        "isabel".to_string()
    );

    // let msg = ExecuteMsg::RemoveRecord {
    //     name: "isabel".to_string(),
    //     bech32_address: pubkey_to_bech32_address(signer1().to_binary(), "osmo".to_string()),
    //     replace_primary_address: 
    // };

}