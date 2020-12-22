use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CanonicalAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
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
