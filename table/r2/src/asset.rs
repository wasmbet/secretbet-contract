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
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

pub enum AssetInfo {
    CW20Token { contract_addr: HumanAddr },
    SNIP20Token { contract_addr: HumanAddr },
    NativeToken { denom: String },
}