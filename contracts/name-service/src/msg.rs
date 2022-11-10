use cosmwasm_std::{Addr, Uint128};
use cw_utils::Duration;
use cosmwasm_schema::{cw_serde, QueryResponses};
use std::collections::HashMap;

use cw_storage_plus::{Map};

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
    GetRecord { name: String },
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ResolveRecordResponse {
    pub address: Option<String>,
}
