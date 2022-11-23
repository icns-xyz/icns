use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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

    #[error("Invalid SEC-1 encoded public key")]
    InvalidPublicKeyFormat {},

    #[error("Invalid ASN.1 DER formatted signature")]
    InvalidSignatureFormat {},

    #[error("Msg and public key does not match signature")]
    InvalidSignature {},

    #[error("Name mismatched")]
    NameMismatched,

    #[error("Claimer mismatched")]
    ClaimerMismatched,

    #[error("Valid verfication is below threshold: expected {expected} but got {actual}")]
    ValidVerificationIsBelowThreshold { expected: u64, actual: u64 },
}
