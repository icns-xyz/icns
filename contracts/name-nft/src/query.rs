use crate::{
    msg::{AdminResponse, IsAdminResponse, TransferrableResponse},
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

pub fn is_admin(deps: Deps, addr: String) -> StdResult<IsAdminResponse> {
    let admins = CONFIG.load(deps.storage)?.admins;

    // iterate over admins and convert to string vector
    let mut is_admin = false;
    for admin in admins {
        // TODO: double check this is correctly comparable
        if admin == addr {
            is_admin = true;
            break;
        }
    }
    Ok(IsAdminResponse { is_admin })
}

pub fn transferrable(deps: Deps) -> StdResult<TransferrableResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(TransferrableResponse {
        transferrable: config.transferrable,
    })
}
