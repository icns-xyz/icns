use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};

use cw_storage_plus::{Item};
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    // registry address to send msg to
    pub registry: Addr,

    // admin pub key defines the pub key of the admin who can call this contract
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
