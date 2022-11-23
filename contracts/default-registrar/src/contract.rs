#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, WasmMsg,
    WasmQuery,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

use registry::msg::{
    ExecuteMsg as RegistryExecuteMsg, IsAdminResponse, QueryMsg as QueryMsgRegistry,
};
use resolver::msg::ExecuteMsg as ResolverExecuteMsg;

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

    let registry_addr = deps.api.addr_validate(&msg.registry)?;
    let resolver_addr = deps.api.addr_validate(&msg.resolver)?;

    let mut verifier_addrs = Vec::new();
    for verifier in msg.verifier_addrs {
        let verifier_addr = deps.api.addr_validate(&verifier)?;
        verifier_addrs.push(verifier_addr);
    }

    let config = Config {
        registry: registry_addr,
        resolver: resolver_addr,
        verifiers: verifier_addrs,
    };

    CONFIG.save(deps.storage, &config)?;

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
            user_name,
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

// execute_register calls two contracts: the registry and the resolver
// the registry contract is called to mint nft of the name service, then save the resolver address for the user_name
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

    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    let is_verifier = is_verifier(deps.as_ref(), info.sender.clone());

    // if the sender is neither a registrar nor an admin, return error
    if !is_admin && !is_verifier {
        return Err(ContractError::Unauthorized {});
    }

    // do a sanity check on the user name
    // they cannot use "." in the user name
    if user_name.contains(".") {
        return Err(ContractError::InvalidUserName { user_name });
    }

    // call resolver and set given addresses
    let set_addresses_msg = WasmMsg::Execute {
        contract_addr: config.registry.to_string(),
        msg: to_binary(&ResolverExecuteMsg::SetRecord {
            user_name: user_name.clone(),
            addresses,
        })?,
        funds: vec![],
    };

    let resolver_addr = deps.api.addr_validate(&resolver)?;
    // call registry and set resolver address
    let set_resolver_msg = WasmMsg::Execute {
        contract_addr: config.registry.to_string(),
        msg: to_binary(&RegistryExecuteMsg::SetResolverAddress {
            user_name: user_name.clone(),
            resolver_address: resolver_addr,
        })?,
        funds: vec![],
    };

    // TODO: add message call for minting nft

    Ok(Response::new()
        .add_message(set_addresses_msg)
        .add_message(set_resolver_msg))
}

// execute_add_verifier adds an verifier to the list of verifiers
pub fn execute_add_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_addr: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    if !is_admin {
        return Err(ContractError::Unauthorized {});
    }

    let verifier_addr = deps.api.addr_validate(&verifier_addr)?;

    // check that the verifier is not already in the list of verifiers
    if config.verifiers.contains(&verifier_addr) {
        return Err(ContractError::VerifierAlreadyExists {});
    }

    let mut verifiers = config.verifiers;
    verifiers.push(verifier_addr);

    let config = Config {
        registry: config.registry,
        resolver: config.resolver,
        verifiers,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "add_verifier"))
}

pub fn execute_remove_verifier(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    verifier_addr: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    if !is_admin {
        return Err(ContractError::Unauthorized {});
    }

    let verifier_addr = deps.api.addr_validate(&verifier_addr)?;

    // check that the verifier is in the list of verifiers
    if !config.verifiers.contains(&verifier_addr) {
        return Err(ContractError::VerifierDoesNotExist {});
    }

    let mut verifiers = config.verifiers;
    verifiers.retain(|addr| addr != &verifier_addr);

    let config = Config {
        registry: config.registry,
        resolver: config.resolver,
        verifiers,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "remove_verifier")
        .add_attribute("verifier", verifier_addr))
}

pub fn is_admin(deps: Deps, address: String) -> Result<bool, ContractError> {
    let response = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: CONFIG.load(deps.storage)?.registry.to_string(),
            msg: to_binary(&QueryMsgRegistry::IsAdmin { address })?,
        }))
        .map(|res| from_binary(&res).unwrap());

    // TODO: come back and decide and change how we handle the contract error here
    match response {
        Ok(IsAdminResponse { is_admin }) => Ok(is_admin),
        Err(_) => Ok(false),
    }
}

pub fn is_verifier(deps: Deps, addr: Addr) -> bool {
    let config = CONFIG.load(deps.storage).unwrap();
    config.verifiers.iter().any(|a| a.as_ref() == addr.as_ref())
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

    fn mock_init(deps: DepsMut, registry: String, resolver: String, verifier_addrs: Vec<String>) {
        let msg = InstantiateMsg {
            registry,
            resolver,
            verifier_addrs,
        };

        let info = mock_info("creator", &coins(1, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }
}
