use crate::{
    msg::{AdminResponse, ResolversResponse, TransferrableResponse},
    state::{CONFIG, RESOLVERS},
};
use cosmwasm_std::{Deps, StdResult};

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(AdminResponse {
        admin: config.admin.to_string(),
    })
}

pub fn transferrable(deps: Deps) -> StdResult<TransferrableResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(TransferrableResponse {
        transferrable: config.transferrable,
    })
}

pub fn resolvers(deps: Deps) -> StdResult<ResolversResponse> {
    let resolvers = RESOLVERS.may_load(deps.storage)?.unwrap_or_default();
    Ok(ResolversResponse { resolvers })
}
