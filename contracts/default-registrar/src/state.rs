use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};

use cw_storage_plus::{Item};
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    // name nft contract address to send msg to
    pub name_nft_contract: Addr,
    pub resolver: Addr,

    // operator defines the pub key of the operator who can call this contract
    pub operators: Vec<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
