pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;


pub use crate::error::ContractError;
pub mod entry {
    use cosmwasm_std::entry_point;
    use cosmwasm_std::Order::Ascending;

    use cosmwasm_std::{
        from_binary, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
        StdResult, WasmQuery,
    };
    use cw2::set_contract_version;
    use subtle_encoding::bech32;
    // use cw2::set_contract_version;

    use icns_name_nft::msg::{QueryMsg as QueryMsgName};
    use cw721::OwnerOfResponse;
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, GetAddressResponse, GetAddressesResponse, InstantiateMsg, QueryMsg};
    use crate::state::{Config, ADDRESSES, CONFIG};

    // version info for migration info
    const CONTRACT_NAME: &str = "crates.io:resolver";
    const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let name_address = deps.api.addr_validate(&msg.name_address)?;
    
        let cfg = Config {
            name_address: name_address,
        };
        CONFIG.save(deps.storage, &cfg)?;
    
        Ok(Response::default())
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SetRecord { user_name, addresses } => execute_set_record(deps, env, info, user_name, addresses),
        }
    }

    pub fn execute_set_record(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        user_name: String,
        addresses: Vec<(String, String)>,
    ) -> Result<Response, ContractError> {
        // check if the msg sender is a registrar or admin. If not, return err
        let is_admin = is_admin(deps.as_ref(), info.sender.to_string())?;
        let is_owner_nft = is_owner(deps.as_ref(), user_name.clone(), info.sender.to_string())?;

        // if the sender is neither a registrar nor an admin, return error
        if !is_admin && !is_owner_nft {
            return Err(ContractError::Unauthorized {});
        }

        // do a sanity check on the given addresses for the different bech32 prefixes
        // We do two checks here:
        // 1. Check that the given addresses are valid bech32 addresses
        // 2. Check if they match the given prefixes
        // if the sanity check fails, we return an error
        for (prefix, address) in addresses.iter() {
            let prefix_decoded = bech32::decode(address)
                .map_err(|_| ContractError::Bech32DecodingErr {
                    addr: address.to_string(),
                })?
                .0;
            if !prefix.eq(&prefix_decoded) {
                return Err(ContractError::Bech32PrefixMismatch {
                    prefix: prefix.to_string(),
                    addr: address.to_string(),
                });
            }
        }

        // check if the user_name is already registered
        let user_name_exists = query_addresses(deps.as_ref(), env, user_name.clone())?;
        if !user_name_exists.addresses.is_empty() {
            return Err(ContractError::UserAlreadyRegistered { name: user_name });
        }

        for (bech32_prefix, address) in addresses {
            ADDRESSES.save(
                deps.storage,
                (user_name.clone(), bech32_prefix.clone()),
                &address,
            )?;
        }

        Ok(Response::default())
    }

    pub fn is_admin(deps: Deps, address: String) -> Result<bool, ContractError> {
        let cfg = CONFIG.load(deps.storage)?;
        let name_address = cfg.name_address;

        // query admin from icns-name-nft contract
        let query_msg = QueryMsgName::Admin {};
        let res: String = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: name_address.to_string(),
            msg: to_binary(&query_msg)?,
        }))?;

        Ok(res.eq(&address))
    }

    pub fn is_owner(deps: Deps, username: String, sender: String) -> Result<bool, ContractError> {
        let response = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: CONFIG.load(deps.storage)?.name_address.to_string(),
            msg: to_binary(&QueryMsgName::OwnerOf {token_id: username, include_expired: None})?,
        })).map(|res| from_binary(&res).unwrap());

        match response {
            Ok(OwnerOfResponse {owner, ..}) => Ok(owner.eq(&sender)),
            Err(_) => Ok(false),
        }
    }



    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
            QueryMsg::GetAddreses { user_name } => to_binary(&query_addresses(deps, env, user_name)?),
            QueryMsg::GetAddress {
                user_name,
                bec32_prefix,
            } => to_binary(&query_address(deps, env, user_name, bec32_prefix)?),
        }
    }

    fn query_addresses(deps: Deps, _env: Env, name: String) -> StdResult<GetAddressesResponse> {
        let addresses = ADDRESSES
            .prefix(name)
            .range(deps.storage, None, None, Ascending)
            .collect::<StdResult<Vec<_>>>()?;
        let resp = GetAddressesResponse {
            addresses: addresses,
        };

        Ok(resp)
    }

    fn query_address(
        deps: Deps,
        _env: Env,
        user_name: String,
        bech32_prefix: String,
    ) -> StdResult<GetAddressResponse> {
        let address = ADDRESSES.may_load(deps.storage, (user_name, bech32_prefix))?;
        let resp = GetAddressResponse { address };

        Ok(resp)
    }
}
#[cfg(test)]
mod tests;