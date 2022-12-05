use crate::state::SIGNATURE;
use base64::encode as base64_encode;
use bech32::ToBase32;
use cosmwasm_crypto::secp256k1_verify;
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use std::ops::Deref;

use cosmwasm_std::{Binary, Deps, Response};

use crate::{msg::Adr36Info, ContractError};

pub fn adr36_verification(
    deps: Deps,
    name: String,
    sender: String,
    bech32_prefix: String,
    adr36_info: Adr36Info,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> Result<Response, ContractError> {
    // extract pubkey to bech32 address, check that it matches with the given bech32 address
    let decoded_bech32_addr =
        pubkey_to_bech32_address(adr36_info.pub_key.clone(), bech32_prefix.clone());
    if decoded_bech32_addr != adr36_info.signer_bech32_address {
        return Err(ContractError::SigntaureMisMatch {});
    }

    let signtaure = SIGNATURE.may_load(deps.storage, adr36_info.signature.as_slice())?;
    if signtaure.is_some() {
        return Err(ContractError::SigntaureAlreadyExists {});
    }

    let message = create_adr36_message(
        name,
        bech32_prefix,
        sender,
        adr36_info.signer_bech32_address,
        chain_id,
        contract_address,
        signature_salt,
    );

    let message_bytes = message.as_bytes();
    let message_hash = Sha256::digest(message_bytes);

    // verify signature using secp256k1
    let verified_result =
        secp256k1_verify(&message_hash, &adr36_info.signature, &adr36_info.pub_key)
            .map_err(|_| ContractError::SigntaureMisMatch {})?;
    if !verified_result {
        return Err(ContractError::SigntaureMisMatch {});
    }

    Ok(Response::default())
}

pub fn pubkey_to_bech32_address(pub_key: Binary, bech32_prefix: String) -> String {
    let decoded_pub_key = pub_key.as_slice();
    let sha256 = Sha256::digest(decoded_pub_key);
    let result = Ripemd160::digest(sha256);

    let bech32_address = bech32::encode(
        &bech32_prefix,
        result.deref().to_base32(),
        bech32::Variant::Bech32,
    )
    .unwrap();

    bech32_address
}

pub fn create_adr36_message(
    name: String,
    bech32_prefix: String,
    sender: String,
    signer_bech32_address: String,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> String {
    let message_prefix = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"";
    let data = create_adr36_data(
        name,
        bech32_prefix,
        sender.clone(),
        chain_id,
        contract_address,
        signature_salt,
    );
    let signer_prefix = "\",\"signer\":\"";
    let message_suffix = "\"}}],\"sequence\":\"0\"}";
    let message = format!(
        "{}{}{}{}{}",
        message_prefix, data, signer_prefix, signer_bech32_address, message_suffix
    );

    message
}

pub fn create_adr36_data(
    name: String,
    bech32_prefix: String,
    sender: String,
    chain_id: String,
    contract_address: String,
    signature_salt: u128,
) -> String {
    let icns = name + "." + &bech32_prefix;
    let address = sender;
    let salt = signature_salt.to_string();

    let data_string = format!(
        "The following is the information for ICNS registration for {}.

Chain id: {}
Contract Address: {}
Owner: {}
Salt: {}",
        icns, chain_id, contract_address, address, salt
    );

    base64_encode(data_string)
}
