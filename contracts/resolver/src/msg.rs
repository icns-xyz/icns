use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Config;
#[cw_serde]
pub struct InstantiateMsg {
    pub registry_address: String,
}

#[cw_serde]
#[serde(untagged)]
pub enum ExecuteMsg {
    SetRecord {
        user_name: String,
        // tuple of (bech32 prefix, address)
        addresses: Vec<(String, String)>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},

    #[returns(GetAddressesResponse)]
    GetAddreses { user_name: String },

    #[returns(GetAddressResponse)]
    GetAddress {
        user_name: String,
        bec32_prefix: String,
    },
}

#[cw_serde]
pub struct GetAddressesResponse {
    // tuple of (bech32 prefix, address)
    pub addresses: Vec<(String, String)>,
}

#[cw_serde]
pub struct GetAddressResponse {
    pub address: Option<String>,
}
