use cosmwasm_std::Binary;
use hex_literal::hex;
use subtle_encoding::{hex::{decode as hex_decode, self}};
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use std::ops::Deref;
use cosmwasm_crypto::{secp256k1_verify};
use bech32::ToBase32;
use base64::{encode as base64_encode};
use cosmwasm_std::testing::{mock_dependencies};

use crate::{crypto::{pubkey_to_bech32_address, create_adr36_message, create_adr36_data, adr36_verification}, msg::AddressInfo};
use crate::msg::AddressHash;

#[test]
fn pubkey_to_address() {
    let original_binary_vec = hex_decode("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc").unwrap();

    // first check using pubkey_to_bech32_address method
    let pub_key_binary =  Binary::from(original_binary_vec.clone());
    let bech32_address = pubkey_to_bech32_address(pub_key_binary, "osmo".to_string());
    assert_eq!(
        bech32_address,
        "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697"
    );

    // check each step of the method individually
    let sha256 = Sha256::digest(original_binary_vec.clone());
    let result = Ripemd160::digest(sha256);

    assert_eq!(
        result.as_ref(),
        hex!("6aad751bb99fb0f13237f027b45eeebc37bec200")
    );

    let bech32_address = bech32::encode("osmo", result.deref().to_base32(), bech32::Variant::Bech32).unwrap();
    
    assert_eq!(
        bech32_address,
        "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697"
    );
}

#[test]
fn secp256k1_verification() {
    let message = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"dGVzdA==\",\"signer\":\"osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697\"}}],\"sequence\":\"0\"}";
    let bytes = message.as_bytes();
    let hashed  = Sha256::digest(bytes);

    let signature = hex!("8c009e1fa58d6ae5dfcda93208f800dbd8815f20ea9c690b56a5758e999c9cb66fdb764b1e070d65ea22fe5827214631b1aba54730a9dfa74dc37b73da529c00");
    let pub_key = hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");


    let verify_result = secp256k1_verify(&hashed, &signature, &pub_key).unwrap();
    assert_eq!(verify_result, true);

    let false_signature = hex!("8c009e1fa58d6ae5dfcda93208f800dbd8815f20ea9c690b56a5758e999c9cb66fdb764b1e070d65ea22fe5827214631b1aba54730a9dfa74dc37b73da529c01");
    let verify_result = secp256k1_verify(&hashed, &false_signature, &pub_key).unwrap();
    assert_eq!(verify_result, false);

    let false_message = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"aW52YWxpZA==\",\"signer\":\"osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697\"}}],\"sequence\":\"0\"}";
    let bytes = false_message.as_bytes();
    let hashed  = Sha256::digest(bytes);

    let verify_result = secp256k1_verify(&hashed, &signature, &pub_key).unwrap();
    assert_eq!(verify_result, false);
}

#[test]
fn create_valid_adr36_data() {
    let user_name = "tony".to_string();
    let bech32_prefix = "osmo".to_string();
    let bech32_address = "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string();
    let chain_id = "osmosis-1".to_string();
    let contract_address = "osmo1cjta2pw3ltzsvy9phdvtvqprexclt0p3m9aj54".to_string();
    let signature_salt = 1323124;

    let message = create_adr36_data(
        user_name,
        bech32_prefix,
        bech32_address,
        chain_id,
        contract_address,
        signature_salt
    );

    let expected_message = "The following is the information for ICNS registration for tony.osmo.

Chain id: osmosis-1
Contract Address: osmo1cjta2pw3ltzsvy9phdvtvqprexclt0p3m9aj54
Address: osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697
Salt: 1323124".to_string();

    let expected_base_64 = base64_encode(expected_message);
    assert_eq!(
        message,
        expected_base_64
    );
}

#[test]
fn create_valid_adr36_message() {
    let user_name = "tony".to_string();
    let bech32_prefix = "osmo".to_string();
    let bech32_address = "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string();
    let chain_id = "osmosis-1".to_string();
    let contract_address = "osmo1cjta2pw3ltzsvy9phdvtvqprexclt0p3m9aj54".to_string();
    let signature_salt = 1323124;

    let message = create_adr36_message(
        user_name,
        bech32_prefix,
        bech32_address.clone(),
        chain_id,
        contract_address,
        signature_salt
    );

    let message_prefix = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"";
    let data = "VGhlIGZvbGxvd2luZyBpcyB0aGUgaW5mb3JtYXRpb24gZm9yIElDTlMgcmVnaXN0cmF0aW9uIGZvciB0b255Lm9zbW8uCgpDaGFpbiBpZDogb3Ntb3Npcy0xCkNvbnRyYWN0IEFkZHJlc3M6IG9zbW8xY2p0YTJwdzNsdHpzdnk5cGhkdnR2cXByZXhjbHQwcDNtOWFqNTQKQWRkcmVzczogb3NtbzFkMmtoMnhhZW43YzB6djNoN3FubWdoaHdoc21tYXNzcWhxczY5NwpTYWx0OiAxMzIzMTI0".to_string();
    let signer_prefix = "\",\"signer\":\"";
    let signer = bech32_address.clone();
    let message_suffix = "\"}}],\"sequence\":\"0\"}";

    let expected_message = format!("{}{}{}{}{}", message_prefix, data, signer_prefix, signer, message_suffix);

    assert_eq!(
        message,
        expected_message
    );
}

#[test]
fn adr36_verify() {
    let user_name = "tony".to_string();
    let bech32_prefix = "osmo".to_string();
    let bech32_address = "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string();
    let chain_id = "cosmos-testnet-14002".to_string();
    let contract_address = "contract0".to_string();
    let signature_salt = 1323124;
    
    let original_pubkey_vec = hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");
    let original_signature_vec = hex!("79d0a79004a709090d4038e9e0df5e0744c5065bf03fe7a30b60872a414d85de4e023a8c8123cf6519af86cfa996fc24618651bbbf8ab13732cef9d10c577d97");
    let pub_key = Binary::from(original_pubkey_vec);
    let signature = Binary::from(original_signature_vec);

    
    let address_info = AddressInfo {
        bech32_address, 
        address_hash: AddressHash::SHA256,
        pub_key: pub_key,
        signature: signature
    };

    let deps = mock_dependencies();
    let adr_verification = adr36_verification(
        deps.as_ref(),
        user_name,
        bech32_prefix,
        address_info,
        chain_id,
        contract_address,
        signature_salt
    ).is_err();
    assert!(!adr_verification)
    

}