use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    UpdateOwner {
        owner: HumanAddr,
    },
    UpdateHouseContract {
        house_contract: HumanAddr,
    },
    UpdateName {
        name: String,
    },
    UpdateDescription {
        description: String,
    },
    UpdateMinBetAmount {
        amount: u64,
    },
    UpdateMaxBetAmount {
        amount: u64,
    },
    UpdateMaxBetRate {
        rate: u8,
    },
    UpdateHouseFee {
        house_fee: u64,
    },
    UpdateBetAmountSum {
        amount: Uint128,
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
