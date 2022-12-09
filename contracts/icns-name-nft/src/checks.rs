use cosmwasm_std::{Addr, Deps};

use crate::{error::ContractError, state::CONFIG};

// check_admin checks if the sender is an admin, if not returns error
pub fn check_admin(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    for admin in config.admins {
        if admin == *sender {
            return Ok(());
        }
    }

    Err(cw721_base::ContractError::Unauthorized {}.into())
}

// is_admin checks if the given sender is an admin, if not returns false
pub fn is_admin(deps: Deps, sender: &Addr) -> Result<bool, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    for admin in config.admins {
        if admin == *sender {
            return Ok(true);
        }
    }

    Ok(false)
}

// is_transferrable checks if the contract is transferrable, if not returns false
pub fn is_transferrable(deps: Deps) -> Result<bool, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !config.transferrable {
        return Ok(false);
    }

    Ok(true)
}

// validate_name returns error if the name contains a dot.
// This is to prevent the name containing a dot, which is used to separate the name and the bech32 prefix.
pub fn validate_name(name: &str) -> Result<(), ContractError> {
    if name.contains('.') {
        return Err(ContractError::InvalidName {});
    }

    Ok(())
}
