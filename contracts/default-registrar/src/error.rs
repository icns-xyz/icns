use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Invalid user name: {user_name:?}")]
    InvalidUserName { user_name: String },

    #[error("Verifier already exists")]
    VerifierAlreadyExists {},

    #[error("Verifier does not exist")]
    VerifierDoesNotExist {},
}
