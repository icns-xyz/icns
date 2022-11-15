use cw2::set_contract_version;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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

      // TODO: add duplication check
      let mut admin_addrs = Vec::new();
      for admin in msg.admins {
          admin_addrs.push(deps.api.addr_validate(&admin)?);
      }
  
      let registry_addr = deps.api.addr_validate(&msg.registry_address)?;
  
      let cfg = Config {
          admins: admin_addrs,
          registry_address: registry_addr,
      };
      CONFIG.save(deps.storage, &cfg)?;
  
      Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn execute_set_address(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_name: String,
    addresses: Vec<(String, String)>,
) -> Result<Response, ContractError> {
    // check if the msg sender is a registrar or admin. If not, return err
    let cfg = CONFIG.load(deps.storage)?;

    // TODO: make this into method
    let authorized = cfg.admins.iter().any(|a| a.as_ref() == info.sender.as_ref()) ||
        cfg.registry_address == info.sender.as_ref();

    if !authorized {
        return Err(ContractError::Unauthorized {});
    }

    for (bech32_prefix, address) in addresses {
        ADDRESSES.save(deps.storage, (user_name.clone(), bech32_prefix.clone()), &address)?;
    }

    Ok(Response::default())   
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
