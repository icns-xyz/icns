use crate::{
    msg::{AdminResponse, TransferrableResponse},
    state::CONFIG,
};
use cosmwasm_std::{Deps, StdResult};

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(AdminResponse {
        admin: config.admin.to_string(),
    })
}

pub fn transferable(deps: Deps) -> StdResult<TransferrableResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(TransferrableResponse {
        transferrable: config.transferable,
    })
}
