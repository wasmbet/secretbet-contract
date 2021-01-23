use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage, Uint128};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket,
    ReadonlyPrefixedStorage, ReadonlySingleton, Singleton,
};

const CONFIG_KEY: &[u8] = b"config";
const PARAMS_KEY: &[u8] = b"params";
const STATE_KEY: &[u8] = b"state";
const PREFIX_GAME: &[u8] = b"bet";

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub owner: CanonicalAddr,
    pub house_contract: CanonicalAddr,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    pub min_bet_amount: Uint128,
    pub max_bet_amount: Uint128,
    pub max_bet_rate: Decimal,
    pub house_fee: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct State {
    pub cumulative_bet_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Bet {
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

// meta is the token definition as well as the total_supply
pub fn params(storage: &mut dyn Storage) -> Singleton<Params> {
    singleton(storage, PARAMS_KEY)
}

pub fn params_read(storage: &dyn Storage) -> ReadonlySingleton<Params> {
    singleton_read(storage, PARAMS_KEY)
}

// meta is the token definition as well as the total_supply
pub fn state(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}

/// balances are state of the erc20 tokens
pub fn bets(storage: &mut dyn Storage) -> Bucket<Bet> {
    bucket(storage, PREFIX_BET)
}

/// balances are state of the erc20 tokens (read-only version for queries)
pub fn bets_read(storage: &dyn Storage) -> ReadonlyBucket<Bet> {
    bucket_read(storage, PREFIX_BET)
}

pub fn bets_prefix_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, PREFIX_BET)
}