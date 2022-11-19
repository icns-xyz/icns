#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::set_contract_version;

use cosmwasm_std::Order::Ascending;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, GetAddressResponse, GetAddressesResponse, GetResolverAddrResponse, InstantiateMsg,
    IsAdminResponse, QueryMsg,
};
use crate::state::{Config, ADDRESSES, CONFIG, RESOLVER};

const CONTRACT_NAME: &str = "crates.io:default-registrar";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO: add duplication check
    let mut admin_addrs = Vec::new();
    for admin in msg.admins {
        admin_addrs.push(deps.api.addr_validate(&admin)?);
    }

    let registrar_address = deps.api.addr_validate(&msg.registrar_address)?;
    let name_address = deps.api.addr_validate(&msg.name_address)?;

    let cfg = Config {
        admins: admin_addrs,
        registrar_address,
        name_address,
    };
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetResolverAddress {
            user_name,
            resolver_address,
        } => execute_set_resolver_address(_deps, _env, _info, user_name, resolver_address),
    }
}

pub fn execute_set_resolver_address(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_name: String,
    resolver_addr: Addr,
) -> Result<Response, ContractError> {
    let is_admin = is_admin(deps.as_ref(), info.sender.clone());
    let is_registrar = is_registrar(deps.as_ref(), info.sender);

    // if not admin and if not registrar, return err
    if !is_admin && !is_registrar {
        return Err(ContractError::Unauthorized {});
    }

    // check if the user_name is already registered
    // TODO: make only admin be able to override, error if registrar
    let existing = RESOLVER.may_load(deps.storage, user_name.clone())?;
    if existing.is_some() {
        return Err(ContractError::UserAlreadyRegistered { name: user_name });
    }

    RESOLVER.save(deps.storage, user_name, &resolver_addr)?;

    Ok(Response::default())
}

pub fn is_admin(deps: Deps, addr: Addr) -> bool {
    let cfg = CONFIG.load(deps.storage).unwrap();
    cfg.admins.iter().any(|a| a.as_ref() == addr.as_ref())
}

pub fn is_registrar(deps: Deps, addr: Addr) -> bool {
    let cfg = CONFIG.load(deps.storage).unwrap();
    cfg.registrar_address.as_ref() == addr.as_ref()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetResolverAddr { user_name } => {
            to_binary(&query_resolver_addr(deps, env, user_name)?)
        }
        QueryMsg::GetAddreses { user_name } => to_binary(&query_addresses(deps, env, user_name)?),
        QueryMsg::GetAddress {
            user_name,
            coin_type,
        } => to_binary(&query_address(deps, env, user_name, coin_type)?),
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::IsAdmin { address } => {
            let addr = deps.api.addr_validate(&address)?;
            to_binary(&IsAdminResponse {
                is_admin: is_admin(deps, addr),
            })
        }
    }
}

fn query_resolver_addr(deps: Deps, _env: Env, name: String) -> StdResult<GetResolverAddrResponse> {
    let resolver_addr = RESOLVER.may_load(deps.storage, name)?;
    let resp = GetResolverAddrResponse { resolver_addr };

    Ok(resp)
}

fn query_addresses(deps: Deps, _env: Env, name: String) -> StdResult<GetAddressesResponse> {
    let addresses = ADDRESSES
        .prefix(name)
        .range(deps.storage, None, None, Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    let resp = GetAddressesResponse { addresses };

    Ok(resp)
}

fn query_address(
    deps: Deps,
    _env: Env,
    user_name: String,
    coin_type: i32,
) -> StdResult<GetAddressResponse> {
    let address = ADDRESSES.may_load(deps.storage, (user_name, coin_type))?;
    let resp = GetAddressResponse { address };

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins, from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr, Api, DepsMut,
    };

    use crate::msg::{InstantiateMsg, IsAdminResponse};

    use super::*;

    fn mock_init(deps: DepsMut, admins: Vec<String>, registrar_addr: String, name_addr: String) {
        let msg = InstantiateMsg {
            admins,
            registrar_address: registrar_addr,
            name_address: name_addr,
        };

        let info = mock_info("creator", &coins(1, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }

    fn change_admin_string_to_vec(deps: DepsMut, admins: Vec<String>) -> Vec<Addr> {
        let mut admin_addr = Vec::new();
        for admin in admins {
            admin_addr.push(deps.api.addr_validate(&admin).unwrap());
        }
        admin_addr
    }

    #[test]
    fn proper_instantiate() {
        let mut deps = mock_dependencies();

        let admins = vec![String::from("test_admin")];
        let registrar_addr_string = String::from("test_registrar");
        let name_addr_string = String::from("name");
        mock_init(
            deps.as_mut(),
            admins,
            registrar_addr_string.clone(),
            name_addr_string.clone(),
        );

        let admins = vec![String::from("test_admin")];
        let exp = change_admin_string_to_vec(deps.as_mut(), admins);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();

        let registrar_addr = deps.api.addr_validate(&registrar_addr_string).unwrap();
        let name_addr = deps.api.addr_validate(&name_addr_string).unwrap();

        let expected = Config {
            admins: exp,
            registrar_address: registrar_addr,
            name_address: name_addr,
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_is_admin() {
        let mut deps = mock_dependencies();
        let admin1 = String::from("test_admin1");
        let admin2 = String::from("test_admin2");

        let admins = vec![admin1.clone(), admin2.clone()];
        let registrar_addr = String::from("test_registrar");
        let name_addr = String::from("name");
        mock_init(deps.as_mut(), admins, registrar_addr, name_addr);

        // test valid admins
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IsAdmin { address: admin1 },
        )
        .unwrap();
        let value: IsAdminResponse = from_binary(&res).unwrap();
        assert!(value.is_admin);
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IsAdmin { address: admin2 },
        )
        .unwrap();
        let value: IsAdminResponse = from_binary(&res).unwrap();
        assert!(value.is_admin);

        // test invalid admin
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::IsAdmin {
                address: String::from("test_admin3"),
            },
        )
        .unwrap();
        let value: IsAdminResponse = from_binary(&res).unwrap();
        assert!(!value.is_admin);
    }

    #[test]
    fn test_registrar_addr() {
        let mut deps = mock_dependencies();

        let admins = vec![String::from("test_admin")];
        let registrar_addr = String::from("test_registrar");
        let name_addr = String::from("name");
        mock_init(deps.as_mut(), admins, registrar_addr.clone(), name_addr);

        let resolver = String::from("resolver");
        let non_resolver = String::from("non_resolver");
        let user_name = String::from("test_user");

        // try setting resolver with non-resolver and non admin, it should error
        let info = mock_info(&non_resolver, &coins(1, "token"));
        let msg = ExecuteMsg::SetResolverAddress {
            user_name: user_name.clone(),
            resolver_address: Addr::unchecked(resolver.clone()),
        };
        let res_is_error = execute(deps.as_mut(), mock_env(), info, msg).is_err();
        assert!(res_is_error);

        // first try setting and getting a resolver for a user
        let info = mock_info(&registrar_addr, &coins(1, "token"));
        let msg = ExecuteMsg::SetResolverAddress {
            user_name: user_name.clone(),
            resolver_address: Addr::unchecked(resolver.clone()),
        };

        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let query_res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetResolverAddr {
                user_name: user_name.clone(),
            },
        )
        .unwrap();
        let value: GetResolverAddrResponse = from_binary(&query_res).unwrap();
        assert_eq!(
            value.resolver_addr.unwrap(),
            Addr::unchecked(resolver.clone())
        );

        // now try setting resolver for an existing user, it should error
        let info = mock_info(&registrar_addr, &coins(1, "token"));
        let msg = ExecuteMsg::SetResolverAddress {
            user_name,
            resolver_address: Addr::unchecked(resolver),
        };
        let res_is_error = execute(deps.as_mut(), mock_env(), info, msg).is_err();
        assert!(res_is_error);

        // try querying for a user that does not exist
        let query_res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetResolverAddr {
                user_name: String::from("invalid user"),
            },
        )
        .unwrap();
        let value: GetResolverAddrResponse = from_binary(&query_res).unwrap();
        assert_eq!(value.resolver_addr, None);
    }
}
