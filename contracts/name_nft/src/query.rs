use crate::{
    msg::{AdminResponse, TransferrableResponse, IsAdminResponse},
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

pub fn is_admin(deps: Deps, addr: String) -> StdResult<IsAdminResponse> {
    let admins = CONFIG.load(deps.storage)?.admins;

    // iterate over admins and convert to string vector
    let mut is_admin = false;
    for admin in admins {
        if admin.to_string() == addr {
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
