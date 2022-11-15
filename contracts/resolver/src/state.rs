use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admins: Vec<Addr>,
    pub registry_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

// map of (username, bech32 prefix) -> address
pub const ADDRESSES: Map<(String, String), String> = Map::new("addresses");