use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::{state::{Config, CONFIG}, checks::check_admin};


pub fn add_admin(admin: &str, deps: DepsMut) -> Result<Response, cw721_base::ContractError> {
    // check that admin does not already exist
    let config = CONFIG.load(deps.storage)?;
    for existing_admin in config.admins {
        if existing_admin == deps.api.addr_validate(admin)? {
            return Err(cw721_base::ContractError::Unauthorized {  });
        }
    }

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        let mut admins = config.admins;
        admins.push(deps.api.addr_validate(admin)?);
        Ok(Config { admins, ..config })
    })?;
    Ok(Response::new()
        .add_attribute("method", "add_admin")
        .add_attribute("admin", admin))
}

pub fn remove_admin(admin: &str, deps: DepsMut) -> Result<Response, cw721_base::ContractError> {
    let admin_addr = deps.api.addr_validate(admin)?;
    // check that admin exists
    check_admin(deps.as_ref(), &admin_addr)?;

    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        let mut admins = config.admins;
        admins.retain(|x| x != &admin_addr);
        Ok(Config { admins, ..config })
    })?;
    
    Ok(Response::new()
        .add_attribute("method", "remove_admin")
        .add_attribute("admin", admin))
}

pub fn set_transferrable(
    transferrable: bool,
    deps: DepsMut,
) -> Result<Response, cw721_base::ContractError> {
    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            transferrable,
            ..config
        })
    })?;
    Ok(Response::new()
        .add_attribute("method", "set_transferrable")
        .add_attribute("transferrable", transferrable.to_string()))
}
