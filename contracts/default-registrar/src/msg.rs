use cosmwasm_schema::{cw_serde};

/// Message type for `instantiate` entry_point
// TODO: change this to array
#[cw_serde]
pub struct InstantiateMsg {
    pub registry: String,
    pub resolver: String,
    pub operator_addrs: Vec<String>, 
}

/// Message type for `execute` entry_point
#[cw_serde]
#[serde(untagged)]
pub enum ExecuteMsg {
    Register {
        user_name: String,
        owner: String,
        // tuple of (bech32 prefix, address)
        addresses: Vec<(String, String)>,
    }
}
