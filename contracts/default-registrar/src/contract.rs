#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_slice, to_binary, Binary, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;
use icns_name_nft::MintMsg;

use crate::checks::{
    check_send_from_admin, check_verification_pass_threshold, check_verifying_key,
};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, VerifyingMsg};

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
    let verifier_addrs = msg
        .verifier_addrs
        .into_iter()
        .map(|verifier_pubkey| {
            Ok(
                check_verifying_key(Binary::from_base64(&verifier_pubkey)?.as_slice())?
                    .to_bytes()
                    .to_vec(),
            )
        })
        .collect::<Result<_, ContractError>>()?;

    CONFIG.save(
        deps.storage,
        &Config {
            name_nft: name_nft_addr,
            verifiers: verifier_addrs,
            verification_threshold: msg.verification_threshold,
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
        } => execute_claim(deps, info, name, verifying_msg, verifications),
        ExecuteMsg::AddVerifier { verifier_addr } => {
            execute_add_verifier(deps, env, info, verifier_addr)
        }
        ExecuteMsg::RemoveVerifier { verifier_addr } => {
            execute_remove_verifier(deps, env, info, verifier_addr)
        }
    }
}

pub fn execute_claim(
    deps: DepsMut,
    info: MessageInfo,
    verifying_msg_str: String,
    name: String,
    verifications: Vec<String>,
) -> Result<Response, ContractError> {
    // check if verifying msg has matched name, claimer
    let verifying_msg: VerifyingMsg = from_slice(verifying_msg_str.as_bytes())?;
    if verifying_msg.name != name {
        return Err(ContractError::NameMismatched {});
    }
    if verifying_msg.claimer != info.sender {
        return Err(ContractError::ClaimerMismatched {});
    }

    // checks if verifcation pass threshold
    check_verification_pass_threshold(
        deps.as_ref(),
        &verifying_msg_str,
        &verifications
            .iter()
            .map(|v| Binary::from_base64(v).map(|b| b.to_vec()))
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

    let adding_verifier = Binary::from_base64(&verifier_pubkey)?;
    check_verifying_key(adding_verifier.as_slice())?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verifiers: vec![config.verifiers, vec![adding_verifier.to_vec()]].concat(),
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

    let removing_verifier = Binary::from_base64(&verifier_pubkey)?;
    check_verifying_key(removing_verifier.as_slice())?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verifiers: config
                .verifiers
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
