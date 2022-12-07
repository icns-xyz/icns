use cosmrs::{
    bip32::secp256k1::schnorr::signature::Signature,
    crypto::secp256k1::VerifyingKey,
    tendermint::signature::{Secp256k1Signature, Verifier},
};
use cosmwasm_std::{
    from_slice, to_binary, Addr, Binary, Coin, Decimal, Deps, Env, MessageInfo, QueryRequest,
    WasmQuery,
};

use icns_name_nft::msg::{AdminResponse, QueryMsg as NameNFTQueryMsg};
use itertools::Itertools;

use crate::{msg::VerifyingMsg, state::CONFIG, state::UNIQUE_TWITTER_ID, ContractError};

pub fn is_admin(deps: Deps, address: &Addr) -> Result<bool, ContractError> {
    let AdminResponse { admins } = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CONFIG.load(deps.storage)?.name_nft.to_string(),
        msg: to_binary(&NameNFTQueryMsg::Admin {})?,
    }))?;

    if !admins.contains(&address.to_string()) {
        return Ok(false);
    }

    Ok(true)
}

pub fn check_admin(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let AdminResponse { admins } = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CONFIG.load(deps.storage)?.name_nft.to_string(),
        msg: to_binary(&NameNFTQueryMsg::Admin {})?,
    }))?;

    if !admins.contains(&sender.to_string()) {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn check_fee(deps: Deps, funds: &[Coin]) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if let Some(fee) = config.fee {
        if funds != [fee.to_owned()] {
            return Err(ContractError::InvalidFee { fee_required: fee });
        }
    }

    Ok(())
}

pub fn check_verfying_msg(
    deps: Deps,
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

    // check if unique twitter id is not stored
    if UNIQUE_TWITTER_ID
        .may_load(deps.storage, verifying_msg.unique_twitter_id.clone())?
        .is_some()
    {
        return Err(ContractError::DuplicatedTwitterId {
            msg: format!(
                "unique twitter id `{}` is already used",
                verifying_msg.unique_twitter_id
            ),
        });
    }

    Ok(())
}

pub fn check_verification_pass_threshold(
    deps: Deps,
    msg: &str,
    // (public_key, signature)
    verifications: &[(Vec<u8>, Vec<u8>)],
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // check if verification comes from verifier
    verifications
        .iter()
        .try_for_each(|(pubkey, _)| -> Result<(), ContractError> {
            config
                .verifier_pubkeys
                .iter()
                .find(|verifier| *verifier == pubkey)
                .ok_or(ContractError::NotAVerifierPublicKey {
                    public_key: Binary(pubkey.clone()),
                })?;
            Ok(())
        })?;

    // check for duplicated signatures
    let duplicated_signatures = verifications
        .iter()
        .map(|(_, signature)| signature)
        .duplicates()
        .next();

    if let Some(duplicated_signature) = duplicated_signatures {
        let signature = check_signature(duplicated_signature)?;
        let signature = Binary(signature.to_vec());
        return Err(ContractError::DuplicatedVerification { signature });
    }

    // verify all signatures
    verifications
        .iter()
        .unique()
        .try_for_each(|(public_key, signature)| {
            verify_secp256k1_signature(msg.as_bytes(), signature, public_key)
        })?;

    // when all signature are valid, no duplicates and all from verifiers, check if it pass threshold
    let verification_count = verifications.len() as u64;
    config.check_pass_threshold(Decimal::new(verification_count.into()))?;

    Ok(())
}

fn verify_secp256k1_signature(
    msg: &[u8],
    signature: &[u8], // bytes of r/s components
    pubkey: &[u8],    // SEC-1 encoded
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
    Secp256k1Signature::from_bytes(signature).map_err(|_| ContractError::InvalidSignatureFormat {})
}

pub fn check_valid_threshold(percent: &Decimal) -> Result<(), ContractError> {
    if *percent > Decimal::percent(100) || *percent < Decimal::percent(0) {
        Err(ContractError::InvalidThreshold {})
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        state::{Config, CONFIG},
        ContractError,
    };
    use cosmrs::{bip32, crypto::secp256k1::SigningKey};
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Coin, Decimal, DepsMut,
    };

    use crate::tests::helpers::ToBinary;

    fn from_mnemonic(phrase: &str, derivation_path: &str) -> SigningKey {
        let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)
            .unwrap()
            .to_seed("");
        let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse().unwrap()).unwrap();
        xprv.into()
    }

    #[test]
    fn test_check_verifying_message() {
        let deps = mock_dependencies();
        let env = mock_env();
        let contract_address = &env.contract.address;
        let chain_id = &env.block.chain_id;
        let sender = "sender";
        let unique_twitter_id = "unique_twitter_id";
        let info = mock_info(sender, &[]);
        let name = "name";

        // success case, everything is matched
        check_verfying_msg(deps.as_ref(), &env, &info, name, &format!(
            r#"{{"name":"{name}","claimer":"{sender}","contract_address":"{contract_address}","chain_id":"{chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
        )).unwrap();

        // name mismatched
        let mismatched_name = "mismatched_name";
        let err = check_verfying_msg(deps.as_ref(), &env, &info, name, &format!(
            r#"{{"name":"{mismatched_name}","claimer":"{sender}","contract_address":"{contract_address}","chain_id":"{chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
        )).unwrap_err();

        assert_eq!(
            err,
            ContractError::InvalidVerifyingMessage {
                msg: format!(
                    "name mismatched: expected `{}` but got `{}`",
                    name, mismatched_name
                ),
            }
        );

        // claimer is not sender
        let not_a_sender = "not_a_sender";
        let err = check_verfying_msg(deps.as_ref(),&env, &info, name, &format!(
            r#"{{"name":"{name}","claimer":"{not_a_sender}","contract_address":"{contract_address}","chain_id":"{chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
        )).unwrap_err();

        assert_eq!(
            err,
            ContractError::InvalidVerifyingMessage {
                msg: format!(
                    "claimer mismatched: expected `{}` but got `{}`",
                    sender, not_a_sender
                ),
            }
        );

        // wrong contract_address
        let wrong_contract_address = "wrong_contract_address";
        let err = check_verfying_msg(deps.as_ref(), &env, &info, name, &format!(
            r#"{{"name":"{name}","claimer":"{sender}","contract_address":"{wrong_contract_address}","chain_id":"{chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
        )).unwrap_err();

        assert_eq!(
            err,
            ContractError::InvalidVerifyingMessage {
                msg: format!(
                    "contract address mismatched: expected `{}` but got `{}`",
                    contract_address, wrong_contract_address
                ),
            }
        );

        // wrong chain_id
        let wrong_chain_id = "wrong_chain_id";
        let err = check_verfying_msg(deps.as_ref(), &env, &info, name, &format!(
                    r#"{{"name":"{name}","claimer":"{sender}","contract_address":"{contract_address}","chain_id":"{wrong_chain_id}","unique_twitter_id":"{unique_twitter_id}"}}"#,
                )).unwrap_err();

        assert_eq!(
            err,
            ContractError::InvalidVerifyingMessage {
                msg: format!(
                    "chain id mismatched: expected `{}` but got `{}`",
                    chain_id, wrong_chain_id
                ),
            }
        );
    }

    #[test]
    fn test_verify_secp256k1_signature() {
        let derivation_path = "m/44'/118'/0'/0/0"; //
        let verifier1 = from_mnemonic("notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius", derivation_path);

        // all sec1 encoded should be ok
        let pubkey = verifier1.public_key().to_bytes();
        let msg = r#"{"name":"boss"}"#;
        let signature = verifier1.sign(msg.as_bytes()).unwrap().to_vec();

        assert_eq!(
            verify_secp256k1_signature(msg.as_bytes(), &signature, &pubkey),
            Ok(())
        );

        assert_eq!(
            verify_secp256k1_signature(msg.as_bytes(), &[69, 69, 69], &pubkey),
            Err(ContractError::InvalidSignatureFormat {})
        );

        assert_eq!(
            verify_secp256k1_signature(msg.as_bytes(), &signature, &[69, 69, 69]),
            Err(ContractError::InvalidPublicKeyFormat {})
        );

        assert_eq!(
            verify_secp256k1_signature(r#"{"name":"slave"}"#.as_bytes(), &signature, &pubkey),
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
        let verifier4 = || {
            from_mnemonic("bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty", derivation_path)
        };
        let non_verifier = || {
            from_mnemonic("prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder", derivation_path)
        };

        let set_threshold_pct = |deps: DepsMut, pct: u64| {
            CONFIG
                .save(
                    deps.storage,
                    &Config {
                        name_nft: Addr::unchecked("namenftaddr"),
                        verifier_pubkeys: vec![verifier1(), verifier2(), verifier3()]
                            .iter()
                            .map(|sk| sk.to_binary())
                            .collect(),
                        verification_threshold_percentage: Decimal::percent(pct),
                        fee: Some(Coin::new(100000, "uosmo")),
                    },
                )
                .unwrap();
        };

        let sign_all = |verfiers: &[SigningKey], msg: &str| {
            verfiers
                .iter()
                .map(|v| {
                    (
                        v.public_key().to_bytes(),
                        v.sign(msg.as_bytes()).unwrap().to_vec(),
                    )
                })
                .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
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
            ContractError::NotAVerifierPublicKey {
                public_key: Binary(non_verifier().public_key().to_bytes())
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
        assert_eq!(err, ContractError::InvalidSignature {});

        // test 1/3 valid signature (one duplicated signature) < 51% should error
        let mut deps = mock_dependencies();
        set_threshold_pct(deps.as_mut(), 51);
        let verifications = sign_all(&[verifier3(), verifier3()], msg);
        let err =
            check_verification_pass_threshold(deps.as_ref(), msg, &verifications).unwrap_err();
        assert_eq!(
            err,
            ContractError::DuplicatedVerification {
                signature: Binary(verifications[0].1.clone())
            }
        );

        // test 2/4 valid signature = 50% should pass
        let mut deps = mock_dependencies();
        CONFIG
            .save(
                &mut deps.storage,
                &Config {
                    name_nft: Addr::unchecked("namenftaddr"),
                    verifier_pubkeys: vec![verifier1(), verifier2(), verifier3(), verifier4()]
                        .iter()
                        .map(|sk| Binary(sk.public_key().to_bytes()))
                        .collect(),
                    verification_threshold_percentage: Decimal::percent(50),
                    fee: Some(Coin::new(100000, "uosmo")),
                },
            )
            .unwrap();
        check_verification_pass_threshold(
            deps.as_ref(),
            msg,
            &sign_all(&[verifier1(), verifier4()], msg),
        )
        .unwrap();

        // test no verifier set should error
        let mut deps = mock_dependencies();
        CONFIG
            .save(
                &mut deps.storage,
                &Config {
                    name_nft: Addr::unchecked("namenftaddr"),
                    verifier_pubkeys: vec![],
                    verification_threshold_percentage: Decimal::percent(50),
                    fee: Some(Coin::new(100000, "uosmo")),
                },
            )
            .unwrap();

        let err =
            check_verification_pass_threshold(deps.as_ref(), msg, &sign_all(&[], msg)).unwrap_err();
        assert_eq!(err, ContractError::NoVerifier {});
    }
}
