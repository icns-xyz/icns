use cosmrs::{
    crypto::secp256k1::VerifyingKey,
    tendermint::signature::{Secp256k1Signature, Verifier},
};
use cosmwasm_std::{
    from_slice, to_binary, Addr, Decimal, Deps, Env, MessageInfo, QueryRequest, WasmQuery,
};
use icns_name_nft::msg::{AdminResponse, QueryMsg as NameNFTQueryMsg};
use itertools::Itertools;
use sha2::{Digest, Sha256};

use crate::{msg::VerifyingMsg, state::CONFIG, ContractError};

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

pub fn check_verfying_msg(
    env: &Env,
    info: &MessageInfo,
    name: &str,
    verifying_msg: &str,
) -> Result<(), ContractError> {
    let verifying_msg: VerifyingMsg = from_slice(verifying_msg.as_bytes())?;
    if verifying_msg.name != name {
        return Err(ContractError::InvalidVerifyingMessage {
            msg: format!(
                "name mismatched: expected `{}` but got `{}`",
                name, verifying_msg.name
            ),
        });
    }
    if verifying_msg.claimer != info.sender {
        return Err(ContractError::InvalidVerifyingMessage {
            msg: format!(
                "claimer mismatched: expected `{}` but got `{}`",
                info.sender, verifying_msg.claimer
            ),
        });
    }
    if verifying_msg.contract_address != env.contract.address {
        return Err(ContractError::InvalidVerifyingMessage {
            msg: format!(
                "contract address mismatched: expected `{}` but got `{}`",
                env.contract.address, verifying_msg.contract_address
            ),
        });
    }
    if verifying_msg.chain_id != env.block.chain_id {
        return Err(ContractError::InvalidVerifyingMessage {
            msg: format!(
                "chain id mismatched: expected `{}` but got `{}`",
                env.block.chain_id, verifying_msg.chain_id
            ),
        });
    }
    Ok(())
}

// TODO: test this
pub fn check_verification_pass_threshold(
    deps: Deps,
    msg: &str,
    verifcations: &[Vec<u8>],
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // SHA256(msg)
    // let mut sha256 = Sha256::new();
    // sha256.update(msg.as_bytes());
    // let hashed_msg = sha256.finalize();

    // TODO: err on non unqiue signature

    let passed_verifications = verifcations
        .iter()
        .unique()
        .filter_map(|signature| {
            config
                .verifier_pubkeys
                .iter()
                // TODO: Report invalid signature as it is an important debugging information
                .find(|verifier| {
                    verify_secp256k1_signature(msg.as_bytes(), signature, verifier).is_ok()
                })
        })
        .count() as u64;

    config.check_pass_threshold(Decimal::new(passed_verifications.into()))?;

    Ok(())
}
// signature is der encoded
// pubkey is sec-1 encoded
fn verify_secp256k1_signature(
    msg: &[u8],
    signature: &[u8],
    pubkey: &[u8],
) -> Result<(), ContractError> {
    let verifying_key = check_verifying_key(pubkey)?;
    let signature = check_signature(signature)?;

    verifying_key
        .verify(msg, &signature)
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
    use super::*;
    use crate::{
        state::{Config, CONFIG},
        ContractError,
    };
    use cosmrs::{
        bip32::{self},
        crypto::secp256k1::SigningKey,
    };
    use cosmwasm_std::{testing::mock_dependencies, Addr, Decimal, DepsMut, Uint128};

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
            verify_secp256k1_signature(msg.as_bytes(), signature.as_bytes(), &pubkey),
            Ok(())
        );

        assert_eq!(
            verify_secp256k1_signature(msg.as_bytes(), &[69, 69, 69], &pubkey),
            Err(ContractError::InvalidSignatureFormat {})
        );

        assert_eq!(
            verify_secp256k1_signature(msg.as_bytes(), signature.as_bytes(), &[69, 69, 69]),
            Err(ContractError::InvalidPublicKeyFormat {})
        );

        assert_eq!(
            verify_secp256k1_signature(
                r#"{"name":"slave"}"#.as_bytes(),
                signature.as_bytes(),
                &pubkey
            ),
            Err(ContractError::InvalidSignature {})
        );
    }

    #[test]
    fn test_check_verification_pass_threshold() {
        let derivation_path = "m/44'/118'/0'/0/0";
        let verifier1 = || {
            from_mnemonic("notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius", derivation_path)
        };
        let verifier2 = || {
            from_mnemonic("quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty", derivation_path)
        };
        let verifier3 = || {
            from_mnemonic("symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb", derivation_path)
        };
        let non_verifier = || {
            from_mnemonic("bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty", derivation_path)
        };

        let mut set_threshold_pct = |deps: DepsMut, pct: u64| {
            CONFIG
                .save(
                    deps.storage,
                    &Config {
                        name_nft: Addr::unchecked("namenftaddr"),
                        verifier_pubkeys: vec![verifier1(), verifier2(), verifier3()]
                            .iter()
                            .map(|sk| sk.public_key().to_bytes())
                            .collect(),
                        verification_threshold_percentage: Decimal::percent(pct),
                    },
                )
                .unwrap();
        };

        let sign_all = |verfiers: &[SigningKey], msg: &str| {
            verfiers
                .iter()
                .map(|v| v.sign(msg.as_bytes()).unwrap().to_der().as_bytes().to_vec())
                .collect::<Vec<Vec<u8>>>()
        };

        let msg = "verifying msg";

        // test 2/3 valid signature > 51% should be passed
        let mut deps = mock_dependencies();
        set_threshold_pct(deps.as_mut(), 51);
        check_verification_pass_threshold(
            deps.as_ref(),
            msg,
            &sign_all(&[verifier1(), verifier2()], msg),
        )
        .unwrap();

        // test 1/3 valid signature < 51% should error
        let mut deps = mock_dependencies();
        set_threshold_pct(deps.as_mut(), 51);
        let err =
            check_verification_pass_threshold(deps.as_ref(), msg, &sign_all(&[verifier1()], msg))
                .unwrap_err();

        assert_eq!(
            err,
            ContractError::ValidVerificationIsBelowThreshold {
                expected_over: Decimal::new(510000000000000000u128.into()),
                actual: Decimal::new(333333333333333333u128.into()),
            }
        );

        // test 1/3 valid signature (one signed by non verifier) < 51% should error
        let mut deps = mock_dependencies();
        set_threshold_pct(deps.as_mut(), 51);
        let err = check_verification_pass_threshold(
            deps.as_ref(),
            msg,
            &sign_all(&[verifier1(), non_verifier()], msg),
        )
        .unwrap_err();
        assert_eq!(
            err,
            ContractError::ValidVerificationIsBelowThreshold {
                expected_over: Decimal::new(510000000000000000u128.into()),
                actual: Decimal::new(333333333333333333u128.into()),
            }
        );

        // test 1/3 valid signature (one signed wrong msg) < 51% should error
        let mut deps = mock_dependencies();
        set_threshold_pct(deps.as_mut(), 51);

        let wrong_msg = "wrong msg";
        let err = check_verification_pass_threshold(
            deps.as_ref(),
            msg,
            &vec![
                sign_all(&[verifier1()], msg),
                sign_all(&[verifier2()], wrong_msg),
            ]
            .concat(),
        )
        .unwrap_err();
        assert_eq!(
            err,
            ContractError::ValidVerificationIsBelowThreshold {
                expected_over: Decimal::new(510000000000000000u128.into()),
                actual: Decimal::new(333333333333333333u128.into()),
            }
        );

        // test 1/3 valid signature (one duplicated signature) < 51% should error
        let mut deps = mock_dependencies();
        set_threshold_pct(deps.as_mut(), 51);
        let err = check_verification_pass_threshold(
            deps.as_ref(),
            msg,
            &vec![sign_all(&[verifier3(), verifier3()], msg)].concat(),
        )
        .unwrap_err();
        assert_eq!(
            err,
            ContractError::ValidVerificationIsBelowThreshold {
                expected_over: Decimal::new(510000000000000000u128.into()),
                actual: Decimal::new(333333333333333333u128.into()),
            }
        );
    }
}
