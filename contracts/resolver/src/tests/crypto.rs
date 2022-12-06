use base64::encode as base64_encode;
use bech32::ToBase32;
use cosmwasm_crypto::secp256k1_verify;
use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::Binary;
use hex_literal::hex;
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use sha3::Keccak256;
use std::ops::Deref;
use subtle_encoding::hex::{decode as hex_decode, self};

use crate::msg::AddressHash;
use crate::{
    crypto::{
        adr36_verification, create_adr36_data, create_adr36_message, pubkey_to_bech32_address,
    },
    msg::Adr36Info,
    tests::helpers::signer1,
};

use super::helpers::ToBinary;

#[test]
fn pubkey_to_address() {
    let original_binary_vec =
        hex_decode("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc").unwrap();

    // first check using pubkey_to_bech32_address method
    let pub_key_binary = Binary::from(original_binary_vec.clone());
    let bech32_address = pubkey_to_bech32_address(pub_key_binary, "osmo".to_string());
    assert_eq!(
        bech32_address,
        "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697"
    );

    // check each step of the method individually
    let sha256 = Sha256::digest(original_binary_vec);
    let result = Ripemd160::digest(sha256);

    assert_eq!(
        result.as_ref(),
        hex!("6aad751bb99fb0f13237f027b45eeebc37bec200")
    );

    let bech32_address =
        bech32::encode("osmo", result.deref().to_base32(), bech32::Variant::Bech32).unwrap();

    assert_eq!(
        bech32_address,
        "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697"
    );
}

#[test]
fn secp256k1_verification() {
    let message = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"dGVzdA==\",\"signer\":\"osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697\"}}],\"sequence\":\"0\"}";
    let bytes = message.as_bytes();
    let hashed = Sha256::digest(bytes);

    let signature = hex!("8c009e1fa58d6ae5dfcda93208f800dbd8815f20ea9c690b56a5758e999c9cb66fdb764b1e070d65ea22fe5827214631b1aba54730a9dfa74dc37b73da529c00");
    let pub_key = hex!("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc");

    let verify_result = secp256k1_verify(&hashed, &signature, &pub_key).unwrap();
    assert_eq!(verify_result, true);

    let false_signature = hex!("8c009e1fa58d6ae5dfcda93208f800dbd8815f20ea9c690b56a5758e999c9cb66fdb764b1e070d65ea22fe5827214631b1aba54730a9dfa74dc37b73da529c01");
    let verify_result = secp256k1_verify(&hashed, &false_signature, &pub_key).unwrap();
    assert_eq!(verify_result, false);

    let false_message = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"aW52YWxpZA==\",\"signer\":\"osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697\"}}],\"sequence\":\"0\"}";
    let bytes = false_message.as_bytes();
    let hashed = Sha256::digest(bytes);

    let verify_result = secp256k1_verify(&hashed, &signature, &pub_key).unwrap();
    assert_eq!(verify_result, false);
}

#[test]
fn create_valid_adr36_data() {
    let name = "alice".to_string();
    let bech32_prefix = "cosmos".to_string();
    let owner = "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string();
    let chain_id = "cosmos-testnet-14002".to_string();
    let contract_address = "contract1".to_string();
    let signature_salt = 12313;

    let message = create_adr36_data(
        name,
        bech32_prefix,
        owner,
        chain_id,
        contract_address,
        signature_salt,
    );

    let expected_message =
        "The following is the information for ICNS registration for alice.cosmos.

Chain id: cosmos-testnet-14002
Contract Address: contract1
Owner: osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697
Salt: 12313"
            .to_string();

    let expected_base_64 = base64_encode(expected_message);
    println!("expected_base_64: {}", expected_base_64);
    assert_eq!(message, expected_base_64);
}

#[test]
fn create_valid_adr36_message() {
    let name = "alice".to_string();
    let bech32_prefix = "cosmos".to_string();
    let sender = "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string();
    let signer = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    let chain_id = "cosmos-testnet-14002".to_string();
    let contract_address = "contract1".to_string();
    let signature_salt = 12313;

    let message = create_adr36_message(
        name,
        bech32_prefix,
        sender,
        signer.clone(),
        chain_id,
        contract_address,
        signature_salt,
    );

    let message_prefix = "{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{\"amount\":[],\"gas\":\"0\"},\"memo\":\"\",\"msgs\":[{\"type\":\"sign/MsgSignData\",\"value\":{\"data\":\"";
    let data = "VGhlIGZvbGxvd2luZyBpcyB0aGUgaW5mb3JtYXRpb24gZm9yIElDTlMgcmVnaXN0cmF0aW9uIGZvciBhbGljZS5jb3Ntb3MuCgpDaGFpbiBpZDogY29zbW9zLXRlc3RuZXQtMTQwMDIKQ29udHJhY3QgQWRkcmVzczogY29udHJhY3QxCk93bmVyOiBvc21vMWQya2gyeGFlbjdjMHp2M2g3cW5tZ2hod2hzbW1hc3NxaHFzNjk3ClNhbHQ6IDEyMzEz".to_string();
    let signer_prefix = "\",\"signer\":\"";
    let signer = signer;
    let message_suffix = "\"}}],\"sequence\":\"0\"}";

    let expected_message = format!(
        "{}{}{}{}{}",
        message_prefix, data, signer_prefix, signer, message_suffix
    );

    assert_eq!(message, expected_message);
}

#[test]
fn adr36_verify() {
    let name = "alice".to_string();
    let bech32_prefix = "cosmos".to_string();
    let owner = "osmo1d2kh2xaen7c0zv3h7qnmghhwhsmmassqhqs697".to_string();
    let signer = "cosmos1cyyzpxplxdzkeea7kwsydadg87357qnalx9dqz".to_string();
    let chain_id = "cosmos-testnet-14002".to_string();
    let contract_address = "contract1".to_string();
    let signature_salt = 12313;

    let original_signature_vec = hex!("624fcd052ed8333fe643140ab5fde6fa308dd02c95cb61dd490ab53afa622db12a79ba2826b7da85d56c53bd4e53947b069cc3fb6fb091ca938f8d1952dfdf50");
    let pub_key = signer1().to_binary();
    let signature = Binary::from(original_signature_vec);

    let adr36_info = Adr36Info {
        signer_bech32_address: signer,
        address_hash: AddressHash::Cosmos,
        pub_key,
        signature,
    };

    let deps = mock_dependencies();
    let adr_verification = adr36_verification(
        deps.as_ref(),
        name,
        owner,
        bech32_prefix,
        adr36_info,
        chain_id,
        contract_address,
        signature_salt,
    )
    .is_err();
    assert!(!adr_verification)
}

#[test]
fn keccack256_digest() {
    let original_binary_vec = 
        hex_decode("12345678").unwrap();
    
    let keccack256 =Keccak256::digest(&original_binary_vec);
    assert_eq!(
        keccack256.as_ref(),
        hex!("30ca65d5da355227c97ff836c9c6719af9d3835fc6bc72bddc50eeecc1bb2b25")
    );
}