use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};

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
    },
    SetPrimary {
        name: String,
        bech32_address: String,
    },
    RemoveRecord {
        name: String,
        bech32_address: String,
    },
}

#[cw_serde]
pub struct Adr36Info {
    pub signer_bech32_address: String,
    pub address_hash: AddressHash,
    pub pub_key: Binary,
    pub signature: Binary,
    pub signature_salt: Uint128,
}

#[cw_serde]
#[serde(rename = "name")]
pub enum AddressHash {
    Cosmos,
    Ethereum,
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

    /// Get names bound to an address
    #[returns(NamesResponse)]
    Names { address: String },

    /// An address and hold multiple names, this query returns
    /// their primary name.
    #[returns(PrimaryNameResponse)]
    PrimaryName { address: String },

    #[returns(AdminResponse)]
    Admin {},

    #[returns(AddressByIcnsResponse)]
    AddressByIcns { icns: String },
}

#[cw_serde]
pub struct NamesResponse {
    pub names: Vec<String>,
    pub primary_name: String,
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

#[cw_serde]
pub struct AddressByIcnsResponse {
    pub bech32_address: String,
}

#[cw_serde]
pub struct MigrateMsg {}
