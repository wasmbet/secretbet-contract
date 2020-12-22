use cosmwasm_std::{
    attr, to_binary, Binary, Deps, DepsMut, Env, HandleResponse, HumanAddr, InitResponse, CanonicalAddr,
    MessageInfo, StdError, StdResult, Uint128, BankMsg, Coin, WasmQuery, QueryRequest, from_binary, CosmosMsg, WasmMsg, 
};

use cw2::{get_contract_version, set_contract_version};
use cw20::{BalanceResponse, Cw20CoinHuman, Cw20ReceiveMsg, MinterResponse, TokenInfoResponse, Cw20QueryMsg, Cw20HandleMsg};

// use crate::enumerable::{query_all_accounts, query_all_allowances};
use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, MigrateMsg, QueryMsg, Cw20HookMsg};
use crate::response::{CasinoInfoResponse};
use crate::state::{balances, balances_read, config, config_read, Config};

// version info for migration info
const CONTRACT_NAME: &str = "cowbird:decasino";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn init(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    let cfg = Config {
        owner: deps.api.canonical_address(&_info.sender)?,
        pool_token: CanonicalAddr::default(),
        decasino_token: CanonicalAddr::default(),
        game_contracts: vec![],
        pool: Uint128(0),
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
        HandleMsg::UpdatePoolTokenContract { pool_token } => handle_update_pool_token(deps, env, info, pool_token),
        HandleMsg::UpdateDecasinoTokenContract { decasino_token } => handle_update_decasino_token(deps, env, info, decasino_token),
        HandleMsg::AddGameContract { game_contract } => handle_add_game_contract(deps, env, info, game_contract),
        HandleMsg::RemoveGameContract { game_contract } => handle_remove_game_contract(deps, env, info, game_contract),
        HandleMsg::Deposit { } => handle_deposit(deps, env, info),
        // HandleMsg::Withdraw { amount } => handle_withdraw(deps, env, info, amount),
        HandleMsg::Receive(msg) => handle_withdraw_cw20(deps, env, info, msg),
        HandleMsg::Play {
            result,
            bet_amount,
            prize_amount,
            winner,
        } => handle_play(deps, env, info, result, bet_amount, prize_amount, winner),
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

pub fn handle_update_pool_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    pool_token: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.pool_token = deps.api.canonical_address(&pool_token)?;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "updatePoolTokenContract"),
            attr("pool_token", pool_token),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_update_decasino_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    decasino_token: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    cfg.decasino_token = deps.api.canonical_address(&decasino_token)?;
    config(deps.storage).save(&cfg)?;

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "updateDecasinoTokenContract"),
            attr("pool_token", decasino_token),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_add_game_contract(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    game_contract: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let mut casino = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;
    if casino.owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    let contract = deps.api.canonical_address(&game_contract)?;
    if !casino.game_contracts.contains(&contract) {
        casino.game_contracts.push(contract);
        config(deps.storage).save(&casino)?;
    }

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "addGameContract"),
            attr("owner", game_contract),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_remove_game_contract(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    game_contract: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let mut cfg = config_read(deps.storage).load()?;
    let sender = deps.api.canonical_address(&info.sender)?;

    if cfg.owner != sender {
        return Err(ContractError::Unauthorized {});
    }
    let contract = deps.api.canonical_address(&game_contract)?;
    if cfg.game_contracts.contains(&contract) {
        cfg.game_contracts.retain(|c| c != &contract);
        config(deps.storage).save(&cfg)?;
    }

    let res = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "removeGameContract"),
            attr("owner", game_contract),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<HandleResponse, ContractError> {

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

    let amount = coin.amount;
    
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }
    
    let mut cfg = config_read(deps.storage).load()?;
    let mut pool_token_info = query_pool_token_info(deps.as_ref());

    let mint;
    if &pool_token_info.total_supply.is_zero() || cfg.pool.is_zero() {
        mint = amount;
    } else {
        // @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ 이렇게해야되? 왜 Uint128끼리 계산어케해
        mint = Uint128((pool_token_info.total_supply.u128() * amount.u128()) / cfg.pool.u128());
    }

    cfg.pool += amount;
    config(deps.storage).save(&cfg)?;

    let sender_raw = deps.api.canonical_address(&info.sender)?;

    let res = HandleResponse {
        messages: vec![
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.human_address(&cfg.pool_token)?,
                msg: to_binary(&Cw20HandleMsg::Mint {
                    amount: mint,
                    recipient: info.sender,
                })?,
                send: vec![],
            }),
        ],
        attributes: vec![
            attr("action", "deposit_mint"),
            attr("amount", mint),
        ],
        data: None,
    };
    Ok(res)
}

pub fn handle_withdraw_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<HandleResponse, ContractError> {

    if let Some(msg) = cw20_msg.msg {
        match from_binary(&msg)? {
            Cw20HookMsg::Withdraw {} => {
                let mut cfg = config_read(deps.storage).load()?;
                if deps.api.canonical_address(&info.sender)? != cfg.pool_token {
                    return Err(ContractError::Unauthorized {});
                }

                try_withdraw_liquidity(deps, env, cw20_msg.sender, cw20_msg.amount)
            }
        }
    } else {
        Err(StdError::generic_err("data should be given"))
    }
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let sender_raw = deps.api.canonical_address(&info.sender)?;

    let current_amount = balances(deps.storage)
        .may_load(sender_raw.as_slice())?
        .unwrap_or_default();

    if amount > current_amount {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut cfg = config_read(deps.storage).load()?;
    let mut pool_token_info = query_pool_token_info(deps);

    let send_amount = (casino.pool.u128() / token.total_supply.u128()) * amount.u128();
    casino.pool = Uint128(casino.pool.u128() - send_amount);
    casino_info(deps.storage).save(&casino)?;

    let mut accounts = balances(deps.storage);
    accounts.update(sender_raw.as_slice(), |balance: Option<Uint128>| {
        balance.unwrap_or_default() - amount
    })?;

    token_info(deps.storage).update(|mut info| -> StdResult<_> {
        info.total_supply = (info.total_supply - amount)?;
        Ok(info)
    })?;

    let token_transfer = BankMsg::Send {
        from_address: env.contract.address.clone(),
        to_address: info.sender.clone(),
        amount: vec![Coin {
            denom: "uscrt".to_string(),
            amount: Uint128(send_amount),
        }],
    }
    .into();

    let res = HandleResponse {
        messages: vec![token_transfer],
        attributes: vec![
            attr("action", "withdraw"),
            attr("amount", amount),
            attr("send_amount", send_amount),
        ],
        data: None,
    };
    Ok(res)
}

// pub fn handle_withdraw(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     amount: Uint128,
// ) -> Result<HandleResponse, ContractError> {
//     if amount == Uint128::zero() {
//         return Err(ContractError::InvalidZeroAmount {});
//     }

//     let sender_raw = deps.api.canonical_address(&info.sender)?;

//     let current_amount = balances(deps.storage)
//         .may_load(sender_raw.as_slice())?
//         .unwrap_or_default();

//     if amount > current_amount {
//         return Err(ContractError::InvalidZeroAmount {});
//     }

//     let mut cfg = config_read(deps.storage).load()?;
//     let mut pool_token_info = query_pool_token_info(deps);

//     let send_amount = (casino.pool.u128() / token.total_supply.u128()) * amount.u128();
//     casino.pool = Uint128(casino.pool.u128() - send_amount);
//     casino_info(deps.storage).save(&casino)?;

//     let mut accounts = balances(deps.storage);
//     accounts.update(sender_raw.as_slice(), |balance: Option<Uint128>| {
//         balance.unwrap_or_default() - amount
//     })?;

//     token_info(deps.storage).update(|mut info| -> StdResult<_> {
//         info.total_supply = (info.total_supply - amount)?;
//         Ok(info)
//     })?;

//     let token_transfer = BankMsg::Send {
//         from_address: env.contract.address.clone(),
//         to_address: info.sender.clone(),
//         amount: vec![Coin {
//             denom: "uscrt".to_string(),
//             amount: Uint128(send_amount),
//         }],
//     }
//     .into();

//     let res = HandleResponse {
//         messages: vec![token_transfer],
//         attributes: vec![
//             attr("action", "withdraw"),
//             attr("amount", amount),
//             attr("send_amount", send_amount),
//         ],
//         data: None,
//     };
//     Ok(res)
// }

pub fn handle_play(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    result: bool,
    bet_amount: Uint128,
    prize_amount: Uint128,
    winner: HumanAddr,
) -> Result<HandleResponse, ContractError> {
    let casino = casino_info_read(deps.storage).load()?;
    
    let sender_raw = deps.api.canonical_address(&info.sender)?;
    if !casino.game_contracts.contains(&sender_raw) {
        return Err(ContractError::Unauthorized {});
    }

    let mut messages = vec![];
    if result {
        casino_info(deps.storage).update(|mut info| -> StdResult<_> {
            info.pool = (info.pool - prize_amount)?;
            Ok(info)
        })?;
    } else {
        casino_info(deps.storage).update(|mut info| -> StdResult<_> {
            info.pool = info.pool + bet_amount;
            Ok(info)
        })?;

        let token_transfer = BankMsg::Send {
            from_address: env.contract.address.clone(),
            to_address: winner.clone(),
            amount: vec![Coin {
                denom: "uscrt".to_string(),
                amount: prize_amount,
            }],
        }.into();
        messages.push(token_transfer);
    }

    let res = HandleResponse {
        messages,
        attributes: vec![
            attr("action", "bet"),
            attr("result", result),
            attr("bet_amount", bet_amount),
            attr("prize_amount", prize_amount),
            attr("winner", winner),
        ],
        data: None,
    };
    Ok(res)
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::PoolTokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<CasinoInfoResponse> {
    let info = config_read(deps.storage).load()?;
    let res = CasinoInfoResponse {
        owner: info.owner,
        pool: info.pool,
        game_contracts: info.game_contracts,
    };
    Ok(res)
}

pub fn query_pool_token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let cfg = config_read(deps.storage).load()?;
    let res: TokenInfoResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: &deps.api.human_address(cfg.pool_token),
        msg: to_binary(&QueryMsg::TokenInfo { })?,
    }))?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Api};

    use super::*;

    fn get_balance<T: Into<HumanAddr>>(deps: Deps, address: T) -> Uint128 {
        query_balance(deps, address.into()).unwrap().balance
    }

    // fn get_casino_info(deps: Deps) -> CasinoInfoResponse {
    //     config_read(deps).unwrap()
    // }

    fn get_token_info(deps: Deps) -> TokenInfoResponse {
        query_token_info(deps).unwrap()
    }

    // this will set up the init for other tests
    fn do_init_with_deposit(
        deps: DepsMut,
        addr: &HumanAddr,
    ) -> TokenInfoResponse {
        _do_init(
            deps,
            addr,
        )
    }

    // this will set up the init for other tests
    fn do_init(deps: DepsMut, addr: &HumanAddr) -> TokenInfoResponse {
        _do_init(deps, addr)
    }

    // this will set up the init for other tests
    fn _do_init(
        mut deps: DepsMut,
        addr: &HumanAddr,
    ) -> TokenInfoResponse {
        let init_msg = InitMsg {
            initial_balances: vec![],
        };
        let info = mock_info(addr, &[]);
        let env = mock_env();
        let res = init(dup(&mut deps), env, info, init_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let meta = query_token_info(deps.as_ref()).unwrap();
        assert_eq!(
            meta,
            TokenInfoResponse {
                name: "cpool".to_string(),
                symbol: "cool".to_string(),
                decimals: 18,
                total_supply: Uint128(0),
            }
        );
        // assert_eq!(get_balance(deps.as_ref(), addr), amount);
        meta
    }

    // TODO: replace this with deps.dup()
    // after https://github.com/CosmWasm/cosmwasm/pull/620 is merged
    fn dup<'a>(deps: &'a mut DepsMut<'_>) -> DepsMut<'a> {
        DepsMut {
            storage: deps.storage,
            api: deps.api,
            querier: deps.querier,
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);
        let amount = Uint128::from(11223344u128);
        let init_msg = InitMsg {
            initial_balances: vec![Cw20CoinHuman {
                address: HumanAddr("addr0000".to_string()),
                amount,
            }],
        };
        let info = mock_info(&HumanAddr("creator".to_string()), &[]);
        let env = mock_env();
        let res = init(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
        assert_eq!(0, res.messages.len());

        assert_eq!(
            query_token_info(deps.as_ref()).unwrap(),
            TokenInfoResponse {
                name: "cpool".to_string(),
                symbol: "cool".to_string(),
                decimals: 18,
                total_supply: amount,
            }
        );
        assert_eq!(get_balance(deps.as_ref(), "addr0000"), Uint128(11223344));
    }

    #[test]
    fn can_mint_by_deposit() {
        let mut deps = mock_dependencies(&[]);

        let genesis = HumanAddr::from("genesis");
        let amount = Uint128(11223344);
        let sender = HumanAddr::from("asmodat");
        do_init_with_deposit(deps.as_mut(), &genesis);

        // minter can mint coins to some winner
        let prize = Uint128(1000000);
        let msg = HandleMsg::Deposit {};

        let info = mock_info(&sender, &[Coin{
            denom: "uscrt".to_string(),
            amount,
        }]);
        let env = mock_env();
        let res = handle(deps.as_mut(), env, info, msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());
        // assert_eq!(get_balance(deps.as_ref(), &genesis), Uint128(11234120));

        // let casino = get_casino_info(deps.as_ref());
        // assert_eq!(casino, CasinoInfoResponse{
        //     owner: deps.api.canonical_address(&sender).unwrap(),
        //     pool: Uint128(100),
        //     game_contracts: vec![],
        // });

        // let token = get_token_info(deps.as_ref());
        // assert_eq!(token, TokenInfoResponse{
        //     name: "cpool".to_string(),
        //     symbol: "cool".to_string(),
        //     decimals: 18,
        //     total_supply: Uint128(0),
        // });

        assert_eq!(get_balance(deps.as_ref(), &sender), prize);
    }

    #[test]
    fn test_add_game_contract() {
        let mut deps = mock_dependencies(&[]);
        let genesis = HumanAddr::from("genesis");
        let sender = HumanAddr::from("asmodat");
        do_init_with_deposit(deps.as_mut(), &genesis);

        let msg = HandleMsg::AddGameContract {
            game_contract: HumanAddr::from("lucky"),
        };

        let info = mock_info(&genesis, &[Coin{
            denom: "uscrt".to_string(),
            amount: Uint128(1000000),
        }]);
        let env = mock_env();
        handle(deps.as_mut(), env, info, msg.clone()).unwrap();

        let casino = get_casino_info(deps.as_ref());
        assert_eq!(casino, CasinoInfoResponse{
            owner: deps.api.canonical_address(&sender).unwrap(),
            pool: Uint128(100000),
            game_contracts: vec![],
        });
    }

    
    #[test]
    fn test_remove_game_contract() {
        let mut deps = mock_dependencies(&[]);
        let genesis = HumanAddr::from("genesis");
        let sender = HumanAddr::from("asmodat");
        do_init_with_deposit(deps.as_mut(), &genesis);

        let msg = HandleMsg::AddGameContract {
            game_contract: HumanAddr::from("lucky"),
        };

        let info = mock_info(&genesis, &[Coin{
            denom: "uscrt".to_string(),
            amount: Uint128(1000000),
        }]);
        let env = mock_env();
        handle(deps.as_mut(), env, info, msg.clone()).unwrap();

        let msg1 = HandleMsg::RemoveGameContract {
            game_contract: HumanAddr::from("lucky"),
        };

        let env1 = mock_env();
        let info1 = mock_info(&genesis, &[Coin{
            denom: "uscrt".to_string(),
            amount: Uint128(1000000),
        }]);
        handle(deps.as_mut(), env1, info1, msg1.clone()).unwrap();

        let casino = get_casino_info(deps.as_ref());
        assert_eq!(casino, CasinoInfoResponse{
            owner: deps.api.canonical_address(&sender).unwrap(),
            pool: Uint128(100000),
            game_contracts: vec![],
        });
    }


    #[test]
    fn test_play() {
        let mut deps = mock_dependencies(&[]);
        let genesis = HumanAddr::from("genesis");
        let sender = HumanAddr::from("asmodat");
        do_init_with_deposit(deps.as_mut(), &genesis);

        let prize = Uint128(1000000);
        let deposit_msg = HandleMsg::Deposit {};

        let deposit_info = mock_info(&sender, &[Coin{
            denom: "uscrt".to_string(),
            amount: Uint128(1000000),
        }]);
        let env = mock_env();
        let res = handle(deps.as_mut(), env, deposit_info, deposit_msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(get_balance(deps.as_ref(), &sender), prize);

        let msg = HandleMsg::AddGameContract {
            game_contract: HumanAddr::from("genesis"),
        };

        let info = mock_info(&genesis, &[]);
        let env = mock_env();
        handle(deps.as_mut(), env, info, msg.clone()).unwrap();

        let play = HandleMsg::Play {
            result: false,
            bet_amount: Uint128(10000),
            prize_amount: Uint128(10000),
            winner: sender,
        };

        let info = mock_info(&genesis, &[Coin{
            denom: "uscrt".to_string(),
            amount: Uint128(1000000),
        }]);

        let env = mock_env();
        handle(deps.as_mut(), env, info, play.clone()).unwrap();

        assert_eq!(query_token_ratio(deps.as_ref()).unwrap(), Uint128(123));

        let play2 = HandleMsg::Play {
            result: true,
            bet_amount: Uint128(10000),
            prize_amount: Uint128(5000),
            winner: HumanAddr::from("asmodat"),
        };

        let info2 = mock_info(&genesis, &[Coin{
            denom: "uscrt".to_string(),
            amount: Uint128(1000000),
        }]);

        let env2 = mock_env();
        handle(deps.as_mut(), env2, info2, play2.clone()).unwrap();

        assert_eq!(query_token_ratio(deps.as_ref()).unwrap(), Uint128(123));

        let casino = get_casino_info(deps.as_ref());
        assert_eq!(casino, CasinoInfoResponse{
            owner: deps.api.canonical_address(&genesis).unwrap(),
            pool: Uint128(10000),
            game_contracts: vec![deps.api.canonical_address(&HumanAddr::from("genesis")).unwrap()],
        });
    }
}
