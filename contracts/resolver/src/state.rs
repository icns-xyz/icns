use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct Config {
    pub name_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub struct RecordIndexes<'a> {
    pub address: MultiIndex<'a, Addr,  F, String>,
}

impl<'a> IndexList<Addr> for RecordIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Addr>> + '_> {
        let v: Vec<&dyn Index<Addr>> = vec![&self.address];
        Box::new(v.into_iter())
    }
}

// indexed map of (username, bech32 prefix) -> address
pub fn records<'a>() -> IndexedMap<'a, (&'a str, &'a str), Addr, RecordIndexes<'a>> {
    let indexes = RecordIndexes {
        address: MultiIndex::new(
            |_pk, addr: &Addr| addr.clone(),
            "records",
            "records__address",
        ),
    };
    IndexedMap::new("records", indexes)
}

pub const PRIMARY_NAME: Map<Addr, String> = Map::new("primary_name");
#[cw_serde]
pub struct AddressInfo {
    pub name: String,
    pub bech32_prefix: String,
    pub primary: bool,
}

pub const SIGNATURE: Map<&[u8], bool> = Map::new("signature");
