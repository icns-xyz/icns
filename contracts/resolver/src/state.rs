use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct Config {
    pub name_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

// map of (username, bech32 prefix) -> address
pub const ADDRESSES: Map<(String, String), String> = Map::new("addresses");

// map of address -> Vector of Address Infos
pub const REVERSE_RESOLVER: Map<String, Vec<AddressInfo>> = Map::new("reverse_resolver");

#[cw_serde]
pub struct AddressInfo {
    pub user_name: String,
    pub bech32_prefix: String,
    pub primary: bool,
}

pub const SIGNATURE: Map<&[u8], bool> = Map::new("signature");
