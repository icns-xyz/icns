use cosmrs::{
    crypto::secp256k1::VerifyingKey,
    tendermint::signature::{Secp256k1Signature, Verifier},
};
use cosmwasm_std::{to_binary, Addr, Deps, QueryRequest, WasmQuery};
use icns_name_nft::msg::{AdminResponse, QueryMsg as NameNFTQueryMsg};

use crate::{state::CONFIG, ContractError};

pub fn check_send_from_admin(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let AdminResponse { admins } = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CONFIG.load(deps.storage)?.name_nft.to_string(),
        msg: to_binary(&NameNFTQueryMsg::Admin {})?,
    }))?;

    if !admins.contains(&sender.to_string()) {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn check_verification_pass_threshold(
    deps: Deps,
    msg: &str,
    verifcations: &[Vec<u8>],
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let passed_verifications = verifcations
        .iter()
        .filter_map(|signature| {
            config
                .verifiers
                .iter()
                // TODO: Report invalid signature as it is an important debugging information
                .find(|verifier| verify_secp256k1_signature(msg, signature, verifier).is_ok())
        })
        .count() as u64;

    if passed_verifications < config.verification_threshold {
        return Err(ContractError::ValidVerificationIsBelowThreshold {
            expected: config.verification_threshold,
            actual: passed_verifications,
        });
    }

    Ok(())
}
// signature is der encoded
// pubkey is sec-1 encoded
fn verify_secp256k1_signature(
    msg: &str,
    signature: &[u8],
    pubkey: &[u8],
) -> Result<(), ContractError> {
    let verifying_key = check_verifying_key(pubkey)?;
    let signature = check_signature(signature)?;

    verifying_key
        .verify(msg.as_bytes(), &signature)
        .map_err(|_| ContractError::InvalidSignature {})
}

pub fn check_verifying_key(pubkey: &[u8]) -> Result<VerifyingKey, ContractError> {
    VerifyingKey::from_sec1_bytes(pubkey).map_err(|_| ContractError::InvalidPublicKeyFormat {})
}

pub fn check_signature(signature: &[u8]) -> Result<Secp256k1Signature, ContractError> {
    Secp256k1Signature::from_der(signature).map_err(|_| ContractError::InvalidSignatureFormat {})
}

#[cfg(test)]
mod test {

    use super::verify_secp256k1_signature;
    use crate::ContractError;
    use cosmrs::{
        bip32::{self},
        crypto::secp256k1::SigningKey,
    };

    fn from_mnemonic(phrase: &str, derivation_path: &str) -> SigningKey {
        let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)
            .unwrap()
            .to_seed("");
        let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse().unwrap()).unwrap();
        xprv.into()
    }

    #[test]
    fn test_verify_secp256k1_signature() {
        let derivation_path = "m/44'/118'/0'/0/0"; //
        let verifier1 = from_mnemonic("notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius", derivation_path);

        // all sec1 encoded should be ok
        let pubkey = verifier1.public_key().to_bytes();
        let msg = r#"{"name":"boss"}"#;
        let signature = verifier1.sign(msg.as_bytes()).unwrap().to_der();

        assert_eq!(
            verify_secp256k1_signature(msg, signature.as_bytes(), &pubkey),
            Ok(())
        );

        assert_eq!(
            verify_secp256k1_signature(msg, &[69, 69, 69], &pubkey),
            Err(ContractError::InvalidSignatureFormat {})
        );

        assert_eq!(
            verify_secp256k1_signature(msg, signature.as_bytes(), &[69, 69, 69]),
            Err(ContractError::InvalidPublicKeyFormat {})
        );

        assert_eq!(
            verify_secp256k1_signature(r#"{"name":"slave"}"#, signature.as_bytes(), &pubkey),
            Err(ContractError::InvalidSignature {})
        );
    }
}
