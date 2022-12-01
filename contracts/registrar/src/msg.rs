use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Decimal};

/// Message type for `instantiate` entry_point
// TODO: change this to array
#[cw_serde]
pub struct InstantiateMsg {
    pub name_nft_addr: String,
    pub verifier_pubkeys: Vec<Binary>,
    pub verification_threshold: Decimal,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Claim {
        name: String,
        verifying_msg: String,
        // vec of `base64(secp256k1_sign(verifying_msg, verifier_key))`
        verifications: Vec<Verification>,

        referral: Option<String>,
    },
    UpdateVerifierPubkeys {
        add: Vec<Binary>,
        remove: Vec<Binary>,
    },
    SetVerificationThreshold {
        threshold: Decimal,
    },
    SetNameNFTAddress {
        name_nft_address: String,
    },
}
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(VerifierPubKeysResponse)]
    VerifierPubKeys {},

    #[returns(VerificationThresholdResponse)]
    VerificationThreshold {},

    #[returns(NameNFTAddressResponse)]
    NameNFTAddress {},
}

#[cw_serde]
pub struct VerifierPubKeysResponse {
    pub verifier_pubkeys: Vec<Binary>,
}
#[cw_serde]
pub struct VerificationThresholdResponse {
    pub verification_threshold_percentage: Decimal,
}

#[cw_serde]
pub struct NameNFTAddressResponse {
    pub name_nft_address: String,
}

#[cw_serde]
pub struct Verification {
    pub public_key: Binary,
    pub signature: Binary,
}

#[cw_serde]
pub struct VerifyingMsg {
    pub name: String,
    pub claimer: String,
    pub contract_address: String,
    pub chain_id: String,
}
