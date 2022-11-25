#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Order::Ascending;

use cosmwasm_std::{
    from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdResult, WasmQuery,
};
use cw2::set_contract_version;
use subtle_encoding::bech32;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetAddressResponse, GetAddressesResponse, InstantiateMsg, QueryMsg};
use crate::state::{Config, ADDRESSES, CONFIG};
use cw721::OwnerOfResponse;
use icns_name_nft::msg::{QueryMsg as QueryMsgName, AdminResponse};

// // version info for migration info
// const CONTRACT_NAME: &str = "crates.io:resolver";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn instantiate(
//     deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     msg: InstantiateMsg,
// ) -> Result<Response, ContractError> {
//     set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

//     let name_address = deps.api.addr_validate(&msg.name_address)?;

//     let cfg = Config { name_address };
//     CONFIG.save(deps.storage, &cfg)?;

//     Ok(Response::default())
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> Result<Response, ContractError> {
//     match msg {
//         ExecuteMsg::SetRecord {
//             user_name,
//             addresses,
//         } => execute_set_record(deps, env, info, user_name, addresses),
//     }
// }

// pub fn execute_set_record(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     user_name: String,
//     addresses: Vec<(String, String)>,
// ) -> Result<Response, ContractError> {
//     // check if the msg sender is a registrar or admin. If not, return err
//     let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
//     println!("is_admin: {}", is_admin);
//     let is_owner_nft = is_owner(deps.as_ref(), user_name.clone(), info.sender.to_string())?;

//     // if the sender is neither a registrar nor an admin, return error
//     if !is_admin && !is_owner_nft {
//         return Err(ContractError::Unauthorized {});
//     }

//     // do a sanity check on the given addresses for the different bech32 prefixes
//     // We do two checks here:
//     // 1. Check that the given addresses are valid bech32 addresses
//     // 2. Check if they match the given prefixes
//     // if the sanity check fails, we return an error
//     for (prefix, address) in addresses.iter() {
//         let prefix_decoded = bech32::decode(address)
//             .map_err(|_| ContractError::Bech32DecodingErr {
//                 addr: address.to_string(),
//             })?
//             .0;
//         if !prefix.eq(&prefix_decoded) {
//             return Err(ContractError::Bech32PrefixMismatch {
//                 prefix: prefix.to_string(),
//                 addr: address.to_string(),
//             });
//         }
//     }

//     // check if the user_name is already registered
//     let user_name_exists = query_addresses(deps.as_ref(), env, user_name.clone())?;
//     if !user_name_exists.addresses.is_empty() {
//         return Err(ContractError::UserAlreadyRegistered { name: user_name });
//     }

//     for (bech32_prefix, address) in addresses {
//         ADDRESSES.save(
//             deps.storage,
//             (user_name.clone(), bech32_prefix.clone()),
//             &address,
//         )?;
//     }

//     Ok(Response::default())
// }

// pub fn is_admin(deps: Deps, address: String) -> Result<bool, ContractError> {
//     let cfg = CONFIG.load(deps.storage)?;
//     let name_address = cfg.name_address;

//     // query admin from icns-name-nft contract
//     let query_msg = QueryMsgName::Admin {};
//     let res: AdminResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//         contract_addr: name_address.to_string(),
//         msg: to_binary(&query_msg)?,
//     }))?;

//     Ok(res.admins
//         .into_iter()
//         .find(|admin| admin.eq(&address))
//         .is_some())
// }

// pub fn admin(deps: Deps) -> Result<Vec<String>, ContractError> {
//     let cfg = CONFIG.load(deps.storage)?;
//     let name_address = cfg.name_address;

//     // query admin from icns-name-nft contract
//     let query_msg = QueryMsgName::Admin {};
//     let res: AdminResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//         contract_addr: name_address.to_string(),
//         msg: to_binary(&query_msg)?,
//     }))?;

//     Ok(res.admins)
// }

// pub fn is_owner(deps: Deps, username: String, sender: String) -> Result<bool, ContractError> {
//     let response = deps
//         .querier
//         .query(&QueryRequest::Wasm(WasmQuery::Smart {
//             contract_addr: CONFIG.load(deps.storage)?.name_address.to_string(),
//             msg: to_binary(&QueryMsgName::OwnerOf {
//                 token_id: username,
//                 include_expired: None,
//             })?,
//         }))
//         .map(|res| from_binary(&res).unwrap());

//     match response {
//         Ok(OwnerOfResponse { owner, .. }) => Ok(owner.eq(&sender)),
//         Err(_) => Ok(false),
//     }
// }

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
//         QueryMsg::GetAddreses { user_name } => to_binary(&query_addresses(deps, env, user_name)?),
//         QueryMsg::GetAddress {
//             user_name,
//             bec32_prefix,
//         } => to_binary(&query_address(deps, env, user_name, bec32_prefix)?),
//         QueryMsg::Admin {  } => to_binary(&query_admin(deps)?),
//         // TODO: add query to query directly using ICNS (e.g req: tony.eth)
//     }
// }

// fn query_addresses(deps: Deps, _env: Env, name: String) -> StdResult<GetAddressesResponse> {
//     let addresses = ADDRESSES
//         .prefix(name)
//         .range(deps.storage, None, None, Ascending)
//         .collect::<StdResult<Vec<_>>>()?;
//     let resp = GetAddressesResponse { addresses };

//     Ok(resp)
// }

// fn query_address(
//     deps: Deps,
//     _env: Env,
//     user_name: String,
//     bech32_prefix: String,
// ) -> StdResult<GetAddressResponse> {
//     let address = ADDRESSES.may_load(deps.storage, (user_name, bech32_prefix))?;
//     let resp = GetAddressResponse { address };

//     Ok(resp)
// }

// fn query_admin(
//     deps: Deps,
// ) -> StdResult<AdminResponse> {
//     // unwrap this 
//     let result = admin(deps);
//     match result {
//         Ok(admins) => Ok(AdminResponse { admins }),
//         Err(_) => Ok(AdminResponse { admins: vec![String::from("")] }),
//     }
// }

// #[cfg(test)]
// mod tests {
//     use cosmwasm_std::{
//         coins, from_binary,
//         testing::{mock_dependencies, mock_env, mock_info},
//         Addr, Coin, DepsMut,
//     };

//     use crate::msg::InstantiateMsg;

//     use super::*;

//     fn mock_init(deps: DepsMut, name_addr: String) {
//         let msg = InstantiateMsg {
//             name_address: name_addr,
//         };

//         let info = mock_info("creator", &coins(1, "token"));
//         let _res = instantiate(deps, mock_env(), info, msg)
//             .expect("contract successfully handles InstantiateMsg");
//     }

//     fn set_alice_default_addresses(deps: DepsMut, sent: &[Coin], registrar_addr: String) {
//         // alice can register an available name
//         let info = mock_info(&registrar_addr, sent);
//         let msg = ExecuteMsg::SetRecord {
//             user_name: "alice".to_string(),
//             addresses: vec![
//                 ("eth".to_string(), "0x1234".to_string()),
//                 ("cosmos".to_string(), "cosmos1".to_string()),
//             ],
//         };
//         let _res = execute(deps, mock_env(), info, msg)
//             .expect("contract successfully handles Register message");
//     }

//     #[test]
//     fn proper_instantiate() {
//         let mut deps = mock_dependencies();

//         let name_addr = String::from("name");
//         mock_init(deps.as_mut(), name_addr.clone());

//         let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
//         let value: Config = from_binary(&res).unwrap();
//         let expected = Config {
//             name_address: Addr::unchecked(name_addr),
//         };
//         assert_eq!(value, expected);
//     }

//     // TODO: Move these to integration tests
//     // #[test]
//     // fn set_get_single_record() {
//     //     let mut deps = mock_dependencies();

//     //     let registrar_addr = String::from("registrar");
//     //     mock_init(deps.as_mut(), registrar_addr.clone());

//     //     set_alice_default_addresses(deps.as_mut(), &coins(1, "token"), registrar_addr.clone());

//     //     let addr = query_address(
//     //         deps.as_ref(),
//     //         mock_env(),
//     //         String::from("alice"),
//     //         String::from("eth"),
//     //     )
//     //     .unwrap();
//     //     assert_eq!(addr.address.unwrap(), "0x1234".to_string());

//     //     let addr = query_address(
//     //         deps.as_ref(),
//     //         mock_env(),
//     //         String::from("alice"),
//     //         String::from("cosmos"),
//     //     )
//     //     .unwrap();
//     //     assert_eq!(addr.address.unwrap(), "cosmos1".to_string());

//     //     let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("alice"))
//     //         .unwrap()
//     //         .addresses;
//     //     assert_eq!(addrs.len(), 2);
//     //     assert_eq!(addrs[0].1, "cosmos1".to_string());
//     //     assert_eq!(addrs[1].1, "0x1234".to_string());
//     // }

//     // #[test]
//     // fn set_duplicate_username() {
//     //     let mut deps = mock_dependencies();
//     //     let registry_addr = String::from("registrar");
//     //     mock_init(deps.as_mut(), registry_addr.clone());

//     //     set_alice_default_addresses(deps.as_mut(), &coins(1, "token"), registry_addr.clone());

//     //     // try setting record again, it should fail
//     //     let info = mock_info(&registry_addr, &coins(1, "token"));
//     //     let msg = ExecuteMsg::SetRecord {
//     //         user_name: "alice".to_string(),
//     //         addresses: vec![
//     //             ("eth".to_string(), "0x1234".to_string()),
//     //             ("cosmos".to_string(), "cosmos1".to_string()),
//     //         ],
//     //     };

//     //     // check that duplicate user name returns error
//     //     let res = execute(deps.as_mut(), mock_env(), info, msg).is_err();
//     //     assert_eq!(res, true);
//     // }

//     // #[test]
//     // fn test_address_verification() {
//     //     let mut deps = mock_dependencies();

//     //     let name_addr = String::from("name");

//     //     mock_init(deps.as_mut(), name_addr.clone());

//     //     // first try testing with invalid bech 32 address
//     //     let info = mock_info(&name_addr, &coins(1, "token"));
//     //     let msg = ExecuteMsg::SetRecord {
//     //         user_name: String::from("user_name"),
//     //         addresses: vec![(
//     //             String::from("cosmos"),
//     //             String::from("cosmos1dsfsfasdfknsfkndfknskdfns"),
//     //         )],
//     //     };

//     //     let res = execute(deps.as_mut(), mock_env(), info, msg).is_err();
//     //     assert_eq!(res, true);

//     //     // try testing with unmatching bech32 prefix and address
//     //     // this should fail
//     //     let info = mock_info(&name_addr, &coins(1, "token"));
//     //     let msg = ExecuteMsg::SetRecord {
//     //         user_name: String::from("user_name"),
//     //         addresses: vec![
//     //             (
//     //                 String::from("cosmos"),
//     //                 String::from("osmo19clxjvtgn8es8ylytgztalsw2fygh6etyd9hq7"),
//     //             ),
//     //             (
//     //                 String::from("juno"),
//     //                 String::from("juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts"),
//     //             ),
//     //         ],
//     //     };
//     //     let res = execute(deps.as_mut(), mock_env(), info, msg).is_err();
//     //     assert_eq!(res, true);

//     //     // try testing with valid bech32 address and prefix
//     //     let info = mock_info(&name_addr, &coins(1, "token"));
//     //     let msg = ExecuteMsg::SetRecord {
//     //         user_name: String::from("user_name"),
//     //         addresses: vec![(
//     //             String::from("juno"),
//     //             String::from("juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts"),
//     //         )],
//     //     };
//     //     let res = execute(deps.as_mut(), mock_env(), info, msg).is_err();
//     //     assert_eq!(res, false);
//     // }

//     // #[test]
//     // fn set_get_multiple_records() {
//     //     let mut deps = mock_dependencies();
//     //     let name_addr = String::from("name");
//     //     mock_init(deps.as_mut(), name_addr.clone());

//     //     set_alice_default_addresses(deps.as_mut(), &coins(1, "token"), name_addr.clone());

//     //     // also set record for Bob
//     //     let info = mock_info(&name_addr.clone(), &coins(1, "token"));
//     //     let msg = ExecuteMsg::SetRecord {
//     //         user_name: "bob".to_string(),
//     //         addresses: vec![
//     //             ("eth".to_string(), "0x5678".to_string()),
//     //             ("osmo".to_string(), "osmo1".to_string()),
//     //         ],
//     //     };
//     //     let _res = execute(deps.as_mut(), mock_env(), info, msg)
//     //         .expect("contract successfully handles Register message");

//     //     // check addresses state for alice and bob
//     //     let addr = query_address(
//     //         deps.as_ref(),
//     //         mock_env(),
//     //         String::from("alice"),
//     //         "eth".to_string(),
//     //     )
//     //     .unwrap();
//     //     assert_eq!(addr.address.unwrap(), "0x1234".to_string());
//     //     let addr = query_address(
//     //         deps.as_ref(),
//     //         mock_env(),
//     //         String::from("alice"),
//     //         "cosmos".to_string(),
//     //     )
//     //     .unwrap();
//     //     assert_eq!(addr.address.unwrap(), "cosmos1".to_string());
//     //     let addr = query_address(
//     //         deps.as_ref(),
//     //         mock_env(),
//     //         String::from("bob"),
//     //         "eth".to_string(),
//     //     )
//     //     .unwrap();
//     //     assert_eq!(addr.address.unwrap(), "0x5678".to_string());
//     //     let addr = query_address(
//     //         deps.as_ref(),
//     //         mock_env(),
//     //         String::from("bob"),
//     //         "osmo".to_string(),
//     //     )
//     //     .unwrap();
//     //     assert_eq!(addr.address.unwrap(), "osmo1".to_string());

//     //     // check addresses query for alice and bob
//     //     let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("alice"))
//     //         .unwrap()
//     //         .addresses;
//     //     assert_eq!(addrs.len(), 2);
//     //     assert_eq!(addrs[0].1, "cosmos1".to_string());
//     //     assert_eq!(addrs[1].1, "0x1234".to_string());

//     //     let addrs = query_addresses(deps.as_ref(), mock_env(), String::from("bob"))
//     //         .unwrap()
//     //         .addresses;
//     //     assert_eq!(addrs.len(), 2);
//     //     assert_eq!(addrs[0].1, "0x5678".to_string());
//     //     assert_eq!(addrs[1].1, "osmo1".to_string());
//     // }

//     // #[test]
//     // fn set_with_invalid_registrar() {
//     //     let mut deps = mock_dependencies();

//     //     let name_addr = String::from("name");
//     //     let unregistered_registrar_addr = String::from("unregistered_registrar");
//     //     mock_init(deps.as_mut(), name_addr.clone());

//     //     // test with valid registrar
//     //     let info = mock_info(&name_addr, &coins(1, "token"));
//     //     let msg = ExecuteMsg::SetRecord {
//     //         user_name: "bob".to_string(),
//     //         addresses: vec![
//     //             ("eth".to_string(), "0x5678".to_string()),
//     //             ("osmo".to_string(), "osmo1".to_string()),
//     //         ],
//     //     };
//     //     let is_error = execute(deps.as_mut(), mock_env(), info, msg.clone()).is_err();
//     //     assert!(!is_error);

//     //     // test with invalid address: should fail
//     //     let info = mock_info(&unregistered_registrar_addr, &coins(1, "token"));
//     //     let msg = ExecuteMsg::SetRecord {
//     //         user_name: "alice".to_string(),
//     //         addresses: vec![
//     //             ("eth".to_string(), "0x5678".to_string()),
//     //             ("osmo".to_string(), "osmo1".to_string()),
//     //         ],
//     //     };
//     //     let is_error = execute(deps.as_mut(), mock_env(), info, msg).is_err();
//     //     assert!(is_error);
//     // }
// }
