#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary,  DepsMut, Env, MessageInfo, Response, Addr, WasmMsg, Deps, QueryRequest, from_binary, WasmQuery};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

use registry::msg::{ExecuteMsg as RegistryExecuteMsg, QueryMsg as QueryMsgRegistry, IsAdminResponse};
use resolver::msg::{ExecuteMsg as ResolverExecuteMsg};

use crate::state::{ Config,
    CONFIG,
};

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

    let mut operator_addrs = Vec::new();
    for operator in msg.operator_addrs {
        let operator_addr = deps.api.addr_validate(&operator)?;
        operator_addrs.push(operator_addr);
    }

    let config = Config {
        registry: registry_addr,
        resolver: resolver_addr,
        operators: operator_addrs,
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
        ExecuteMsg::Register { user_name, owner, addresses } => execute_register(deps, env, info, user_name, owner, addresses),
        ExecuteMsg::AddOperator { operator_addr } => execute_add_operator(deps, env, info, operator_addr),
        ExecuteMsg::RemoveOperator { operator_addr } => execute_remove_operator(deps, env, info, operator_addr),
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
    let is_operator = is_operator(deps.as_ref(), info.sender.clone());
    
     // if the sender is neither a registrar nor an admin, return error
     if !is_admin && !is_operator {
        return Err(ContractError::Unauthorized {});
    }

    // do a sanity check on the user name
    // they cannot use "." in the user name
    if user_name.contains(".") {
        return Err(ContractError::InvalidUserName {user_name});
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
        .add_message(set_resolver_msg)
    )
}

// execute_add_operator adds an operator to the list of operators
pub fn execute_add_operator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    operator_addr: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    if !is_admin {
        return Err(ContractError::Unauthorized {});
    }

    let operator_addr = deps.api.addr_validate(&operator_addr)?;

    // check that the operator is not already in the list of operators
    if config.operators.contains(&operator_addr) {
        return Err(ContractError::OperatorAlreadyExists {});
    }

    let mut operators = config.operators;
    operators.push(operator_addr);

    let config = Config {
        registry: config.registry,
        resolver: config.resolver,
        operators,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "add_operator")
    )
}

pub fn execute_remove_operator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    operator_addr: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    if !is_admin {
        return Err(ContractError::Unauthorized {});
    }

    let operator_addr = deps.api.addr_validate(&operator_addr)?;

    // check that the operator is in the list of operators
    if !config.operators.contains(&operator_addr) {
        return Err(ContractError::OperatorDoesNotExist {});
    }

    let mut operators = config.operators;
    operators.retain(|addr| addr != &operator_addr);

    let config = Config {
        registry: config.registry,
        resolver: config.resolver,
        operators,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "remove_operator")
        .add_attribute("operator", operator_addr)
    )
}

pub fn is_admin(deps: Deps, address: String) -> Result<bool, ContractError> {
    let response = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CONFIG.load(deps.storage)?.registry.to_string(),
        msg: to_binary(&QueryMsgRegistry::IsAdmin {address})?,
    })).map(|res| from_binary(&res).unwrap());
 
    // TODO: come back and decide and change how we handle the contract error here
     match response {
          Ok(IsAdminResponse {is_admin}) => Ok(is_admin),
          Err(_) => Ok(false),
     }
 }

pub fn is_operator(deps: Deps, addr: Addr) -> bool {
    let config = CONFIG.load(deps.storage).unwrap();
    config.operators.iter().any(|a| a.as_ref() == addr.as_ref())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, DepsMut, Addr, coins, from_binary, Coin};

    use crate::msg::{InstantiateMsg};

    use super::*;

    fn mock_init(
        deps: DepsMut,
        registry: String,
        resolver: String,
        operator_addrs: Vec<String>,
    ) {
        let msg = InstantiateMsg {
            registry,
            resolver,
            operator_addrs,
        };

        let info = mock_info("creator", &coins(1, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
    }
}
