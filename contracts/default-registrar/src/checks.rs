use cosmwasm_std::{to_binary, Addr, Deps, QueryRequest, WasmQuery};
use icns_name_nft::msg::{AdminResponse, QueryMsg as NameNFTQueryMsg};

use crate::{state::CONFIG, ContractError};

pub fn check_send_from_admin(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let AdminResponse { admins } = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CONFIG.load(deps.storage)?.name_nft.to_string(),
        msg: to_binary(&NameNFTQueryMsg::Admin {})?,
    }))?;

    if !admins.contains(&sender.to_string()) {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}
