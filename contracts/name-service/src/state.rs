use std::collections::BinaryHeap;

use cw_utils::{Duration, Expiration};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};


pub static ADDRESS_RESOLVER_KEY: &[u8] = b"addressresolver";
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    // Denom for all protocol transactions
    pub admins: Vec<Addr>
,}

pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Resolver {
    pub resolver: String,
}

pub fn resolver(storage: &mut dyn Storage) -> Bucket<Resolver> {
    bucket(storage, ADDRESS_RESOLVER_KEY)
}

pub fn resolver_read(storage: &dyn Storage) -> ReadonlyBucket<Resolver> {
    bucket_read(storage, ADDRESS_RESOLVER_KEY)
}
