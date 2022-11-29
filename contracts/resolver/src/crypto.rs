use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use subtle_encoding::{hex::decode as hex_decode};
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use std::ops::Deref;
use bech32::ToBase32;
use base64::{encode as base64_encode};
use cosmwasm_crypto::{secp256k1_verify};
use crate::state::{Config, ADDRESSES, REVERSE_RESOLVER, CONFIG, SIGNATURE};


use cosmwasm_std::{Binary,to_binary, Addr, CosmosMsg, StdResult, WasmMsg, Response, Deps};

use crate::{msg::{ExecuteMsg, AddressInfo}, ContractError};

pub fn adr36_verification(
    deps: Deps,
    user_name: String,
    bech32_prefix: String,
    address_info: AddressInfo,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> Result<Response, ContractError> {
       
    // extract pubkey to bech32 address, check that it matches with the given bech32 address
    let decoded_bech32_addr = pubkey_to_bech32_address(address_info.pub_key.clone(), bech32_prefix.clone());
    if decoded_bech32_addr != address_info.bech32_address.clone() {
        return Err(ContractError::SigntaureMisMatch {  });
    }

    let signtaure = SIGNATURE.may_load(deps.storage, address_info.signature.as_slice())?;
    if signtaure.is_some() {
        return Err(ContractError::SigntaureAlreadyExists {  });
    }

   let message = create_adr36_message(
        user_name,
        bech32_prefix,
        address_info.bech32_address,
        chain_id,
        contract_address,
        signature_salt
    ); 

    // verify signature using secp256k1
    let verified_result = secp256k1_verify(&message.as_bytes(), &address_info.signature, &address_info.pub_key)
       .map_err(|_| ContractError::SigntaureMisMatch {  })?;
    if !verified_result {
        return Err(ContractError::SigntaureMisMatch {  });
    }

    Ok(Response::default())
}


pub fn pubkey_to_bech32_address(pub_key: Binary, bech32_prefix: String) -> String {
    let decoded_pub_key = pub_key.as_slice();
    let sha256 = Sha256::digest(decoded_pub_key);
    let result = Ripemd160::digest(sha256);

    let bech32_address = bech32::encode(&bech32_prefix, result.deref().to_base32(), bech32::Variant::Bech32).unwrap();

    return bech32_address
}


pub fn create_adr36_message(
    user_name: String,
    bech32_prefix: String,
    bech32_address: String,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> String {
    let message_prefix = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"";
    let data = create_adr36_data(user_name, bech32_prefix, bech32_address.clone(), chain_id, contract_address, signature_salt);
    let signer_prefix = "\",\"signer\":\"";
    let message_suffix = "\"}}],\"sequence\":\"0\"}";
    let message = format!("{}{}{}{}{}", message_prefix, data, signer_prefix, bech32_address, message_suffix);

    message
}

pub fn create_adr36_data(
    user_name: String,
    bech32_prefix: String,
    bech32_address: String,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> String {
    let icns = user_name.clone() + "." + &bech32_prefix.clone();
    let address = bech32_address.clone();
    let salt = signature_salt.to_string();

    let data_string = format!("
    The following is the information for ICNS registration for {}.

Chain id: {}
Contract Address: {}
Address: {}
Salt: {}", icns, chain_id, contract_address, address, salt);
    let data = base64_encode(data_string);

    return data
}