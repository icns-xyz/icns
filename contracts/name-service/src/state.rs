use std::collections::BinaryHeap;

use cw_utils::{Duration, Expiration};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};
use cw_storage_plus::{Item, Map};
// impl AdminList {
//     /// returns true if the address is a registered admin
//     pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
//         let addr = addr.as_ref();
//         self.admins.iter().any(|a| a.as_ref() == addr)
//     }

//     /// returns true if the address is a registered admin and the config is mutable
//     pub fn can_modify(&self, addr: &str) -> bool {
//         self.mutable && self.is_admin(addr)
//     }
// }

pub static ADDRESS_RESOLVER_KEY: &[u8] = b"addressresolver";
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    // Denom for all protocol transactions
    pub admins: Vec<Addr>,
    pub registrar_addresses: Vec<Addr>,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Record {
    pub user_name: String,
    pub owner: Addr,
    pub addresses: Vec<(i32, String)>,
}

pub fn resolver(storage: &mut dyn Storage) -> Bucket<Record> {
    bucket(storage, ADDRESS_RESOLVER_KEY)
}

pub fn resolver_read(storage: &dyn Storage) -> ReadonlyBucket<Record> {
    bucket_read(storage, ADDRESS_RESOLVER_KEY)
}

// map of username -> owner address
pub const OWNER: Map<String, Addr> = Map::new("owner");
pub const ADDRESSES: Map<(String, i32), Addr> = Map::new("addresses");