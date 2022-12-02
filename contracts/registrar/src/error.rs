use cosmwasm_std::{Binary, Decimal, StdError};
use cw_utils::ThresholdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Threshold(#[from] ThresholdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not a verfier public key: {public_key}")]
    NotAVerifierPublicKey { public_key: Binary },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Invalid user name: {user_name:?}")]
    InvalidUserName { user_name: String },

    #[error("Verifier already exists")]
    VerifierAlreadyExists {},

    #[error("Verifier does not exist")]
    VerifierDoesNotExist {},

    #[error("Invalid SEC-1 encoded public key")]
    InvalidPublicKeyFormat {},

    #[error("Invalid ASN.1 DER formatted signature")]
    InvalidSignatureFormat {},

    #[error("Verifying sg and public key does not match signature")]
    InvalidSignature {},

    #[error(
        "Valid verfication is below threshold: expected over {expected_over}% but got {actual}%"
    )]
    ValidVerificationIsBelowThreshold {
        expected_over: Decimal,
        actual: Decimal,
    },

    #[error("Invalid verifying message: {msg}")]
    InvalidVerifyingMessage { msg: String },

    #[error("User already registered")]
    DuplicatedTwitterId { msg: String },

    #[error("No verifier set")]
    NoVerifier {},

    #[error("Verification signatures must be unique: `{signature}` is duplicated")]
    DuplicatedVerification { signature: Binary },

    #[error("Invalid voting threshold percentage, must be in the 0-1.0 range")]
    InvalidThreshold {},
}
