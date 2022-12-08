use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct Config {
    pub name_address: Addr, // NL: Document this.
}

pub const CONFIG: Item<Config> = Item::new("config");

pub struct RecordIndexes<'a> {
    pub address: MultiIndex<'a, String, String, String>,
}

impl<'a> IndexList<String> for RecordIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<String>> + '_> {
        let v: Vec<&dyn Index<String>> = vec![&self.address];
        Box::new(v.into_iter())
    }
}

// indexed map of (username, bech32 prefix) -> address
pub fn records<'a>() -> IndexedMap<'a, (&'a str, &'a str), String, RecordIndexes<'a>> {
    let indexes = RecordIndexes {
        address: MultiIndex::new(
            |_pk, addr: &String| addr.clone(),
            "records",
            "records__address",
        ),
    };
    IndexedMap::new("records", indexes)
}
// map of bech32 address -> user name
pub const PRIMARY_NAME: Map<String, String> = Map::new("primary_name");

// NL: We store the used signatures here to make sure they can't be reused. Used in adr36_verification()
// NL: document the meaning of the keys and valies
pub const SIGNATURE: Map<&[u8], bool> = Map::new("signature");
