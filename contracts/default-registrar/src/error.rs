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

    #[error("Bech32 decoding failed for addr: {addr:?}")]
    Bech32DecodingErr { addr: String },

    #[error("Bech32 prefix mismatch between prefix: {prefix:?} and addr: {addr:?}")]
    Bech32PrefixMismatch { prefix: String, addr: String },
}
