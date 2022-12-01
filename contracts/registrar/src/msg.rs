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
    AddVerifier {
        verifier_pubkey: Binary,
    },
    RemoveVerifier {
        verifier_pubkey: Binary,
    },
    SetVerificationThreshold {
        threshold: Decimal,
    },
}
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetReferralCountResponse)]
    GetReferralCount { user_name: String },
}

#[cw_serde]
pub struct GetReferralCountResponse {
    pub admins: Vec<String>,
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
