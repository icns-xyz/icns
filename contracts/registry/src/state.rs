use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    // Denom for all protocol transactions
    pub admins: Vec<Addr>,
    pub registrar_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

// map of username -> resolver contract address
pub const RESOLVER: Map<String, Addr> = Map::new("resolver");
pub const ADDRESSES: Map<(String, i32), String> = Map::new("addresses");