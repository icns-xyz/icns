use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct Config {
    // name_nft address to send msg to
    pub name_nft: Addr,
    // verifiers defines the pub key of the verifiers
    pub verifiers: Vec<Vec<u8>>,
    // number of verification that needs to pass in order to mint name
    pub verification_threshold: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
