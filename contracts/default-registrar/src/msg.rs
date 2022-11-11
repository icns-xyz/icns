use cosmwasm_std::{Addr};

use cosmwasm_schema::{cw_serde};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub registry: String,
    pub admin_addr: String, 
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Register {
        user_name: String,
        owner: Addr,
        // tuple of (cointype, address)
        addresses: Vec<(i32, String)>,
    }
}
