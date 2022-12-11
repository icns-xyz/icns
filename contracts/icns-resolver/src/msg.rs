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
pub enum AddressHash {
    Cosmos,
    Ethereum,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get Config of the Resolver contract.
    /// This includes the Name-nft contract address.
    #[returns(Config)]
    Config {},

    /// Returns list of tuple consisted of (bech32_prefix, bech32_address) for the given ICNS name.
    #[returns(AddressesResponse)]
    Addresses { name: String },

    /// Returns the bech32 address set for the given name and bech32 prefix.
    /// Returns when address does not exist.
    #[returns(AddressResponse)]
    Address { name: String, bech32_prefix: String },

    /// Get names bound to an address
    /// only returns name itself, not full icns name
    #[returns(NamesResponse)]
    Names { address: String },

    /// Returns list of full icns name (e.g alice.osmo, alice.juno) given bech32 address
    #[returns(IcnsNamesResponse)]
    IcnsNames { address: String },

    /// An address and hold multiple names, this query returns
    /// their primary name.
    #[returns(PrimaryNameResponse)]
    PrimaryName { address: String },

    /// Returns list of admin queried from the name-nft contract.
    #[returns(AdminResponse)]
    Admin {},

    /// Returns bech32 addresses for the given full ICNS name.
    #[returns(AddressByIcnsResponse)]
    AddressByIcns { icns: String },
}

#[cw_serde]
pub struct NamesResponse {
    pub names: Vec<String>,
    pub primary_name: String,
}

#[cw_serde]
pub struct IcnsNamesResponse {
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
    // vector of tuple of (bech32 prefix, address)
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
