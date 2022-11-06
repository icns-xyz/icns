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
    let mut admin_addr = Vec::new();
    for admin in msg.admins {
        admin_addr.push(deps.api.addr_validate(&admin)?);
    }

    let config_state = Config {
        admins: admin_addr,
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


#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, DepsMut, Addr, coins, from_binary};

    use crate::msg::InstantiateMsg;

    use super::*;

    fn mock_init_with_admin(
        deps: DepsMut,
        admins: Vec<String>,
    ) {
        let msg = InstantiateMsg {
            admins: admins,
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
    }

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value, expected);
    }

    fn get_name_owner(deps: Deps, name: &str) -> String {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::GetResolver  {
                name: name.to_string(),
            },
        )
        .unwrap();

        let value: ResolveRecordResponse = from_binary(&res).unwrap();
        value.address.unwrap()
    }

    fn change_admin_string_to_vec(deps: DepsMut, admins: Vec<String>) -> Vec<Addr>{
        let mut admin_addr = Vec::new();
        for admin in admins {
            admin_addr.push(deps.api.addr_validate(&admin).unwrap());
        }
        admin_addr
    }

    fn mock_register_resolver_for_alice(deps: DepsMut, sent: &[Coin], resolver: String) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::SetResolver {
            name: "alice".to_string(),
            resolver_addr: resolver.to_string(),
        };
        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract successfully handles Register message");
    }

    #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        let admins = vec![String::from("test_admin")];
        mock_init_with_admin(deps.as_mut(), admins);

        let admins = vec![String::from("test_admin")];
        let exp = change_admin_string_to_vec(deps.as_mut(), admins);

        assert_config_state(
            deps.as_ref(),
            Config { admins: exp }
        );

        mock_register_resolver_for_alice(deps.as_mut(), &coins(2, "token"), String::from("test_resolver"));

        let registered_resolver = get_name_owner(deps.as_ref(), "alice");

        assert_ne!(registered_resolver, String::from("invalid_resolvera"));
        assert_eq!(registered_resolver, String::from("test_resolver"));
    }
}
