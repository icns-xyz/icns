use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


/// Message type for `instantiate` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub registrar_addresses: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetRecord {
        user_name: String,
        owner: Addr,
        // tuple of (cointype, address)
        addresses: Vec<(i32, String)>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // ResolveAddress returns the current address that the name resolves to
    GetOwner { user_name: String },
    GetAddreses {user_name: String},
    GetAddress {user_name: String, coin_type: i32},
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetOwnerResponse {
    pub owner: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAddressesResponse {
    pub addresses: Vec<(i32, String)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAddressResponse {
    pub address: Option<String>,
}