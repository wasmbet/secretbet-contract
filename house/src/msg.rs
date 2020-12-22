use cosmwasm_std::{Binary, HumanAddr, StdError, StdResult, Uint128};
use cw20::{Cw20CoinHuman, Expiration, Cw20ReceiveMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    pub initial_balances: Vec<Cw20CoinHuman>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// Transfer is a base message to move tokens to another account without triggering actions
    UpdateOwner {
        owner: HumanAddr,
    },
    UpdatePoolTokenContract {
        pool_token: HumanAddr,
    },
    UpdateDecasinoTokenContract {
        decasino_token: HumanAddr,
    },
    AddGameContract {
        game_contract: HumanAddr,
    },
    RemoveGameContract {
        game_contract: HumanAddr,
    },
    Deposit {},
    Receive(Cw20ReceiveMsg),
    Result {
        result: bool,
        bet_amount: Uint128,
        prize_amount: Uint128,
        winner: HumanAddr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Withdraw {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
