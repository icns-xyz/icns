use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CW721Base(#[from] cw721_base::ContractError),

    #[error("Invalid name")]
    InvalidName {},
}
