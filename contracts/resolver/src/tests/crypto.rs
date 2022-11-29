use hex_literal::hex;
use subtle_encoding::{hex::decode as hex_decode};
use ripemd::{Digest as RipemdDigest, Ripemd160};
use sha2::Sha256;
use std::ops::Deref;
use cosmwasm_crypto::{secp256k1_verify};
use bech32::ToBase32;


#[test]
fn pubkey_to_address() {
    let original_hex = hex_decode("02394bc53633366a2ab9b5d697a94c8c0121cc5e3f0d554a63167edb318ceae8bc").unwrap();

    let sha256 = Sha256::digest(original_hex);

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