use cosmwasm_std::{Addr, Deps};

use crate::{error::ContractError, state::CONFIG};

pub fn check_admin(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    for admin in config.admins {
        if admin == *sender {
            return Ok(());
        }
    }

    Err(cw721_base::ContractError::Unauthorized {}.into())
}

pub fn is_admin(deps: Deps, sender: &Addr) -> Result<bool, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    for admin in config.admins {
        if admin == *sender {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn is_transferrable(deps: Deps) -> Result<bool, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !config.transferrable {
        return Ok(false);
    }

    Ok(true)
}

pub fn validate_name(name: &str) -> Result<(), ContractError> {
    if name.contains('.') {
        return Err(ContractError::InvalidName {});
    }

    Ok(())
}
