use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{CanonicalAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct HouseResponse {
    pub owner: CanonicalAddr,
    pub pool: Uint128,
    pub game_contracts: Vec<CanonicalAddr>,
}