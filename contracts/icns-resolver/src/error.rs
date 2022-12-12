use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Storage error")]
    StorageErr {},

    #[error("User Already Registered")]
    UserAlreadyRegistered { name: String },

    #[error("Bech32 decoding failed for addr: {addr:?}")]
    Bech32DecodingErr { addr: String },

    #[error("Address hash method not supported")]
    HashMethodNotSupported {},

    #[error("Signature mismatch")]
    SignatureMisMatch {},

    #[error("signature already exists")]
    SigntaureAlreadyExists {},

    #[error("invalid pub key: {pub_key:?}")]
    InvalidPubKey { pub_key: String },

    #[error("adr36 info signature and salt should be empty")]
    SignatureShouldBeEmpty { },

    #[error("Bech32 prefix mismatch between prefix: {prefix:?} and addr: {addr:?}")]
    Bech32PrefixMismatch { prefix: String, addr: String },

    #[error("Bech32 Address not set for name: {name:?}, address: {address:?}")]
    Bech32AddressNotSet { name: String, address: String },

    #[error("Removing primary address not allowed when address has more than 1 name, consider setting primary address to another address")]
    RemovingPrimaryAddressNotAllowed {},

    #[error("Invalid ICNS")]
    InvalidICNS {},
}
