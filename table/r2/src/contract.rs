use cosmwasm_std::{
    attr, to_binary, Binary, Deps, DepsMut, Env, HandleResponse, HumanAddr, InitResponse, CanonicalAddr,
    MessageInfo, StdError, StdResult, Uint128, BankMsg, Coin, WasmQuery, QueryRequest, from_binary, CosmosMsg, WasmMsg, 
};

use cw20::{BalanceResponse, Cw20CoinHuman, Cw20ReceiveMsg, MinterResponse, TokenInfoResponse, Cw20QueryMsg, Cw20HandleMsg};

// use crate::enumerable::{query_all_accounts, query_all_allowances};
use crate::error::ContractError;
use crate::msg::{HandleMsg, HouseMsg, InitMsg, QueryMsg};
use crate::response::{ConfigResponse};
use crate::state::{games, games_read, config, config_read, Config, Game};
use crate::rand::Prng;
use sha2::{Digest, Sha256};


pub fn init(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    let cfg = Config {
        owner: deps.api.canonical_address(&_info.sender)?,
        house_contract: CanonicalAddr::default(),
        name: String::default(),
        description: String::default(),
        min_bet_amount: 0,
        max_bet_amount: 0,
        max_bet_rate: 0,
        house_fee: 0,
        bet_amount_sum: Uint128(0),
    };
    config(deps.storage).save(&cfg)?;
    Ok(InitResponse::default())
}

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        HandleMsg::UpdateOwner { owner } => handle_update_owner(deps, env, info, owner),
        HandleMsg::UpdateHouseContract { house_contract } => handle_update_house_contract(deps, env, info, house_contract),
        HandleMsg::UpdateName { name } => handle_update_name(deps, env, info, name),
        HandleMsg::UpdateDescription { description } => handle_update_description(deps, env, info, description),
        HandleMsg::UpdateMinBetAmount { amount } => handle_update_min_bet_amount(deps, env, info, amount),
        HandleMsg::UpdateMaxBetAmount { amount } => handle_update_max_bet_amount(deps, env, info, amount),
        HandleMsg::UpdateMaxBetRate { rate } => handle_update_max_bet_rate(deps, env, info, rate),
        HandleMsg::UpdateHouseFee { house_fee } => handle_update_house_fee(deps, env, info, house_fee),
        HandleMsg::UpdateBetAmountSum { amount } => handle_update_bet_amount_sum(deps, env, info, amount),
        HandleMsg::Bet { bet_amount, prediction_number, position } => handle_bet(deps, env, info, bet_amount, prediction_number, position),
        HandleMsg::Result { } => handle_result(deps, env, info),
    }
}

pub fn handle_update_owner(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.owner = deps.api.canonical_address(&owner)?;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "updateOwner"),
            attr("owner", owner),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_house_contract(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    house_contract: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.house_contract = deps.api.canonical_address(&house_contract)?;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateHouseContract"),
            attr("house_contract", house_contract),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_name(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.name = name;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateName"),
            attr("name", name),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_description(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    description: String,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.description = description;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateDescription"),
            attr("description", description),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_min_bet_amount(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u64,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.min_bet_amount = amount;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateMinBetAmount"),
            attr("min_bet_amount", amount),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_max_bet_amount(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u64,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.max_bet_amount = amount;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateMaxBetAmount"),
            attr("max_bet_amount", amount),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_max_bet_rate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rate: u8,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.max_bet_rate = rate;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateMaxBetRate"),
            attr("max_bet_rate", rate),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_house_fee(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    house_fee: u64,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.house_fee = house_fee;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateHouseFee"),
            attr("house_fee", house_fee),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_bet_amount_sum(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u64,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.bet_amount_sum = Uint128::from(amount);
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "UpdateBetAmountSum"),
            attr("bet_amount_sum", amount),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_bet(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    bet_amount: u64,
    prediction_number: u8,
    position: bool,
) -> Result<HandleResponse, ContractError> {
    
    let mut cfg = config_read(deps.storage).load()?;
    let sender_raw = deps.api.canonical_address(&info.sender)?;

    let mut games = games(deps.storage);

    // 이전 게임이 진행중인지 체크하는 로직 추가할 예정
    // let before = games.may_load(sender_raw.as_slice());
    // let coin = match before {
    //     Ok(coin) => coin,
    //     Err(_err) => return Err(ContractError::InvalidZeroAmount {}),
    // };

    // match before {
    //     Ok(before) => {
    //         if before.
    //     }
    //     Err(before) => {

    //     }
    // };
    

    //1. prediction check
    if position {
        if prediction_number < 2 || prediction_number > 58 {
            return Err(ContractError::GenericErr{
                msg: "prediction number, 2~58".to_string(),
            });
        }
    } else {
        if prediction_number < 1 || prediction_number > 57 {
            return Err(ContractError::GenericErr{
                msg: "prediction number, 1~57".to_string(),
            });
        }
    }

    let payment = info
        .sent_funds
        .iter()
        .find(|x| x.denom == "uscrt")
        .ok_or_else(|| ContractError::EmptyBalance {
            denom: "uscrt".to_string(),
        });
    
    let coin = match payment {
        Ok(coin) => coin,
        Err(_err) => return Err(ContractError::InvalidZeroAmount {}),
    };

    let house_config = query_house_info()?;

    // 여기서 house Contract 로 쿼리날려서 배팅초과인지 체크

    let game = Game {
        user: deps.api.canonical_address(&info.sender)?,
        bet_amount: coin.amount,
        prediction_number: prediction_number,
        lucky_number: 0,
        position: position,
        result: false,
        payout: calc_payout(prediction_number, position, bet_amount, cfg.house_fee)?,
        time: env.block.time,
        block_height: env.block.height,
    };

    games.save(sender_raw.as_slice(), &game)?;

    cfg.bet_amount_sum = cfg.bet_amount_sum + Uint128(bet_amount as u128);
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "Bet"),
            attr("bet_amount_sum", true),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_result(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<HandleResponse, ContractError> {

    let mut cfg = config_read(deps.storage).load()?;
    let sender_raw = deps.api.canonical_address(&info.sender)?;

    let mut games = games(deps.storage);
    let game = games.may_load(sender_raw.as_slice()).unwrap_or_default();

    // 다음 블록이 안나왔을때, 이렇게 체크하는게 맞나?
    match game {
        Some(g) => {
            if g.block_height >= env.block.height {
                return Err(ContractError::NoResult {});
            } else if env.block.height - g.block_height != 1 {
                let refundMsg = BankMsg::Send {
                    from_address: env.contract.address.clone(),
                    to_address: info.sender.clone(),
                    amount: vec![Coin {
                        denom: "uscrt".to_string(),
                        amount: g.bet_amount,
                    }],
                }.into();

                games.remove(sender_raw.as_slice());
                
                let res = HandleResponse {
                    messages: vec![refundMsg],
                    attributes: vec![
                        attr("action", "Refund"),
                        attr("bet_amount_sum", true),
                    ],
                    data: None,
                };
                return Ok(res);
            } else {
                let mut rand_entropy: Vec<u8> = Vec::new();

                rand_entropy.extend(sender_raw.as_slice().to_vec());
                rand_entropy.extend(env.block.chain_id.as_bytes().to_vec());
                rand_entropy.extend(&env.block.height.to_be_bytes());
                rand_entropy.extend(&env.block.time.to_be_bytes());
                rand_entropy.extend(&env.block.time_nanos.to_be_bytes());
                rand_entropy = Sha256::digest(&rand_entropy).as_slice().to_vec();
                rand_entropy.extend_from_slice(&env.block.time.to_be_bytes());

                let mut rng: Prng = Prng::new(&sender_raw.as_slice().to_vec(), &rand_entropy);

                let lucky_number_u32 = rng.select_one_of(59);
                let lucky_number = lucky_number_u32 as u64;
            
                games.remove(sender_raw.as_slice());
            
                let result = check_result(g, lucky_number)?;

                let message = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: deps.api.human_address(&cfg.house_contract)?,
                    msg: to_binary(&HouseMsg::Result {
                        result: result,
                        bet_amount: g.bet_amount,
                        prize_amount: g.payout,
                        winner: info.sender,
                    })?,
                    send: vec![Coin {
                        denom: "uscrt".to_string(),
                        amount: g.bet_amount,
                    }],
                }).into();

                let attributes = vec![];

                if result {
                    attributes.push(attr("action", "win"));
                    attributes.push(attr("payout", g.payout));

                } else {
                    attributes.push(attr("action", "lose"));
                    attributes.push(attr("bet_amount", g.bet_amount));
                }

                let res = HandleResponse {
                    messages: vec![message],
                    attributes,
                    data: None,
                };
                return Ok(res);
            }
        },
        None => { return Err(ContractError::NoGame {}); },
    };
}

pub fn check_result(game: Game, lucky_number: u64) -> StdResult<bool> {
    match game.position {
        true => {
            if lucky_number > game.prediction_number as u64 {
                return Ok(true);
            } else {
                return Ok(false);
            };
        },
        false => {
            if lucky_number < game.prediction_number as u64 {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
    }


}

pub fn calc_payout(prediction_number: u8, position: bool, bet_amount: u64, house_fee: u64) -> StdResult<Uint128> {
    if position {
        let multiplier = (1000000 as u128- house_fee as u128)/(99 as u128-(prediction_number as u128*5/3));
        let payout = Uint128(bet_amount as u128 * multiplier/10000);
        return Ok(payout);
    } else {
        let multiplier = (1000000 as u128- house_fee as u128)/(99 as u128-(prediction_number as u128*5/3));
        let payout = Uint128(bet_amount as u128 * multiplier/10000);
        return Ok(payout);
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let info = config_read(deps.storage).load()?;
    let res = ConfigResponse {
        owner: info.owner,
        house_contract: info.house_contract,
        name: info.name,
        description: info.description,
        min_bet_amount: info.min_bet_amount,
        max_bet_amount: info.max_bet_amount,
        max_bet_rate: info.max_bet_rate,
        house_fee: info.house_fee,
        bet_amount_sum: info.bet_amount_sum,
    };
    Ok(res)
}

pub fn query_house_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let cfg = config_read(deps.storage).load()?;
    let res: TokenInfoResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: deps.api.human_address(&cfg.house_contract)?,
        msg: to_binary(&HouseQuery::Config{})?,
    }))?;
    Ok(res)

}

// #[cfg(test)]
// mod tests {
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{Api};

//     use super::*;

//     fn get_balance<T: Into<HumanAddr>>(deps: Deps, address: T) -> Uint128 {
//         query_balance(deps, address.into()).unwrap().balance
//     }

//     // fn get_casino_info(deps: Deps) -> CasinoInfoResponse {
//     //     config_read(deps).unwrap()
//     // }

//     fn get_token_info(deps: Deps) -> TokenInfoResponse {
//         query_token_info(deps).unwrap()
//     }

//     // this will set up the init for other tests
//     fn do_init_with_deposit(
//         deps: DepsMut,
//         addr: &HumanAddr,
//     ) -> TokenInfoResponse {
//         _do_init(
//             deps,
//             addr,
//         )
//     }

//     // this will set up the init for other tests
//     fn do_init(deps: DepsMut, addr: &HumanAddr) -> TokenInfoResponse {
//         _do_init(deps, addr)
//     }

//     // this will set up the init for other tests
//     fn _do_init(
//         mut deps: DepsMut,
//         addr: &HumanAddr,
//     ) -> TokenInfoResponse {
//         let init_msg = InitMsg {
//             initial_balances: vec![],
//         };
//         let info = mock_info(addr, &[]);
//         let env = mock_env();
//         let res = init(dup(&mut deps), env, info, init_msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         let meta = query_token_info(deps.as_ref()).unwrap();
//         assert_eq!(
//             meta,
//             TokenInfoResponse {
//                 name: "cpool".to_string(),
//                 symbol: "cool".to_string(),
//                 decimals: 18,
//                 total_supply: Uint128(0),
//             }
//         );
//         // assert_eq!(get_balance(deps.as_ref(), addr), amount);
//         meta
//     }

//     // TODO: replace this with deps.dup()
//     // after https://github.com/CosmWasm/cosmwasm/pull/620 is merged
//     fn dup<'a>(deps: &'a mut DepsMut<'_>) -> DepsMut<'a> {
//         DepsMut {
//             storage: deps.storage,
//             api: deps.api,
//             querier: deps.querier,
//         }
//     }

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies(&[]);
//         let amount = Uint128::from(11223344u128);
//         let init_msg = InitMsg {
//             initial_balances: vec![Cw20CoinHuman {
//                 address: HumanAddr("addr0000".to_string()),
//                 amount,
//             }],
//         };
//         let info = mock_info(&HumanAddr("creator".to_string()), &[]);
//         let env = mock_env();
//         let res = init(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         assert_eq!(
//             query_token_info(deps.as_ref()).unwrap(),
//             TokenInfoResponse {
//                 name: "cpool".to_string(),
//                 symbol: "cool".to_string(),
//                 decimals: 18,
//                 total_supply: amount,
//             }
//         );
//         assert_eq!(get_balance(deps.as_ref(), "addr0000"), Uint128(11223344));
//     }

//     #[test]
//     fn can_mint_by_deposit() {
//         let mut deps = mock_dependencies(&[]);

//         let genesis = HumanAddr::from("genesis");
//         let amount = Uint128(11223344);
//         let sender = HumanAddr::from("asmodat");
//         do_init_with_deposit(deps.as_mut(), &genesis);

//         // minter can mint coins to some winner
//         let prize = Uint128(1000000);
//         let msg = HandleMsg::Deposit {};

//         let info = mock_info(&sender, &[Coin{
//             denom: "uscrt".to_string(),
//             amount,
//         }]);
//         let env = mock_env();
//         let res = handle(deps.as_mut(), env, info, msg.clone()).unwrap();
//         assert_eq!(0, res.messages.len());
//         // assert_eq!(get_balance(deps.as_ref(), &genesis), Uint128(11234120));

//         // let casino = get_casino_info(deps.as_ref());
//         // assert_eq!(casino, CasinoInfoResponse{
//         //     owner: deps.api.canonical_address(&sender).unwrap(),
//         //     pool: Uint128(100),
//         //     game_contracts: vec![],
//         // });

//         // let token = get_token_info(deps.as_ref());
//         // assert_eq!(token, TokenInfoResponse{
//         //     name: "cpool".to_string(),
//         //     symbol: "cool".to_string(),
//         //     decimals: 18,
//         //     total_supply: Uint128(0),
//         // });

//         assert_eq!(get_balance(deps.as_ref(), &sender), prize);
//     }

//     #[test]
//     fn test_add_game_contract() {
//         let mut deps = mock_dependencies(&[]);
//         let genesis = HumanAddr::from("genesis");
//         let sender = HumanAddr::from("asmodat");
//         do_init_with_deposit(deps.as_mut(), &genesis);

//         let msg = HandleMsg::AddGameContract {
//             game_contract: HumanAddr::from("lucky"),
//         };

//         let info = mock_info(&genesis, &[Coin{
//             denom: "uscrt".to_string(),
//             amount: Uint128(1000000),
//         }]);
//         let env = mock_env();
//         handle(deps.as_mut(), env, info, msg.clone()).unwrap();

//         let casino = get_casino_info(deps.as_ref());
//         assert_eq!(casino, CasinoInfoResponse{
//             owner: deps.api.canonical_address(&sender).unwrap(),
//             pool: Uint128(100000),
//             game_contracts: vec![],
//         });
//     }

    
//     #[test]
//     fn test_remove_game_contract() {
//         let mut deps = mock_dependencies(&[]);
//         let genesis = HumanAddr::from("genesis");
//         let sender = HumanAddr::from("asmodat");
//         do_init_with_deposit(deps.as_mut(), &genesis);

//         let msg = HandleMsg::AddGameContract {
//             game_contract: HumanAddr::from("lucky"),
//         };

//         let info = mock_info(&genesis, &[Coin{
//             denom: "uscrt".to_string(),
//             amount: Uint128(1000000),
//         }]);
//         let env = mock_env();
//         handle(deps.as_mut(), env, info, msg.clone()).unwrap();

//         let msg1 = HandleMsg::RemoveGameContract {
//             game_contract: HumanAddr::from("lucky"),
//         };

//         let env1 = mock_env();
//         let info1 = mock_info(&genesis, &[Coin{
//             denom: "uscrt".to_string(),
//             amount: Uint128(1000000),
//         }]);
//         handle(deps.as_mut(), env1, info1, msg1.clone()).unwrap();

//         let casino = get_casino_info(deps.as_ref());
//         assert_eq!(casino, CasinoInfoResponse{
//             owner: deps.api.canonical_address(&sender).unwrap(),
//             pool: Uint128(100000),
//             game_contracts: vec![],
//         });
//     }


//     #[test]
//     fn test_play() {
//         let mut deps = mock_dependencies(&[]);
//         let genesis = HumanAddr::from("genesis");
//         let sender = HumanAddr::from("asmodat");
//         do_init_with_deposit(deps.as_mut(), &genesis);

//         let prize = Uint128(1000000);
//         let deposit_msg = HandleMsg::Deposit {};

//         let deposit_info = mock_info(&sender, &[Coin{
//             denom: "uscrt".to_string(),
//             amount: Uint128(1000000),
//         }]);
//         let env = mock_env();
//         let res = handle(deps.as_mut(), env, deposit_info, deposit_msg.clone()).unwrap();
//         assert_eq!(0, res.messages.len());
//         assert_eq!(get_balance(deps.as_ref(), &sender), prize);

//         let msg = HandleMsg::AddGameContract {
//             game_contract: HumanAddr::from("genesis"),
//         };

//         let info = mock_info(&genesis, &[]);
//         let env = mock_env();
//         handle(deps.as_mut(), env, info, msg.clone()).unwrap();

//         let play = HandleMsg::Play {
//             result: false,
//             bet_amount: Uint128(10000),
//             prize_amount: Uint128(10000),
//             winner: sender,
//         };

//         let info = mock_info(&genesis, &[Coin{
//             denom: "uscrt".to_string(),
//             amount: Uint128(1000000),
//         }]);

//         let env = mock_env();
//         handle(deps.as_mut(), env, info, play.clone()).unwrap();

//         assert_eq!(query_token_ratio(deps.as_ref()).unwrap(), Uint128(123));

//         let play2 = HandleMsg::Play {
//             result: true,
//             bet_amount: Uint128(10000),
//             prize_amount: Uint128(5000),
//             winner: HumanAddr::from("asmodat"),
//         };

//         let info2 = mock_info(&genesis, &[Coin{
//             denom: "uscrt".to_string(),
//             amount: Uint128(1000000),
//         }]);

//         let env2 = mock_env();
//         handle(deps.as_mut(), env2, info2, play2.clone()).unwrap();

//         assert_eq!(query_token_ratio(deps.as_ref()).unwrap(), Uint128(123));

//         let casino = get_casino_info(deps.as_ref());
//         assert_eq!(casino, CasinoInfoResponse{
//             owner: deps.api.canonical_address(&genesis).unwrap(),
//             pool: Uint128(10000),
//             game_contracts: vec![deps.api.canonical_address(&HumanAddr::from("genesis")).unwrap()],
//         });
//     }
// }
