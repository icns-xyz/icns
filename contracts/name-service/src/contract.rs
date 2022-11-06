use std::collections::BinaryHeap;

use cosmwasm_std::{
    coin, entry_point, to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse, ReverseResolveRecordResponse,
};
use crate::state::{
    config, config_read, resolver, resolver_read,
    Resolver, Config
};

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config_state = Config {
        admins: msg.admins,
    };

    config(deps.storage).save(&config_state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetResolver { name, resolver_addr } => execute_set_resolver(deps, env, info, name, resolver_addr),
    }
}


pub fn execute_set_resolver(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    resolver_addr: String,
) -> Result<Response, ContractError> {
    let config_state = config(deps.storage).load()?;

    let key = name.as_bytes();

    let new_resolver = Resolver {
        resolver: resolver_addr,
    };

    if let Some(existing_record) = resolver(deps.storage).may_load(key)? {
        // name is already taken and expiry still not past
        return Err(ContractError::NameTaken { name });
    }

    // name is available
    resolver(deps.storage).save(key, &new_resolver)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetResolver { name } => query_resolver(deps, env, name),
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
    }
}

fn query_resolver(deps: Deps, env: Env, name: String) -> StdResult<Binary> {
    let key = name.as_bytes();

    let address = match resolver_read(deps.storage).may_load(key)? {
        Some(record) => {
            Some(String::from(&record.resolver))
        }
        None => None,
    };
    let resp = ResolveRecordResponse { address };

    to_binary(&resp)
}
