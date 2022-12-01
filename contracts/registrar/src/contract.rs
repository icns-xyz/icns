#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, WasmMsg,
};
use cw2::set_contract_version;
use icns_name_nft::MintMsg;
use itertools::Itertools;

use crate::checks::{
    check_send_from_admin, check_valid_threshold, check_verfying_msg,
    check_verification_pass_threshold, check_verifying_key,
};
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, NameNFTAddressResponse, QueryMsg, Verification,
    VerificationThresholdResponse, VerifierPubKeysResponse,
};

use icns_name_nft::msg::ExecuteMsg as NameNFTExecuteMsg;

use crate::state::{Config, CONFIG, REFERRAL};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:icns-registrar";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let name_nft_addr = deps.api.addr_validate(&msg.name_nft_addr)?;
    let verifier_pubkeys = msg
        .verifier_pubkeys
        .into_iter()
        .map(|pubkey| {
            let pubkey_bytes = pubkey.to_vec();
            check_verifying_key(&pubkey_bytes)?;

            Ok(pubkey)
        })
        .collect::<Result<_, ContractError>>()?;

    check_valid_threshold(&msg.verification_threshold)?;
    CONFIG.save(
        deps.storage,
        &Config {
            name_nft: name_nft_addr,
            verifier_pubkeys,
            verification_threshold_percentage: msg.verification_threshold,
        },
    )?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Claim {
            name,
            verifying_msg,
            verifications,
            referral,
        } => execute_claim(
            deps,
            env,
            info,
            name,
            verifying_msg,
            verifications,
            referral,
        ),
        ExecuteMsg::SetVerificationThreshold { threshold } => {
            execute_set_verification_threshold(deps, info, threshold)
        }
        ExecuteMsg::UpdateVerifierPubkeys { add, remove } => {
            check_send_from_admin(deps.as_ref(), &info.sender)?;

            CONFIG.update(deps.storage, |config| -> Result<_, ContractError> {
                Ok(Config {
                    verifier_pubkeys: vec![config.verifier_pubkeys, add]
                        .concat()
                        .into_iter()
                        .filter(|v| !remove.contains(v))
                        .unique()
                        .map(|verifier_pubkey| {
                            check_verifying_key(verifier_pubkey.as_slice())?;
                            Ok(verifier_pubkey)
                        })
                        .collect::<Result<_, ContractError>>()?,
                    ..config
                })
            })?;

            Ok(Response::new().add_attribute("method", "update_verifier_pubkeys"))
        }
        ExecuteMsg::SetNameNFTAddress { name_nft_address } => {
            check_send_from_admin(deps.as_ref(), &info.sender)?;
            CONFIG.update(deps.storage, |config| -> Result<_, ContractError> {
                Ok(Config {
                    name_nft: deps.api.addr_validate(&name_nft_address)?,
                    ..config
                })
            })?;

            Ok(Response::new())
        }
    }
}

fn execute_set_verification_threshold(
    deps: DepsMut,
    info: MessageInfo,
    verification_threshold: Decimal,
) -> Result<Response, ContractError> {
    check_send_from_admin(deps.as_ref(), &info.sender)?;

    let attrs = vec![
        attr("method", "set_verification_threshold"),
        attr("verfication_threshold", verification_threshold.to_string()),
    ];

    check_valid_threshold(&verification_threshold)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verification_threshold_percentage: verification_threshold,
            ..config
        })
    })?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn execute_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    verifying_msg_str: String,
    verifications: Vec<Verification>,
    referral: Option<String>,
) -> Result<Response, ContractError> {
    check_verfying_msg(&env, &info, &name, &verifying_msg_str)?;
    check_verification_pass_threshold(
        deps.as_ref(),
        &verifying_msg_str,
        &verifications
            .iter()
            .map(|verification| {
                Ok((
                    verification.public_key.to_vec(),
                    verification.signature.to_vec(),
                ))
            })
            .collect::<StdResult<Vec<_>>>()?,
    )?;

    // add referral count if referral is set
    if let Some(referral) = referral {
        // initialize referral count to 1 if not exists
        let referral_count = REFERRAL.may_load(deps.storage, referral.to_string())?;
        match referral_count {
            Some(count) => {
                REFERRAL.save(deps.storage, referral, &(count + 1))?;
            }
            None => {
                REFERRAL.save(deps.storage, referral, &1)?;
            }
        }
    }

    // mint name nft
    let config = CONFIG.load(deps.storage)?;
    let mint_msg = WasmMsg::Execute {
        contract_addr: config.name_nft.to_string(),
        msg: to_binary(&NameNFTExecuteMsg::CW721Base(
            icns_name_nft::CW721BaseExecuteMsg::Mint(MintMsg {
                token_id: name.clone(),
                owner: info.sender.to_string(),
                token_uri: None,
                extension: None,
            }),
        ))?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_attribute("method", "claim")
        .add_attribute("name", name)
        .add_message(mint_msg))
}

// execute_add_verifier adds an verifier to the list of verifiers
pub fn execute_add_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_pubkey: Binary,
) -> Result<Response, ContractError> {
    let attrs = vec![
        attr("method", "add_verifier"),
        attr("verifier", verifier_pubkey.to_base64()),
    ];

    check_send_from_admin(deps.as_ref(), &info.sender)?;
    let adding_verifier = verifier_pubkey;
    check_verifying_key(&adding_verifier)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verifier_pubkeys: vec![config.verifier_pubkeys, vec![adding_verifier]].concat(),
            ..config
        })
    })?;

    Ok(Response::new().add_attributes(attrs))
}

pub fn execute_remove_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_pubkey: Binary,
) -> Result<Response, ContractError> {
    check_send_from_admin(deps.as_ref(), &info.sender)?;
    let removing_verifier = verifier_pubkey.to_vec();
    check_verifying_key(&removing_verifier)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verifier_pubkeys: config
                .verifier_pubkeys
                .into_iter()
                .filter(|v| *v != removing_verifier)
                .collect(),
            ..config
        })
    })?;

    Ok(Response::new()
        .add_attribute("method", "remove_verifier")
        .add_attribute("verifier", verifier_pubkey.to_base64()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, StdError> {
    match msg {
        QueryMsg::VerifierPubKeys {} => to_binary(&VerifierPubKeysResponse {
            verifier_pubkeys: CONFIG.load(deps.storage)?.verifier_pubkeys,
        }),
        QueryMsg::VerificationThreshold {} => to_binary(&VerificationThresholdResponse {
            verification_threshold_percentage: CONFIG
                .load(deps.storage)?
                .verification_threshold_percentage,
        }),
        QueryMsg::NameNFTAddress {} => to_binary(&NameNFTAddressResponse {
            name_nft_address: CONFIG.load(deps.storage)?.name_nft.to_string(),
        }),
    }
}
