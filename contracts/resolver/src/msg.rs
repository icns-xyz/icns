use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Uint128};

use crate::state::Config;
#[cw_serde]
pub struct InstantiateMsg {
    pub name_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetRecord {
        name: String,
        bech32_prefix: String,
        adr36_info: Adr36Info,
        signature_salt: Uint128,
    },
    SetPrimary {
        name: String,
        bech32_address: String,
    },
}

#[cw_serde]
pub struct Adr36Info {
    pub bech32_address: String,
    pub address_hash: AddressHash,
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

    #[returns(AddressesResponse)]
    Addresses { name: String },

    #[returns(AddressResponse)]
    Address { name: String, bech32_prefix: String },

    /// An address and hold multiple names, this query returns
    /// their primary name.
    #[returns(PrimaryNameResponse)]
    PrimaryName { address: String },

    #[returns(AdminResponse)]
    Admin {},
}

#[cw_serde]
pub struct PrimaryNameResponse {
    pub name: String,
}

#[cw_serde]
pub struct AdminResponse {
    pub admins: Vec<String>,
}

#[cw_serde]
pub struct AddressesResponse {
    // tuple of (bech32 prefix, address)
    pub addresses: Vec<(String, String)>,
}

#[cw_serde]
pub struct AddressResponse {
    pub address: String,
}
