#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, from_slice, to_binary, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use icns_name_nft::msg::Metadata;
use icns_name_nft::MintMsg;
use itertools::Itertools;

use crate::checks::{
    check_admin, check_existing_icns_name, check_fee, check_pubkey_length, check_valid_threshold,
    check_verfying_msg, check_verification_pass_threshold, is_admin,
};
use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, FeeResponse, InstantiateMsg, MigrateMsg, NameByTwitterIdResponse,
    NameNftAddressResponse, QueryMsg, ReferralCountResponse, Verification,
    VerificationThresholdResponse, VerifierPubKeysResponse, VerifyingMsg,
};

use crate::state::{Config, CONFIG, REFERRAL, UNIQUE_TWITTER_ID};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
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

    // validate name nft address
    let name_nft_addr = deps.api.addr_validate(&msg.name_nft_addr)?;

    // check each verififying key if there is invalid key
    msg.verifier_pubkeys
        .iter()
        .try_for_each(|pubkey| check_pubkey_length(pubkey).map(|_| ()))?;

    // check if threshold is valid (0.0-1.0)
    check_valid_threshold(&msg.verification_threshold)?;

    // save all configs
    CONFIG.save(
        deps.storage,
        &Config {
            name_nft: name_nft_addr,
            verifier_pubkeys: msg.verifier_pubkeys,
            verification_threshold_percentage: msg.verification_threshold,
            fee: msg.fee,
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
            execute_update_verifier_pubkeys(deps, info, add, remove)
        }
        ExecuteMsg::SetNameNftAddress { name_nft_address } => {
            execute_set_name_nft_address(deps, info, name_nft_address)
        }
        ExecuteMsg::SetMintingFee { minting_fee: fee } => execute_set_fee(deps, info, fee),
        ExecuteMsg::WithdrawFunds { amount, to_address } => {
            execute_withdraw_funds(deps, info, amount, to_address)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // No state migrations performed, just returned a Response
    Ok(Response::default())
}

// executes the claiming process for the icns name and nft.
pub fn execute_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    verifying_msg_str: String,
    verifications: Vec<Verification>,
    referral: Option<String>,
) -> Result<Response, ContractError> {
    let is_admin = is_admin(deps.as_ref(), &info.sender)?;

    // if not admin, need to pass check verification pass threshold before being able to claim name
    if !is_admin {
        check_verfying_msg(deps.as_ref(), &env, &info, &name, &verifying_msg_str)?;
        // Client creates `verfifying_msg` and send to verifiers to get verifications
        // with thier signatures. Then accumulates and send those signatures to this
        // contract via `Claim` message.
        // given verifications(signatures), check if the given verifications are valid and they pass the threshold set in config.
        // only the verifiers in the config can sign the verfifying_msg.
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
    }

    // if referral is set, check referral is an existing icns name
    if let Some(referral) = referral.as_ref() {
        check_existing_icns_name(deps.as_ref(), referral).map_err(|_| {
            ContractError::InvalidReferral {
                referral: referral.clone(),
            }
        })?;
    }

    // check if fees are correctly given.
    check_fee(deps.as_ref(), &info.funds)?;

    // add referral count if referral is set
    if let Some(referral) = referral.clone() {
        // initialize referral count to 1 if not exists
        REFERRAL.update(deps.storage, referral, |referral_count| -> StdResult<_> {
            Ok(referral_count.unwrap_or(0) + 1)
        })?;
    }

    
    // save unique_twitter_id to storage to prevent duplicate claim for single user.
    let verifying_msg: VerifyingMsg = from_slice(verifying_msg_str.as_bytes())?;
    UNIQUE_TWITTER_ID.save(deps.storage, verifying_msg.unique_twitter_id, &name)?;

    // mint name nft
    let config = CONFIG.load(deps.storage)?;

    // set minter of `icns-name-nft` to this contract
    // so that only this contract can mint name nft
    let mint_msg = WasmMsg::Execute {
        contract_addr: config.name_nft.to_string(),
        msg: to_binary(&icns_name_nft::ExecuteMsg::Mint(MintMsg {
            token_id: name.clone(),
            owner: info.sender.to_string(),
            token_uri: None,
            extension: Metadata { referral },
        }))?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_attribute("method", "claim")
        .add_attribute("name", name)
        .add_message(mint_msg))
}


fn execute_set_verification_threshold(
    deps: DepsMut,
    info: MessageInfo,
    verification_threshold: Decimal,
) -> Result<Response, ContractError> {
    // check if sender is admin. Only admin can set verification threshold
    check_admin(deps.as_ref(), &info.sender)?;

    let attrs = vec![
        attr("method", "set_verification_threshold"),
        attr("verfication_threshold", verification_threshold.to_string()),
    ];

    // sanity check on the given threshold
    check_valid_threshold(&verification_threshold)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verification_threshold_percentage: verification_threshold,
            ..config
        })
    })?;

    Ok(Response::new().add_attributes(attrs))
}

// execute_update_verifier_pubkeys updates the list of verifier pubkeys.
fn execute_update_verifier_pubkeys(
    deps: DepsMut,
    info: MessageInfo,
    add: Vec<Binary>,
    remove: Vec<Binary>,
) -> Result<Response, ContractError> {
    check_admin(deps.as_ref(), &info.sender)?;

    CONFIG.update(deps.storage, |config| -> Result<_, ContractError> {
        Ok(Config {
            verifier_pubkeys: vec![config.verifier_pubkeys, add]
                .concat()
                .into_iter()
                .filter(|v| !remove.contains(v))
                .unique()
                .map(|verifier_pubkey| {
                    check_pubkey_length(verifier_pubkey.as_slice())?;
                    Ok(verifier_pubkey)
                })
                .collect::<Result<_, ContractError>>()?,
            ..config
        })
    })?;

    Ok(Response::new().add_attribute("method", "update_verifier_pubkeys"))
}

// execute_withdraw_funds withdraws accumulated funds from fees.
fn execute_withdraw_funds(
    deps: DepsMut,
    info: MessageInfo,
    amount: Vec<Coin>,
    to_address: String,
) -> Result<Response, ContractError> {
    // check if the sender is admin. If not, return error.
    check_admin(deps.as_ref(), &info.sender)?;
    deps.api.addr_validate(&to_address)?;
    let attrs = vec![
        attr("method", "withraw_funds"),
        attr("to_address", &to_address),
        attr(
            "amount",
            amount
                .iter()
                .map(|amount| amount.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ),
    ];

    Ok(Response::new()
        .add_attributes(attrs)
        .add_message(BankMsg::Send { to_address, amount }))
}

// execute_set_fee sets the fee for claiming a name.
// This can either be a fixed amount or None.
fn execute_set_fee(
    deps: DepsMut,
    info: MessageInfo,
    fee: Option<Coin>,
) -> Result<Response, ContractError> {
    check_admin(deps.as_ref(), &info.sender)?;

    let attrs = vec![
        attr("method", "set_fee"),
        attr(
            "fee",
            fee.as_ref()
                .map(|fee| fee.to_string())
                .unwrap_or_else(|| "no fee".to_string()),
        ),
    ];

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config { fee, ..config })
    })?;

    Ok(Response::new().add_attributes(attrs))
}

// execute_set_name_nft_address sets the address of the name nft contract.
// This is used to mint name nft when a name is claimed.
fn execute_set_name_nft_address(
    deps: DepsMut,
    info: MessageInfo,
    name_nft_address: String,
) -> Result<Response, ContractError> {
    check_admin(deps.as_ref(), &info.sender)?;
    CONFIG.update(deps.storage, |config| -> Result<_, ContractError> {
        Ok(Config {
            name_nft: deps.api.addr_validate(&name_nft_address)?,
            ..config
        })
    })?;

    Ok(Response::new()
        .add_attribute("method", "set_name_nft_address")
        .add_attribute("name_nft_address", name_nft_address))
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

    check_admin(deps.as_ref(), &info.sender)?;
    let adding_verifier = verifier_pubkey;
    check_pubkey_length(&adding_verifier)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verifier_pubkeys: vec![config.verifier_pubkeys, vec![adding_verifier]].concat(),
            ..config
        })
    })?;

    Ok(Response::new().add_attributes(attrs))
}

// execute_remove_verifier removes an verifier from the list of verifiers
pub fn execute_remove_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_pubkey: Binary,
) -> Result<Response, ContractError> {
    check_admin(deps.as_ref(), &info.sender)?;
    let removing_verifier = verifier_pubkey.to_vec();
    check_pubkey_length(&removing_verifier)?;

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
        QueryMsg::NameNftAddress {} => to_binary(&NameNftAddressResponse {
            name_nft_address: CONFIG.load(deps.storage)?.name_nft.to_string(),
        }),
        QueryMsg::ReferralCount { name } => to_binary(&query_referral_count(deps, name)?),
        QueryMsg::Fee {} => to_binary(&query_fee(deps)?),
        QueryMsg::NameByTwitterId { twitter_id } => {
            to_binary(&query_name_by_twitter_id(deps, twitter_id)?)
        }
    }
}

fn query_name_by_twitter_id(deps: Deps, twitter_id: String) -> StdResult<NameByTwitterIdResponse> {
    Ok(NameByTwitterIdResponse {
        name: UNIQUE_TWITTER_ID.load(deps.storage, twitter_id)?,
    })
}

fn query_fee(deps: Deps) -> StdResult<FeeResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(FeeResponse { fee: config.fee })
}

fn query_referral_count(deps: Deps, name: String) -> StdResult<ReferralCountResponse> {
    let count = REFERRAL.may_load(deps.storage, name)?;
    Ok(ReferralCountResponse {
        count: count.unwrap_or_default(),
    })
}
