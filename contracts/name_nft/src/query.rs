use crate::{
    msg::{AdminResponse, TransferrableResponse},
    state::CONFIG,
};
use cosmwasm_std::{Deps, StdResult};

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let admins = CONFIG.load(deps.storage)?.admins;

    // iterate over admins and convert to string vector
    let mut admins_str = Vec::new();
    for admin in admins {
        admins_str.push(admin.to_string());
    }
    Ok(AdminResponse { admins: admins_str })
}

pub fn transferrable(deps: Deps) -> StdResult<TransferrableResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(TransferrableResponse {
        transferrable: config.transferrable,
    })
}
