use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use crate::state::Config;
#[cw_serde]
pub struct InstantiateMsg {
    pub name_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetRecord {
        user_name: String,
        bech32_prefix: String,
        adr36_info: Adr36Info,
        replace_primary_if_exists: bool,
        signature_salt: u128,
    },
}

#[cw_serde]
pub struct Adr36Info {
    pub bech32_address: String,
    pub address_hash: AddressHash,
    //
    pub pub_key: Binary,
    pub signature: Binary,
}

#[cw_serde]
pub enum AddressHash {
    SHA256,
    Keccak256,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},

    #[returns(GetAddressesResponse)]
    GetAddresses { user_name: String },

    #[returns(GetAddressResponse)]
    GetAddress {
        user_name: String,
        bech32_prefix: String,
    },

    #[returns(AdminResponse)]
    Admin {},
}

#[cw_serde]
pub struct AdminResponse {
    pub admins: Vec<String>,
}

#[cw_serde]
pub struct GetAddressesResponse {
    // tuple of (bech32 prefix, address)
    pub addresses: Vec<(String, String)>,
}

#[cw_serde]
pub struct GetAddressResponse {
    pub address: String,
}
