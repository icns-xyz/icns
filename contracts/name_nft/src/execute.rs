use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::state::{Config, CONFIG};

pub fn set_admin(admin: &str, deps: DepsMut) -> Result<Response, cw721_base::ContractError> {
    CONFIG.update(deps.storage, |config| -> StdResult<_> {
        Ok(Config {
            admin: deps.api.addr_validate(admin)?,
            ..config
        })
    })?;
    Ok(Response::new()
        .add_attribute("method", "set_admin")
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
