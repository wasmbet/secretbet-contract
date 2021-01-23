use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    UpdateConfig {
        owner: HumanAddr,
        house_contract: HumanAddr,
        name: String,
        description: String,
    },
    UpdateParams {
        house_fee: Decimal,
        max_bet_amount: Uint128,
        min_bet_amount: Uint128,
        max_bet_rate: Decimal,
        min_bet_rate: Decimal,
    },
    Bet {
        bet_amount: Uint128,
        prediction_number: u8,
        position: bool,
    },
    Result { },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}
