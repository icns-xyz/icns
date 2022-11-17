use cw2::set_contract_version;

use cosmwasm_std::Order::Ascending;
use cosmwasm_std::{
     to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, 
};


use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, GetOwnerResponse, GetAddressesResponse, GetAddressResponse, IsAdminResponse
};
use crate::state::{ Config,
     CONFIG, OWNER, ADDRESSES
};

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

    let mut registrar_addrs = Vec::new();
    for registrar in msg.registrar_addresses {
        registrar_addrs.push(deps.api.addr_validate(&registrar)?);
    }

    let cfg = Config {
        admins: admin_addrs,
        registrar_addresses: registrar_addrs,
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
    }
}

pub fn is_admin(deps: Deps, addr: Addr) -> bool {
    let cfg = CONFIG.load(deps.storage).unwrap();
    cfg.admins.iter().any(|a| a.as_ref() == addr.as_ref())
}

pub fn is_registrar(deps: Deps, addr: Addr) -> bool {
    let cfg = CONFIG.load(deps.storage).unwrap();
    cfg.registrar_addresses.iter().any(|a| a.as_ref() == addr.as_ref())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner { user_name } => to_binary(&query_owner(deps, env, user_name)?),
        QueryMsg::GetAddreses { user_name } => to_binary(&query_addresses(deps, env, user_name)?),
        QueryMsg::GetAddress { user_name, coin_type } => to_binary(&query_address(deps, env, user_name, coin_type)?),
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::IsAdmin { address } => {
            let addr = deps.api.addr_validate(&address)?;
            to_binary(&IsAdminResponse { is_admin: is_admin(deps, addr)})
        }
    }
}

fn query_owner(deps: Deps, _env: Env, name: String) -> StdResult<GetOwnerResponse> {
    let owner = OWNER.may_load(deps.storage, name)?;
    let resp = GetOwnerResponse { owner };

    Ok(resp)
}

fn query_addresses(deps: Deps, _env: Env, name: String) -> StdResult<GetAddressesResponse> {
    let addresses = ADDRESSES
        .prefix(name)
        .range(deps.storage, None, None, Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    let resp = GetAddressesResponse { addresses: addresses };

    Ok(resp)
}

fn query_address(deps: Deps, _env: Env, user_name: String, coin_type: i32) -> StdResult<GetAddressResponse> {
    let address = ADDRESSES.may_load(deps.storage, (user_name, coin_type))?;
    let resp = GetAddressResponse { address };

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, DepsMut, Addr, coins, from_binary};

    use crate::msg::{InstantiateMsg, IsAdminResponse};

    use super::*;

    fn mock_init(
        deps: DepsMut,
        admins: Vec<String>,
        registrar_addrs: Vec<String>,
    ) {
        let msg = InstantiateMsg {
            admins: admins,
            registrar_addresses: registrar_addrs,
        };

        let info = mock_info("creator", &coins(1, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
    }

    fn change_admin_string_to_vec(deps: DepsMut, admins: Vec<String>) -> Vec<Addr>{
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
        let registrar_addrs = vec![];
        mock_init(deps.as_mut(), admins, registrar_addrs);

        let registrar_addrs = vec![];

        let admins = vec![String::from("test_admin")];
        let exp = change_admin_string_to_vec(deps.as_mut(), admins);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        let expected = Config {
            admins: exp,
            registrar_addresses: registrar_addrs,
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_is_admin() {
        let mut deps = mock_dependencies();
        let admin1 = String::from("test_admin1");
        let admin2 = String::from("test_admin2");

        let admins = vec![admin1.clone(), admin2.clone()];
        let registrar_addrs = vec![];
        mock_init(deps.as_mut(), admins, registrar_addrs);

        // test valid admins
        let res = query(deps.as_ref(), mock_env(), QueryMsg::IsAdmin { address: admin1.clone() }).unwrap();
        let value: IsAdminResponse = from_binary(&res).unwrap();
        assert_eq!(value.is_admin, true);
        let res = query(deps.as_ref(), mock_env(), QueryMsg::IsAdmin { address: admin2.clone() }).unwrap();
        let value: IsAdminResponse = from_binary(&res).unwrap();
        assert_eq!(value.is_admin, true);

        // test invalid admin
        let res = query(deps.as_ref(), mock_env(), QueryMsg::IsAdmin { address: String::from("test_admin3") }).unwrap();
        let value: IsAdminResponse = from_binary(&res).unwrap();
        assert_eq!(value.is_admin, false);
    }

}
