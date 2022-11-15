#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary,  DepsMut, Env, MessageInfo, Response, Addr, WasmMsg};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};

use name_service::msg::{ExecuteMsg as NameServiceExecuteMsg};

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
    let admin_addr = deps.api.addr_validate(&msg.admin_addr)?;

    let config = Config {
        registry: registry_addr,
        admin: admin_addr,
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
        ExecuteMsg::Register { user_name, owner, addresses } => execute_register(deps, env, info, user_name, owner, addresses)
    }
}

pub fn execute_register(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_name: String,
    owner: Addr,
    addresses: Vec<(i32, String)>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    let set_record_msg = WasmMsg::Execute {
        contract_addr: config.registry.to_string(),
        msg: to_binary(&NameServiceExecuteMsg::SetRecord {
            user_name,
            owner,
            addresses,
        })?,
        funds: vec![],
    };
    
    Ok(Response::new()
        .add_message(set_record_msg)
    )
}
