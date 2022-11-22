use itertools::Itertools;

use cosmwasm_std::{attr, Addr, DepsMut, Response, StdResult};

use crate::state::{Config, CONFIG, RESOLVERS};

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

pub fn update_resolvers(
    add: Vec<Addr>,
    remove: Vec<Addr>,
    deps: DepsMut,
) -> Result<Response, cw721_base::ContractError> {
    let attributes = vec![
        attr("action", "update_resolvers"),
        attr("added", add.len().to_string()),
        attr("removed", remove.len().to_string()),
    ];

    let resolvers = RESOLVERS.may_load(deps.storage)?.unwrap_or_default();

    let result: Vec<Addr> = vec![resolvers, add]
        .concat()
        .into_iter()
        .filter(|addr| !remove.contains(addr))
        .unique()
        .collect();

    RESOLVERS.save(deps.storage, &result)?;

    Ok(Response::new().add_attributes(attributes))
}
