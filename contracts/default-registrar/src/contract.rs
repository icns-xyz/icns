#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use icns_name_nft::MintMsg;

use crate::checks::{
    check_send_from_admin, check_valid_threshold, check_verfying_msg,
    check_verification_pass_threshold, check_verifying_key,
};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Verification};

use icns_name_nft::msg::ExecuteMsg as NameNFTExecuteMsg;

use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:default-registrar";
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
        .map(|pubkey| base64_sec1_pubkey_to_bytes(&pubkey))
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
        } => execute_claim(deps, env, info, name, verifying_msg, verifications),
        ExecuteMsg::AddVerifier { verifier_addr } => {
            execute_add_verifier(deps, env, info, verifier_addr)
        }
        ExecuteMsg::RemoveVerifier { verifier_addr } => {
            execute_remove_verifier(deps, env, info, verifier_addr)
        }
        ExecuteMsg::SetVerificationThreshold { threshold } => {
            execute_set_verification_threshold(deps, info, threshold)
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
) -> Result<Response, ContractError> {
    check_verfying_msg(&env, &info, &name, &verifying_msg_str)?;
    check_verification_pass_threshold(
        deps.as_ref(),
        &verifying_msg_str,
        &verifications
            .iter()
            .map(|verification| {
                Ok((
                    Binary::from_base64(&verification.public_key).map(|binary| binary.to_vec())?,
                    Binary::from_base64(&verification.signature).map(|binary| binary.to_vec())?,
                ))
            })
            .collect::<StdResult<Vec<_>>>()?,
    )?;

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
    verifier_pubkey: String,
) -> Result<Response, ContractError> {
    check_send_from_admin(deps.as_ref(), &info.sender)?;
    let adding_verifier = base64_sec1_pubkey_to_bytes(&verifier_pubkey)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verifier_pubkeys: vec![config.verifier_pubkeys, vec![adding_verifier.to_vec()]]
                .concat(),
            ..config
        })
    })?;

    Ok(Response::new()
        .add_attribute("method", "add_verifier")
        .add_attribute("verifier", verifier_pubkey))
}

pub fn execute_remove_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_pubkey: String,
) -> Result<Response, ContractError> {
    check_send_from_admin(deps.as_ref(), &info.sender)?;
    let removing_verifier = base64_sec1_pubkey_to_bytes(&verifier_pubkey)?;

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
        .add_attribute("verifier", verifier_pubkey))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msgg: QueryMsg) -> Result<Binary, ContractError> {
    unimplemented!()
}

fn base64_sec1_pubkey_to_bytes(pubkey: &str) -> Result<Vec<u8>, ContractError> {
    Ok(
        check_verifying_key(Binary::from_base64(pubkey)?.as_slice())?
            .to_bytes()
            .to_vec(),
    )
}

// #[cfg(test)]
// mod tests {
//     use cosmwasm_std::{
//         coins, from_binary,
//         testing::{mock_dependencies, mock_env, mock_info},
//         Addr, Coin, DepsMut,
//     };

//     use crate::msg::InstantiateMsg;

//     use super::*;

//     fn mock_init(deps: DepsMut, name_nft: String, resolver: String, verifier_addrs: Vec<String>) {
//         let msg = InstantiateMsg {
//             name_nft_addr: name_nft,
//             verifier_addrs,
//         };

//         let info = mock_info("creator", &coins(1, "token"));
//         let _res = instantiate(deps, mock_env(), info, msg)
//             .expect("contract successfully handles InstantiateMsg");
//     }
// }
