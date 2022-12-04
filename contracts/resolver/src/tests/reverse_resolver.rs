#![cfg(test)]

use crate::{
    msg::{self, Adr36Info, ExecuteMsg},
    msg::{GetAddressesResponse, QueryMsg},
};

use cosmwasm_std::{Addr, Binary, Empty, StdResult};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};

use cw_multi_test::{BasicApp, Executor};
use hex_literal::hex;
use icns_name_nft::msg::ExecuteMsg as NameExecuteMsg;

use super::helpers::{default_setting, instantiate_name_nft, instantiate_resolver_with_name_nft};

#[test]
fn reverse_resolver() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2];
    let registrar = String::from("default-registrar");

    let (name_nft_contract, resolver_contract_addr, mut app) = default_setting(admins, registrar.clone());
    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let GetAddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddresses { user_name: name },
        )?;

        Ok(addresses)
    };

    // now set record for the same address, same pub key but for different user name
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

   let original_pubkey_vec =
   hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");
    let original_signature_vec = hex!("8e07dbc31b135ac5d9e79c23940e70e94fae4edf0bcd8267094b04c6c1744e736df693a72429fee86b08fbe946852210bcc015b88966f470a275b1c3e2c1196b");
    let pub_key = Binary::from(original_pubkey_vec);
    let signature = Binary::from(original_signature_vec);
    let record_msg = ExecuteMsg::SetRecord {
        user_name: "bob".to_string(),
        adr36_info: Adr36Info {
            // invalid address
            bech32_address: "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string(),
            address_hash: msg::AddressHash::SHA256,
            pub_key: pub_key.clone(),
            signature: signature.clone(),
        },
        bech32_prefix: "osmo".to_string(),
        replace_primary_if_exists: false,
        signature_salt: 13231,
    };

    let err = app
        .execute_contract(
            Addr::unchecked(admin1.clone()),
            resolver_contract_addr.clone(),
            &record_msg,
            &[],
        )
        .is_err();
    assert_eq!(err, false
    );
}