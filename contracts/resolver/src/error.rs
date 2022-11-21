use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("User Already Registered")]
    UserAlreadyRegistered{ name: String },

    #[error("Bech32 decoding failed for addr: {addr:?}")]
    Bech32DecodingErr { addr: String },

    #[error("Bech32 prefix mismatch between prefix: {prefix:?} and addr: {addr:?}")]
    Bech32PrefixMismatch { prefix: String, addr: String },
}
