use cosmwasm_std::{Addr, Deps};
use cw721_base::{ContractError, MinterResponse};

use crate::{state::CONFIG, ICNSNameNFTContract};

pub fn check_transferrable(deps: Deps) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !config.transferrable {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn check_send_from_registrar(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let MinterResponse { minter } = ICNSNameNFTContract::default().minter(deps)?;
    if *sender != minter {
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

pub fn pass_any(checks: &[Result<(), ContractError>]) -> Result<(), ContractError> {
    if !checks.iter().any(|check| check.is_ok()) {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}
