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

use crate::crypto::adr36_verification;
use crate::error::ContractError;
use crate::msg::{
    AddressHash, Adr36Info, ExecuteMsg, GetAddressResponse, GetAddressesResponse, InstantiateMsg,
    QueryMsg,
};
use crate::state::{AddressInfo, Config, ADDRESSES, CONFIG, REVERSE_RESOLVER, SIGNATURE};
use cw721::OwnerOfResponse;
use icns_name_nft::msg::{AdminResponse, QueryMsg as QueryMsgName};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:icns-resolver";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let name_address = deps.api.addr_validate(&msg.name_address)?;

    let cfg = Config { name_address };
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
        ExecuteMsg::SetRecord {
            user_name,
            bech32_prefix,
            adr36_info,
            replace_primary_if_exists,
            signature_salt,
        } => execute_set_record(
            deps,
            env,
            info,
            user_name,
            bech32_prefix,
            adr36_info,
            replace_primary_if_exists,
            signature_salt,
        ),
    }
}

pub fn execute_set_record(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_name: String,
    bech32_prefix: String,
    adr36_info: Adr36Info,
    replace_primary_if_exists: bool,
    signature_salt: u128,
) -> Result<Response, ContractError> {
    // check if the msg sender is a registrar or admin. If not, return err
    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    let is_owner_nft = is_owner(deps.as_ref(), user_name.clone(), info.sender.to_string())?;

    // if the sender is neither a registrar nor an admin, return error
    if !is_admin && !is_owner_nft {
        return Err(ContractError::Unauthorized {});
    }

    // check address hash method, currently only sha256 is supported
    if adr36_info.address_hash != AddressHash::SHA256 {
        return Err(ContractError::HashMethodNotSupported {});
    }

    // extract bech32 prefix from given address
    let bech32_prefix_decoded = bech32::decode(adr36_info.bech32_address.clone())
        .map_err(|_| ContractError::Bech32DecodingErr {
            addr: adr36_info.bech32_address.to_string(),
        })?
        .0;

    // first check if the user input for prefix + address is valid
    if bech32_prefix != bech32_prefix_decoded {
        return Err(ContractError::Bech32PrefixMismatch {
            prefix: bech32_prefix,
            addr: adr36_info.bech32_address,
        });
    }

    // do adr36 verification
    let chain_id = env.block.chain_id;
    let contract_address = env.contract.address.to_string();
    adr36_verification(
        deps.as_ref(),
        user_name.clone(),
        bech32_prefix.clone(),
        adr36_info.clone(),
        chain_id,
        contract_address,
        signature_salt,
    )?;

    // now append address to the existing reverse resolver list
    let mut address_info = AddressInfo {
        user_name: user_name.clone(),
        bech32_prefix: bech32_prefix.clone(),
        primary: false,
    };

    let existing_addresses =
        REVERSE_RESOLVER.may_load(deps.storage, adr36_info.bech32_address.clone());

    match existing_addresses {
        Ok(Some(mut existing_addresses)) => {
            if replace_primary_if_exists {
                // iterate over all existing addresses and set primary to false
                for address in existing_addresses.iter_mut() {
                    if address.primary {
                        address.primary = false;
                    }
                }
                // set the current one as primary
                address_info.primary = true;
            }

            existing_addresses.push(address_info);
            REVERSE_RESOLVER.save(
                deps.storage,
                adr36_info.bech32_address.clone(),
                &existing_addresses,
            )?;
        }
        Ok(None) => {
            let mut records = Vec::new();
            // set this to primary address if it was no address has existed before
            address_info.primary = true;
            records.push(address_info);
            REVERSE_RESOLVER.save(deps.storage, adr36_info.bech32_address.clone(), &records)?;
        }
        Err(_) => {
            return Err(ContractError::StorageErr {});
        }
    }

    // now override the address in the storage
    ADDRESSES.save(
        deps.storage,
        (user_name, bech32_prefix),
        &adr36_info.bech32_address,
    )?;

    // save signature to prevent replay attack
    SIGNATURE.save(deps.storage, adr36_info.signature.as_slice(), &true)?;

    Ok(Response::default())
}

pub fn is_admin(deps: Deps, address: String) -> Result<bool, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let name_address = cfg.name_address;

    // query admin from icns-name-nft contract
    let query_msg = QueryMsgName::Admin {};
    let res: AdminResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: name_address.to_string(),
        msg: to_binary(&query_msg)?,
    }))?;

    Ok(res.admins.into_iter().any(|admin| admin.eq(&address)))
}

pub fn admin(deps: Deps) -> Result<Vec<String>, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let name_address = cfg.name_address;

    // query admin from icns-name-nft contract
    let query_msg = QueryMsgName::Admin {};
    let res: AdminResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: name_address.to_string(),
        msg: to_binary(&query_msg)?,
    }))?;

    Ok(res.admins)
}

pub fn is_owner(deps: Deps, username: String, sender: String) -> Result<bool, ContractError> {
    let response = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: CONFIG.load(deps.storage)?.name_address.to_string(),
            msg: to_binary(&QueryMsgName::OwnerOf {
                token_id: username,
                include_expired: None,
            })?,
        }))
        .map(|res| from_binary(&res).unwrap());

    match response {
        Ok(OwnerOfResponse { owner, .. }) => Ok(owner.eq(&sender)),
        Err(_) => Ok(false),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::GetAddresses { user_name } => to_binary(&query_addresses(deps, env, user_name)?),
        QueryMsg::GetAddress {
            user_name,
            bech32_prefix,
        } => to_binary(&query_address(deps, env, user_name, bech32_prefix)?),
        QueryMsg::Admin {} => to_binary(&query_admin(deps)?),
        // TODO: add query to query directly using ICNS (e.g req: tony.eth)
    }
}

fn query_addresses(deps: Deps, _env: Env, name: String) -> StdResult<GetAddressesResponse> {
    let addresses = ADDRESSES
        .prefix(name)
        .range(deps.storage, None, None, Ascending)
        .collect::<StdResult<Vec<_>>>()?;
    if &addresses.len() == &0 {
        return Ok(GetAddressesResponse { addresses: vec![] });
    }
    let resp = GetAddressesResponse { addresses };

    Ok(resp)
}

fn query_address(
    deps: Deps,
    _env: Env,
    user_name: String,
    bech32_prefix: String,
) -> StdResult<GetAddressResponse> {
    let address = ADDRESSES.may_load(deps.storage, (user_name, bech32_prefix))?;

    match address {
        Some(addr) => Ok(GetAddressResponse { address: addr }),
        None => Ok(GetAddressResponse {
            address: "".to_string(),
        }),
    }
}

fn query_admin(deps: Deps) -> StdResult<AdminResponse> {
    // unwrap this
    let result = admin(deps);
    match result {
        Ok(admins) => Ok(AdminResponse { admins }),
        Err(_) => Ok(AdminResponse {
            admins: vec![String::from("")],
        }),
    }
}
