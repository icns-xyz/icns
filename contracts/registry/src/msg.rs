use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::{Config};


/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub registrar_address: String,

    /// name contract, an NFT contract that encodes name's ownership
    pub name_address: String,
}

#[cw_serde]
#[serde(untagged)]
pub enum ExecuteMsg {
    SetResolverAddress {
        user_name: String,
        resolver_address: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetResolverAddrResponse)]
    GetResolverAddr { user_name: String },

    #[returns(GetAddressesResponse)]
    GetAddreses { user_name: String },

    #[returns(GetAddressResponse)]
    GetAddress { user_name: String, coin_type: i32 },

    #[returns(Config)]
    Config {},

    #[returns(IsAdminResponse)]
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
