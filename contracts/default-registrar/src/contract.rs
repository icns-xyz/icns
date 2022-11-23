#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg};
use cw2::set_contract_version;

use crate::checks::check_send_from_admin;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

use icns_name_nft::msg::{ QueryMsg as QueryMsgNameNft, IsAdminResponse};
use resolver::msg::{ExecuteMsg as ResolverExecuteMsg};

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
        .map(|addr| deps.api.addr_validate(&addr))
        .collect::<StdResult<_>>()?;

    CONFIG.save(
        deps.storage,
        &Config {
            name_nft: name_nft_addr,
            verifiers: verifier_addrs,
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
        ExecuteMsg::Register {
            name: user_name,
            owner,
            addresses,
        } => execute_register(deps, env, info, user_name, owner, addresses),
        ExecuteMsg::AddVerifier { verifier_addr } => {
            execute_add_verifier(deps, env, info, verifier_addr)
        }
        ExecuteMsg::RemoveVerifier { verifier_addr } => {
            execute_remove_verifier(deps, env, info, verifier_addr)
        }
    }
}

// execute_register calls two contracts: the name_nft and the resolver
// the name_nft contract is called to mint nft of the name service, then save the resolver address for the user_name
// the resolver contract is called to save the addresses for the user_name
pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_name: String,
    resolver: String,
    addresses: Vec<(String, String)>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // do a sanity check on the user name
    // they cannot use "." in the user name
    if user_name.contains('.') {
        return Err(ContractError::InvalidUserName { user_name });
    }

    // call resolver and set given addresses
    let set_record_msg = WasmMsg::Execute {
        contract_addr: config.name_nft.to_string(),
        msg: to_binary(&ResolverExecuteMsg::SetRecord {
            user_name,
            addresses,
        })?,
        funds: vec![],
    };

    Ok(Response::new().add_message(set_record_msg))
}

// execute_add_verifier adds an verifier to the list of verifiers
pub fn execute_add_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_addr: String,
) -> Result<Response, ContractError> {
    check_send_from_admin(deps.as_ref(), &info.sender)?;
    let adding_verifier = deps.api.addr_validate(&verifier_addr)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            verifiers: vec![config.verifiers, vec![adding_verifier]].concat(),
            ..config
        })
    })?;

    Ok(Response::new().add_attribute("method", "add_verifier"))
}

pub fn execute_remove_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_addr: String,
) -> Result<Response, ContractError> {
    check_send_from_admin(deps.as_ref(), &info.sender)?;
    let removing_verifier = deps.api.addr_validate(&verifier_addr)?;

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
        .add_attribute("verifier", verifier_addr))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Coin, DepsMut,
    };

    use crate::msg::InstantiateMsg;

    use super::*;

    fn mock_init(deps: DepsMut, name_nft: String, resolver: String, verifier_addrs: Vec<String>) {
        let msg = InstantiateMsg {
            name_nft_addr: name_nft,
            verifier_addrs,
        };

        let info = mock_info("creator", &coins(1, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }
}
