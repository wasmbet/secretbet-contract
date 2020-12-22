use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket,
    ReadonlyPrefixedStorage, ReadonlySingleton, Singleton,
};

const CONFIG_KEY: &[u8] = b"config";
const PREFIX_GAME: &[u8] = b"game";

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub owner: CanonicalAddr,
    pub house_contract: CanonicalAddr,
    pub name: String,
    pub description: String,
    pub min_bet_amount: u64,
    pub max_bet_amount: u64,
    pub max_bet_rate: u8,
    pub house_fee: u64,
    pub bet_amount_sum: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Game {
    pub user: CanonicalAddr,
    pub bet_amount: Uint128,
    pub prediction_number: u8,
    pub lucky_number: u8,
    pub position: bool,
    pub result: bool,
    pub payout: Uint128,
    pub time: u64,
    pub block_height: u64,
}


// meta is the token definition as well as the total_supply
pub fn config(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

/// balances are state of the erc20 tokens
pub fn games(storage: &mut dyn Storage) -> Bucket<Game> {
    bucket(storage, PREFIX_GAME)
}

/// balances are state of the erc20 tokens (read-only version for queries)
pub fn games_read(storage: &dyn Storage) -> ReadonlyBucket<Game> {
    bucket_read(storage, PREFIX_GAME)
}

pub fn games_prefix_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, PREFIX_GAME)
}