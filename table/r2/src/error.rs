use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Generic error: {msg}")]
    GenericErr {
        msg: String
    },

    #[error("NoResult Wait Next Block")]
    NoResult { },

    #[error("NoGame")]
    NoGame { },

    #[error("No Range")]
    NoRange {},

    #[error("No Method")]
    NoMethod {},

    #[error("No {denom} tokens sent")]
    EmptyBalance { denom: String },

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},
}
