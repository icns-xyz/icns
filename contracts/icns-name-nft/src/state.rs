use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    // list of admins
    pub admins: Vec<Addr>,
    // transferrable flag. If true nfts minted can be transferred to other accounts.
    // If false, transferred are not allowed.
    pub transferrable: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
