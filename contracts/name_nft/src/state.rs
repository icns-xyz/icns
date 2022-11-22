use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub admins: Vec<Addr>,
    pub transferrable: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
