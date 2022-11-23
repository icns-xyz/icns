use crate::{
    msg::{AdminResponse, TransferrableResponse},
    state::CONFIG,
};
use cosmwasm_std::{Deps, StdResult};

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let admins = CONFIG
        .load(deps.storage)?
        .admins
        .into_iter()
        .map(String::from)
        .collect();

    Ok(AdminResponse { admins })
}

pub fn transferrable(deps: Deps) -> StdResult<TransferrableResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(TransferrableResponse {
        transferrable: config.transferrable,
    })
}
