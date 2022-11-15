use cw2::set_contract_version;

use cosmwasm_std::Order::Ascending;
use cosmwasm_std::{
     to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, 
};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, GetOwnerResponse, GetAddressesResponse, GetAddressResponse,
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRecord { user_name, owner, addresses } => execute_set_record(deps, env, info, user_name, owner, addresses),
    }
}

// TODO: add msg for adding admin and removing admin
pub fn execute_set_record(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_name: String,
    owner: Addr,
    addresses: Vec<(i32, String)>,
) -> Result<Response, ContractError> {
    // check if the msg sender is a registrar or admin. If not, return err
    let cfg = CONFIG.load(deps.storage)?;

    // TODO: make this into method
    let authorized = cfg.admins.iter().any(|a| a.as_ref() == info.sender.as_ref()) ||
        cfg.registrar_addresses.iter().any(|a| a.as_ref() == info.sender.as_ref());

    if !authorized {
        return Err(ContractError::Unauthorized {});
    }

    // check if the user_name is already registered
    // TODO: make only admin be able to override, error if registrar
    let existing = OWNER.may_load(deps.storage, user_name.clone())?;
    if let Some(_) = existing {
        return Err(ContractError::UserAlreadyRegistered { name: user_name });
    }

    OWNER.save(deps.storage, user_name.clone(), &owner)?;

    // save addresses per coin type for the user
    for (coin_type, address) in addresses {
        ADDRESSES.save(deps.storage, (user_name.clone(), coin_type), &address)?;
    }
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner { user_name } => to_binary(&query_owner(deps, env, user_name)?),
        QueryMsg::GetAddreses { user_name } => to_binary(&query_addresses(deps, env, user_name)?),
        QueryMsg::GetAddress { user_name, coin_type } => to_binary(&query_address(deps, env, user_name, coin_type)?),
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
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
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, DepsMut, Addr, coins, from_binary, Coin};

    use crate::msg::InstantiateMsg;

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

    fn set_alice_default_record(deps: DepsMut, sent: &[Coin], admin: String) {
        // alice can register an available name
        let info = mock_info(&admin, sent);
        let msg = ExecuteMsg::SetRecord {
            user_name: "alice".to_string(),
            owner: deps.api.addr_validate("alice").unwrap(),
            addresses: vec![(60, "0x1234".to_string()), (118, "cosmos1".to_string())],
        };
        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract successfully handles Register message");
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
    fn set_get_single_record() {
        let mut deps = mock_dependencies();
        let admin = String::from("test_admin");

        let admins = vec![admin.clone()];
        let registrar_addrs = vec![];
        mock_init(deps.as_mut(), admins, registrar_addrs);

        set_alice_default_record(deps.as_mut(), &coins(1, "token"), admin.clone());

        let get_owner_resp = query_owner(deps.as_ref(), mock_env(), String::from("alice")).unwrap();
        assert_eq!(get_owner_resp.owner.unwrap(), deps.as_ref().api.addr_validate("alice").unwrap());

        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), 60).unwrap();
        assert_eq!(addr.address.unwrap(), "0x1234".to_string());

        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), 118).unwrap();
        assert_eq!(addr.address.unwrap(), "cosmos1".to_string());

        let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("alice")).unwrap().addresses;
        assert_eq!(addrs.len(), 2);
        assert_eq!(addrs[0].1, "0x1234".to_string());
        assert_eq!(addrs[1].1, "cosmos1".to_string());
    }

    #[test]
    fn set_duplicate_username() {
        let mut deps = mock_dependencies();
        let admin = String::from("test_admin");

        let admins = vec![admin.clone()];
        let registrar_addrs = vec![];
        mock_init(deps.as_mut(), admins, registrar_addrs);

        set_alice_default_record(deps.as_mut(), &coins(1, "token"), admin.clone());

        // try setting record again, it should fail
        let info = mock_info(&admin, &coins(1, "token"));
        let msg = ExecuteMsg::SetRecord {
            user_name: "alice".to_string(),
            owner: deps.as_ref().api.addr_validate("alice").unwrap(),
            addresses: vec![(60, "0x1234".to_string()), (118, "cosmos1".to_string())],
        };

        // check that duplicate user name returns error
        let res = execute(deps.as_mut(), mock_env(), info, msg).is_err();
        assert_eq!(res, true);
    }



    #[test]
    fn set_get_multiple_records() {
        let mut deps = mock_dependencies();
        let admin = String::from("test_admin");
        let admins = vec![admin.clone()];
        let registrar_addrs = vec![];
        mock_init(deps.as_mut(), admins, registrar_addrs);

        set_alice_default_record(deps.as_mut(), &coins(1, "token"), admin.clone());

        // also set record for Bob
        let info = mock_info(&admin.clone(), &coins(1, "token"));
        let msg = ExecuteMsg::SetRecord {
            user_name: "bob".to_string(),
            owner: deps.as_ref().api.addr_validate("bob").unwrap(),
            addresses: vec![(60, "0x5678".to_string()), (118, "osmo1".to_string())],
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Register message");

        // check owner state for alice and bob
        let get_owner_resp = query_owner(deps.as_ref(), mock_env(), String::from("alice")).unwrap();
        assert_eq!(get_owner_resp.owner.unwrap(), deps.as_ref().api.addr_validate("alice").unwrap());
        let get_owner_resp = query_owner(deps.as_ref(), mock_env(), String::from("bob")).unwrap();
        assert_eq!(get_owner_resp.owner.unwrap(), deps.as_ref().api.addr_validate("bob").unwrap());

        // check addresses state for alice and bob
        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), 60).unwrap();
        assert_eq!(addr.address.unwrap(), "0x1234".to_string());
        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), 118).unwrap();
        assert_eq!(addr.address.unwrap(), "cosmos1".to_string());
        let addr = query_address(deps.as_ref(), mock_env(), String::from("bob"), 60).unwrap();
        assert_eq!(addr.address.unwrap(), "0x5678".to_string());
        let addr = query_address(deps.as_ref(), mock_env(), String::from("bob"), 118).unwrap();
        assert_eq!(addr.address.unwrap(), "osmo1".to_string());

        // check addresses query for alice and bob
        let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("alice")).unwrap().addresses;
        assert_eq!(addrs.len(), 2);
        assert_eq!(addrs[0].1, "0x1234".to_string());
        assert_eq!(addrs[1].1, "cosmos1".to_string());

        let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("bob")).unwrap().addresses;
        assert_eq!(addrs.len(), 2);
        assert_eq!(addrs[0].1, "0x5678".to_string());
        assert_eq!(addrs[1].1, "osmo1".to_string());
    }

    #[test]
    fn invalid_admin_registrar_addr() {
        let mut deps = mock_dependencies();

        let admin = String::from("test_admin");
        let registrar = String::from("test_registrar");
        let unregistered_address = String::from("unregistered_address");

        let admins = vec![admin.clone()];
        let registrar_addrs = vec![registrar.clone()];
        mock_init(deps.as_mut(), admins, registrar_addrs);

        // test with valid registrar
        let info = mock_info(&registrar, &coins(1, "token"));
        let msg = ExecuteMsg::SetRecord {
            user_name: "bob".to_string(),
            owner: deps.as_ref().api.addr_validate("bob").unwrap(),
            addresses: vec![(60, "0x5678".to_string()), (118, "osmo1".to_string())],
        };
        let is_error = execute(deps.as_mut(), mock_env(), info, msg.clone()).is_err();
        assert!(!is_error);

        let info = mock_info(&admin, &coins(1, "token"));
        let msg = ExecuteMsg::SetRecord {
            user_name: "charlie".to_string(),
            owner: deps.as_ref().api.addr_validate("charlie").unwrap(),
            addresses: vec![(60, "0x5678".to_string()), (118, "osmo1".to_string())],
        };
        let is_error = execute(deps.as_mut(), mock_env(), info, msg).is_err();
        assert!(!is_error);

        // test with invalid address: should fail
        let info = mock_info(&unregistered_address, &coins(1, "token"));
        let msg = ExecuteMsg::SetRecord {
            user_name: "alice".to_string(),
            owner: deps.as_ref().api.addr_validate("alice").unwrap(),
            addresses: vec![(60, "0x5678".to_string()), (118, "osmo1".to_string())],
        };
        let is_error = execute(deps.as_mut(), mock_env(), info, msg).is_err();
        assert!(is_error);
    }
}
