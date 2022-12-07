#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Order::Ascending;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError,
    StdResult, WasmQuery,
};
use cw2::set_contract_version;
use cw_storage_plus::KeyDeserialize;
use subtle_encoding::bech32;

use crate::crypto::{
    adr36_verification, cosmos_pubkey_to_bech32_address, eth_pubkey_to_bech32_address,
};
use crate::error::ContractError;
use crate::msg::{
    AddressByIcnsResponse, AddressHash, AddressResponse, AddressesResponse, Adr36Info, ExecuteMsg,
    InstantiateMsg, MigrateMsg, NamesResponse, PrimaryNameResponse, QueryMsg, IcnsNamesResponse,
};
use crate::state::{records, Config, CONFIG, PRIMARY_NAME, SIGNATURE};
use cw721::OwnerOfResponse;
use icns_name_nft::msg::{AdminResponse, QueryMsg as QueryMsgName};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
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
            name,
            bech32_prefix,
            adr36_info,
        } => execute_set_record(deps, env, info, name, bech32_prefix, adr36_info),
        ExecuteMsg::SetPrimary {
            name,
            bech32_address,
        } => execute_set_primary(deps, info, name, bech32_address),
        ExecuteMsg::RemoveRecord {
            name,
            bech32_address,
        } => execute_remove_record(deps, info, name, bech32_address),
    }
}

pub fn execute_set_record(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    bech32_prefix: String,
    adr36_info: Adr36Info,
) -> Result<Response, ContractError> {
    // check if the msg sender is a registrar or admin. If not, return err
    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    let is_owner_nft = is_owner(deps.as_ref(), name.clone(), info.sender.to_string())?;

    // if the sender is neither a registrar nor an admin, return error
    if !is_admin && !is_owner_nft {
        return Err(ContractError::Unauthorized {});
    }

    // if the sender is admin, skip adr 36 verification
    if !is_admin {
        // first check sender and the bech32 address in msg match
        // if it does, no need to verify adr36
        // in order to check if they match, we first need to decode the bech32 address
        let (bech32_prefix_decoded, bech32_address_decoded) = bech32::decode(adr36_info.signer_bech32_address.clone())
            .map_err(|_| ContractError::Bech32DecodingErr {
                addr: bech32_prefix.clone(),
            })?;
        let decoded_bech32_addr_from_info = bech32::decode(info.sender.clone())
            .map_err(|_| ContractError::Bech32DecodingErr {
                addr: adr36_info.signer_bech32_address.clone(),
            })?
            .1;

        // check if the user input for prefix + address is valid
        if bech32_prefix != bech32_prefix_decoded {
            return Err(ContractError::Bech32PrefixMismatch {
                prefix: bech32_prefix,
                addr: adr36_info.signer_bech32_address,
            });
        }

        // if they don't match, verify adr36
        if bech32_address_decoded != decoded_bech32_addr_from_info {
            if adr36_info.address_hash == AddressHash::Cosmos {
                // if address hash is for Cosmos, first verify that pub key is 33 bytes
                if adr36_info.pub_key.len() != 33 {
                    return Err(ContractError::InvalidPubKey {
                        pub_key: adr36_info.pub_key.to_string(),
                    });
                }
                
              

                // extract pubkey to bech32 address, check that it matches with the given bech32 address
                let decoded_bech32_addr = cosmos_pubkey_to_bech32_address(
                    adr36_info.pub_key.clone(),
                    bech32_prefix.clone(),
                );
                if decoded_bech32_addr != adr36_info.signer_bech32_address {
                    return Err(ContractError::SignatureMisMatch {});
                }
            } else if adr36_info.address_hash == AddressHash::Ethereum {
                if adr36_info.pub_key.len() != 65 {
                    return Err(ContractError::InvalidPubKey {
                        pub_key: adr36_info.pub_key.to_string(),
                    });
                }

                let decoded_bech32_addr =
                    eth_pubkey_to_bech32_address(adr36_info.pub_key.clone(), bech32_prefix.clone());
                if decoded_bech32_addr != adr36_info.signer_bech32_address {
                    return Err(ContractError::SignatureMisMatch {});
                }
            } else {
                return Err(ContractError::HashMethodNotSupported {});
            }

            // do adr36 verification
            let chain_id = env.block.chain_id;
            let contract_address = env.contract.address.to_string();
            adr36_verification(
                deps.as_ref(),
                name.clone(),
                info.sender.into_string(),
                bech32_prefix.clone(),
                adr36_info.clone(),
                chain_id,
                contract_address,
            )?;
        }
    }

    // save record
    records().save(
        deps.storage,
        (&name, &bech32_prefix),
        &adr36_info.signer_bech32_address,
    )?;

    // over-ride primary name
    PRIMARY_NAME.save(deps.storage, adr36_info.signer_bech32_address, &name)?;

    // save signature to prevent replay attack
    SIGNATURE.save(deps.storage, adr36_info.signature.as_slice(), &true)?;

    Ok(Response::default())
}

fn execute_set_primary(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    bech32_address: String,
) -> Result<Response, ContractError> {
    // check if the msg sender is a registrar or admin. If not, return err
    let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
    let is_owner_nft = is_owner(deps.as_ref(), name.clone(), info.sender.to_string())?;
    if !is_admin && !is_owner_nft {
        return Err(ContractError::Unauthorized {});
    }

    // extract bech32 prefix from given address
    let bech32_prefix_decoded = bech32::decode(bech32_address.clone())
        .map_err(|_| ContractError::Bech32DecodingErr {
            addr: bech32_address.to_string(),
        })?
        .0;

    // bech32 address needs to be already set in the records(for adr36 veficiation)
    // check in state if this is already set
    let bech32_address_stored =
        records().may_load(deps.storage, (&name, &bech32_prefix_decoded))?;

    if bech32_address_stored.as_ref() != Some(&bech32_address) {
        return Err(ContractError::Bech32AddressNotSet {
            name,
            address: bech32_address,
        });
    }

    PRIMARY_NAME.save(deps.storage, bech32_address, &name)?;

    Ok(Response::new()
        .add_attribute("method", "set_primary")
        .add_attribute("name", name))
}

fn execute_remove_record(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    bech32_address: String,
) -> Result<Response, ContractError> {
    // check if the msg sender is the owner of the name or an admin. If not, return err
    if !is_owner(deps.as_ref(), name.clone(), info.sender.to_string())?
        && !is_admin(deps.as_ref(), info.sender.to_string())?
    {
        return Err(ContractError::Unauthorized {});
    }

    // check if the name exists
    // extract bech32 prefix from given address
    let bech32_prefix_decoded = bech32::decode(bech32_address.clone())
        .map_err(|_| ContractError::Bech32DecodingErr {
            addr: bech32_address.to_string(),
        })?
        .0;
    let bech32_address_stored =
        records().may_load(deps.storage, (&name, &bech32_prefix_decoded))?;

    // check if bech32 address is set for this name
    if bech32_address_stored.as_ref() != Some(&bech32_address) {
        return Err(ContractError::Bech32AddressNotSet {
            name,
            address: bech32_address,
        });
    }

    // remove primary name if mapped to this address
    let primary_name = PRIMARY_NAME.may_load(deps.storage, bech32_address.clone())?;

    // if name is primary_name
    if primary_name.as_ref() == Some(&name) {
        let has_multiple_record = records()
            .idx
            .address
            .prefix(bech32_address.clone())
            .range(deps.storage, None, None, Ascending)
            .take(2) // save iteration
            .count()
            > 1;

        // should not removing record allow when address has multiple name to map to
        if has_multiple_record {
            return Err(ContractError::RemovingPrimaryAddressNotAllowed {});
        }

        // if there is only one name mapped to this address, it is allowed to remove
        // even though it's a primary_name since set_record when there is not record for that address
        // will force set it to primary_name which will make address to always have primary_name
        // when there is a record for that address
        PRIMARY_NAME.remove(deps.storage, bech32_address)
    }

    records().remove(deps.storage, (&name, &bech32_prefix_decoded))?;

    Ok(Response::new()
        .add_attribute("method", "remove_record")
        .add_attribute("name", name))
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
    let response = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CONFIG.load(deps.storage)?.name_address.to_string(),
        msg: to_binary(&QueryMsgName::OwnerOf {
            token_id: username,
            include_expired: None,
        })?,
    }));

    match response {
        Ok(OwnerOfResponse { owner, .. }) => Ok(owner == sender),
        Err(_) => Ok(false),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::Addresses { name } => to_binary(&query_addresses(deps, env, name)?),
        QueryMsg::Address {
            name,
            bech32_prefix,
        } => to_binary(&query_address(deps, env, name, bech32_prefix)?),
        QueryMsg::Admin {} => to_binary(&query_admin(deps)?),
        QueryMsg::PrimaryName { address } => to_binary(&query_primary_name(deps, address)?),
        QueryMsg::Names { address } => to_binary(&query_names(deps, address)?),
        QueryMsg::IcnsNames { address } => to_binary(&query_icns_names(deps, address)?),
        QueryMsg::AddressByIcns { icns } => to_binary(&query_address_by_icns(deps, icns)?),
    }
}

fn query_names(deps: Deps, address: String) -> StdResult<NamesResponse> {
    let primary_name = PRIMARY_NAME.load(deps.storage, address.clone())?;
    Ok(NamesResponse {
        names: records()
            .idx
            .address
            .prefix(address)
            .keys(deps.storage, None, None, Ascending)
            // get name out of StdResult<(name, bech32_prefix)
            .map(|result| {
                result
                    .iter()
                    .map(|key| {
                        let (name, _) = <(String, String)>::from_slice(key.as_bytes())?;
                        Ok(name)
                    })
                    .collect::<StdResult<String>>()
            })
            .collect::<StdResult<_>>()?,
        primary_name,
    })
}

fn query_icns_names(deps: Deps, address: String) -> StdResult<IcnsNamesResponse> {
    let primary_name = PRIMARY_NAME.load(deps.storage, address.clone())?;

    let bech32_prefix = bech32::decode(address.clone())
        .map_err(|_| cosmwasm_std::StdError::GenericErr {
            msg: "Invalid bech32 address".to_string(),
        })?
        .0;

    Ok(IcnsNamesResponse {
        names: records()
            .idx
            .address
            .prefix(address)
            .keys(deps.storage, None, None, Ascending)
            // get name out of StdResult<(name, bech32_prefix)
            .map(|result| {
                result
                    .iter()
                    .map(|key| {
                        let (name, _) = <(String, String)>::from_slice(key.as_bytes())?;
                        Ok(name)
                    })
                    .collect::<StdResult<String>>()
                    // and then append bech32 prefix
                    .map(|name| format!("{}.{}", name, bech32_prefix))
            })
            .collect::<StdResult<_>>()?,
        primary_name,
    })
}

fn query_primary_name(deps: Deps, address: String) -> StdResult<PrimaryNameResponse> {
    let primary_name = PRIMARY_NAME.may_load(deps.storage, address)?;
    match primary_name {
        Some(name) => Ok(PrimaryNameResponse { name }),
        None => Err(cosmwasm_std::StdError::NotFound {
            kind: "PrimaryName".to_string(),
        }),
    }
}

fn query_addresses(deps: Deps, _env: Env, name: String) -> StdResult<AddressesResponse> {
    Ok(AddressesResponse {
        addresses: records()
            .prefix(&name)
            .range(deps.storage, None, None, Ascending)
            .collect::<StdResult<Vec<_>>>()?,
    })
}

fn query_address(
    deps: Deps,
    _env: Env,
    name: String,
    bech32_prefix: String,
) -> StdResult<AddressResponse> {
    Ok(AddressResponse {
        address: records().load(deps.storage, (&name, &bech32_prefix))?,
    })
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

fn query_address_by_icns(deps: Deps, icns: String) -> StdResult<AddressByIcnsResponse> {
    let split: Vec<&str> = icns.split('.').collect();

    // check if split length is 2
    if split.len() != 2 {
        return Err(StdError::generic_err("Invalid ICNS"));
    }

    let name = split[0];
    let bech32_prefix = split[1];

    Ok(AddressByIcnsResponse {
        bech32_address: records().load(deps.storage, (name, bech32_prefix))?,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // No state migrations performed, just returned a Response
    Ok(Response::default())
}
