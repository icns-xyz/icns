use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response, StdResult};
use cw721_base::ContractError;

use crate::state::{Config, CONFIG};

pub fn set_admin(
    admin: &str,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, cw721_base::ContractError> {
    check_is_admin(deps.as_ref(), info)?;

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
    info: MessageInfo,
) -> Result<Response, cw721_base::ContractError> {
    check_is_admin(deps.as_ref(), info)?;
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

pub fn check_is_admin(deps: Deps, info: MessageInfo) -> Result<(), cw721_base::ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.admin != info.sender {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}
