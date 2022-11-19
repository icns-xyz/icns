use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Message type for `instantiate` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub registrar_address: String,

    /// name contract, an NFT contract that encodes name's ownership
    pub name_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetResolverAddress {
        user_name: String,
        resolver_address: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetResolverAddr { user_name: String },
    GetAddreses { user_name: String },
    GetAddress { user_name: String, coin_type: i32 },
    Config {},
    IsAdmin { address: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetResolverAddrResponse {
    pub resolver_addr: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetAddressesResponse {
    pub addresses: Vec<(i32, String)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetAddressResponse {
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct IsAdminResponse {
    pub is_admin: bool,
}
