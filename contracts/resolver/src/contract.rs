use cw2::set_contract_version;
use cosmwasm_std::Order::Ascending;
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, QueryRequest, WasmQuery, to_binary, from_binary};
// use cw2::set_contract_version;

use registry::msg::{QueryMsg as QueryMsgRegistry, IsAdminResponse};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetAddressesResponse, GetAddressResponse};
use crate::state::{ Config,
    CONFIG, ADDRESSES
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:resolver";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

      let registrar_address = deps.api.addr_validate(&msg.registrar_address)?;
  
      let cfg = Config {
          registrar_address: registrar_address,
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
        ExecuteMsg::SetAddresses { user_name, addresses } => execute_set_addresses(deps, env, info, user_name, addresses),
    }
}

pub fn execute_set_addresses(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_name: String,
    addresses: Vec<(String, String)>,
) -> Result<Response, ContractError> {
     // check if the msg sender is a registrar or admin. If not, return err
     let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
     let is_registrar = is_registrar(deps.as_ref(), info.sender.to_string())?;

     // if the sender is neither a registrar nor an admin, return error
     if !is_admin && !is_registrar {
         return Err(ContractError::Unauthorized {});
     }

     // check if the user_name is already registered
    let user_name_exists = query_addresses(deps.as_ref(), env, user_name.clone())?;
    if user_name_exists.addresses.len() > 0 {
        return Err(ContractError::UserAlreadyRegistered { name: user_name });
    }

     for (bech32_prefix, address) in addresses {
         ADDRESSES.save(deps.storage, (user_name.clone(), bech32_prefix.clone()), &address)?;
     }
 
     Ok(Response::default())   
}

pub fn is_admin(deps: Deps, address: String) -> Result<bool, ContractError> {
   let response = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
       contract_addr: CONFIG.load(deps.storage)?.registrar_address.to_string(),
       msg: to_binary(&QueryMsgRegistry::IsAdmin {address})?,
   })).map(|res| from_binary(&res).unwrap());

   // TODO: come back and decide and change how we handle the contract error here
    match response {
         Ok(IsAdminResponse {is_admin}) => Ok(is_admin),
         Err(_) => Ok(false),
    }
}

pub fn is_registrar(deps: Deps, address: String) -> Result<bool, ContractError> {
    let registrar = CONFIG.load(deps.storage)?.registrar_address.to_string();
    Ok(registrar == address)
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::GetAddreses { user_name } => to_binary(&query_addresses(deps, env, user_name)?),
        QueryMsg::GetAddress { user_name, bec32_prefix } => to_binary(&query_address(deps, env, user_name, bec32_prefix)?),
    }
}

fn query_addresses(deps: Deps, _env: Env, name: String) -> StdResult<GetAddressesResponse> {
    let addresses = ADDRESSES
        .prefix(name)
        .range(deps.storage, None, None, Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    let resp = GetAddressesResponse { addresses: addresses };

    Ok(resp)
}

fn query_address(deps: Deps, _env: Env, user_name: String, bech32_prefix: String) -> StdResult<GetAddressResponse> {
    let address = ADDRESSES.may_load(deps.storage, (user_name, bech32_prefix))?;
    let resp = GetAddressResponse { address };

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, DepsMut, Addr, coins, from_binary, Coin};

    use crate::msg::{InstantiateMsg};

    use super::*;

    fn mock_init(
        deps: DepsMut,
        registrar_addr: String,
    ) {
        let msg = InstantiateMsg {
            registrar_address: registrar_addr.to_string(),
        };

        let info = mock_info("creator", &coins(1, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
    }

    fn set_alice_default_addresses(deps: DepsMut, sent: &[Coin], registrar_addr: String) {
        // alice can register an available name
        let info = mock_info(&registrar_addr, sent);
        let msg = ExecuteMsg::SetAddresses {
            user_name: "alice".to_string(),
            addresses: vec![("eth".to_string(), "0x1234".to_string()), ("cosmos".to_string(), "cosmos1".to_string())],
        };
        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract successfully handles Register message");
    }

    #[test]
    fn proper_instantiate() {
        let mut deps = mock_dependencies();

        let registry_addr = String::from("registry");
        mock_init(deps.as_mut(), registry_addr.clone());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        let expected = Config {
            registrar_address: Addr::unchecked(registry_addr),
        };
        assert_eq!(value, expected);
    }

    #[test]
    fn test_is_registry() {
        let mut deps = mock_dependencies();

        let registrar_addr = String::from("registrar");

        mock_init(deps.as_mut(), registrar_addr.clone());

        let is_registrar_result = is_registrar(deps.as_ref(), registrar_addr.clone()).unwrap();
        assert_eq!(is_registrar_result, true);

        let non_registrar_addr = String::from("non_registrar");
        let is_registrar_result = is_registrar(deps.as_ref(), non_registrar_addr.clone()).unwrap();
        assert_eq!(is_registrar_result, false);
    }
    #[test]
    fn set_get_single_record() {
        let mut deps = mock_dependencies();
      
        let registrar_addr = String::from("registrar");
        mock_init(deps.as_mut(), registrar_addr.clone());

        set_alice_default_addresses(deps.as_mut(), &coins(1, "token"), registrar_addr.clone());

        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), String::from("eth")).unwrap();
        assert_eq!(addr.address.unwrap(), "0x1234".to_string());

        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), String::from("cosmos")).unwrap();
        assert_eq!(addr.address.unwrap(), "cosmos1".to_string());

        let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("alice")).unwrap().addresses;
        assert_eq!(addrs.len(), 2);
        assert_eq!(addrs[0].1, "cosmos1".to_string());
        assert_eq!(addrs[1].1, "0x1234".to_string());
    }

    #[test]
    fn set_duplicate_username() {
        let mut deps = mock_dependencies();
        let registrar_addr = String::from("registrar");
        mock_init(deps.as_mut(), registrar_addr.clone());

        set_alice_default_addresses(deps.as_mut(), &coins(1, "token"), registrar_addr.clone());

        // try setting record again, it should fail
        let info = mock_info( &registrar_addr, &coins(1, "token"));
        let msg = ExecuteMsg::SetAddresses {
            user_name: "alice".to_string(),
            addresses: vec![("eth".to_string(), "0x1234".to_string()), ("cosmos".to_string(), "cosmos1".to_string())],
        };

        // check that duplicate user name returns error
        let res = execute(deps.as_mut(), mock_env(), info, msg).is_err();
        assert_eq!(res, true);
    }

    #[test]
    fn set_get_multiple_records() {
        let mut deps = mock_dependencies();
        let registrar_addr = String::from("registrar");
        mock_init(deps.as_mut(), registrar_addr.clone());

        set_alice_default_addresses(deps.as_mut(), &coins(1, "token"), registrar_addr.clone());

        // also set record for Bob
        let info = mock_info(&registrar_addr.clone(), &coins(1, "token"));
        let msg = ExecuteMsg::SetAddresses {
            user_name: "bob".to_string(),
            addresses: vec![("eth".to_string(), "0x5678".to_string()), ("osmo".to_string(), "osmo1".to_string())],
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg)
            .expect("contract successfully handles Register message");

        // check addresses state for alice and bob
        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), "eth".to_string()).unwrap();
        assert_eq!(addr.address.unwrap(), "0x1234".to_string());
        let addr = query_address(deps.as_ref(), mock_env(), String::from("alice"), "cosmos".to_string()).unwrap();
        assert_eq!(addr.address.unwrap(), "cosmos1".to_string());
        let addr = query_address(deps.as_ref(), mock_env(), String::from("bob"), "eth".to_string()).unwrap();
        assert_eq!(addr.address.unwrap(), "0x5678".to_string());
        let addr = query_address(deps.as_ref(), mock_env(), String::from("bob"), "osmo".to_string()).unwrap();
        assert_eq!(addr.address.unwrap(), "osmo1".to_string());

        // check addresses query for alice and bob
        let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("alice")).unwrap().addresses;
        assert_eq!(addrs.len(), 2);
        assert_eq!(addrs[0].1, "cosmos1".to_string());
        assert_eq!(addrs[1].1, "0x1234".to_string());

        let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("bob")).unwrap().addresses;
        assert_eq!(addrs.len(), 2);
        assert_eq!(addrs[0].1, "0x5678".to_string());
        assert_eq!(addrs[1].1, "osmo1".to_string());
    }

    #[test]
    fn set_with_invalid_registrar() {
        let mut deps = mock_dependencies();

        let registrar_addr = String::from("registrar");
        let unregistered_registrar_addr = String::from("unregistered_registrar");
        mock_init(deps.as_mut(), registrar_addr.clone());

        // test with valid registrar
        let info = mock_info(&registrar_addr, &coins(1, "token"));
        let msg = ExecuteMsg::SetAddresses {
            user_name: "bob".to_string(),
            addresses: vec![("eth".to_string(), "0x5678".to_string()), ("osmo".to_string(), "osmo1".to_string())],
        };
        let is_error = execute(deps.as_mut(), mock_env(), info, msg.clone()).is_err();
        assert!(!is_error);

        // test with invalid address: should fail
        let info = mock_info(&unregistered_registrar_addr, &coins(1, "token"));
        let msg = ExecuteMsg::SetAddresses {
            user_name: "alice".to_string(),
            addresses: vec![("eth".to_string(), "0x5678".to_string()), ("osmo".to_string(), "osmo1".to_string())],
        };
        let is_error = execute(deps.as_mut(), mock_env(), info, msg).is_err();
        assert!(is_error);
    }
}
