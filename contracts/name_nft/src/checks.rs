use cosmwasm_std::{Addr, Deps};
use cw721_base::ContractError;

use crate::state::CONFIG;

pub fn check_transferrable(deps: Deps) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !config.transferrable {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn check_admin(deps: Deps, sender: &Addr) -> Result<(), cw721_base::ContractError> {
    let config = CONFIG.load(deps.storage)?;

    for admin in config.admins {
        if admin == *sender {
            return Ok(());
        }
    }

    Err(cw721_base::ContractError::Unauthorized {})
}
