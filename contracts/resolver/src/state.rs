use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary};
use cw_storage_plus::{Item, Map};
pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct Config {
    pub name_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

// map of (username, bech32 prefix) -> address
pub const ADDRESSES: Map<(String, String), String> = Map::new("addresses");
// map of address -> (username, bech32 prefix)
pub const REVERSE_RESOLVER: Map<String, (String, String)> = Map::new("reverse_resolver");

pub const SIGNATURE: Map<&[u8], bool> = Map::new("signature");