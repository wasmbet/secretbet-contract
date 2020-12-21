use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket,
    ReadonlyPrefixedStorage, ReadonlySingleton, Singleton,
};

const CONFIG_KEY: &[u8] = b"config";
const PREFIX_BALANCE: &[u8] = b"balance";

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub owner: CanonicalAddr,
    pub pool_token: CanonicalAddr,
    pub decasino_token: CanonicalAddr,
    pub game_contracts: Vec<CanonicalAddr>,
    pub pool: Uint128,
}

// meta is the token definition as well as the total_supply
pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

/// balances are state of the erc20 tokens
pub fn balances(storage: &mut dyn Storage) -> Bucket<Uint128> {
    bucket(storage, PREFIX_BALANCE)
}

/// balances are state of the erc20 tokens (read-only version for queries)
pub fn balances_read(storage: &dyn Storage) -> ReadonlyBucket<Uint128> {
    bucket_read(storage, PREFIX_BALANCE)
}

pub fn balances_prefix_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, PREFIX_BALANCE)
}