use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Coin, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    // NL: Document this
    /// valid contract address of the name nft contract
    pub name_nft_addr: String,
    /// verifiers's public key that can sign verification message
    pub verifier_pubkeys: Vec<Binary>,
    /// percentage of verification signature required out of all verifiers
    pub verification_threshold: Decimal,
    /// fee required for minting new name
    pub fee: Option<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// To claim name, sender needs to gather signatures of `verifying_msg` form `verifiers`
    /// number of signatures must pass `verification_threshold` in order to proceed with minting
    /// and owning the name NFT
    ///
    /// The contract's admin can execute this message regardless of the threashold
    Claim {
        /// Name to be minted as NFT
        name: String,

        /// String representation of [`VerifyingMsg`] that is used for
        /// generating verification signature
        verifying_msg: String,

        /// Vec of all verfications, which contains both signature
        /// and pubkey that use for that signature.
        verifications: Vec<Verification>,

        /// icns name of the referer, tracked for future incentivization
        referral: Option<String>,
    },

    /// Update verifiers's public key that can sign verification message
    UpdateVerifierPubkeys {
        add: Vec<Binary>,
        remove: Vec<Binary>,
    },

    /// Set threshold as percentage of verification signature required out of all verifiers
    /// In order to pass verification and proceed with the claim
    SetVerificationThreshold {
        /// threshold percentage
        /// "0" => 0%
        /// "1000000000000000000" => 100%
        threshold: Decimal,
    },

    /// Set name NFT address to be minted once passing verfication
    SetNameNftAddress {
        /// valid contract address of the name nft contract
        name_nft_address: String,
    },

    /// Set fee required for claim
    SetMintingFee {
        /// fee required for minting new name
        minting_fee: Option<Coin>,
    },

    /// Withdraw funds from this contract
    WithdrawFunds {
        /// amount to withdraw
        amount: Vec<Coin>,
        /// address to withdraw fudn to
        to_address: String,
    },
}
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(VerifierPubKeysResponse)]
    VerifierPubKeys {},

    #[returns(VerificationThresholdResponse)]
    VerificationThreshold {},

    #[returns(NameNftAddressResponse)]
    NameNftAddress {},

    #[returns(ReferralCountResponse)]
    ReferralCount { name: String },

    #[returns(FeeResponse)]
    Fee {},

    #[returns(NameByTwitterIdResponse)]
    NameByTwitterId { twitter_id: String },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct VerifierPubKeysResponse {
    pub verifier_pubkeys: Vec<Binary>,
}
#[cw_serde]
pub struct VerificationThresholdResponse {
    pub verification_threshold_percentage: Decimal,
}

#[cw_serde]
pub struct NameNftAddressResponse {
    pub name_nft_address: String,
}

#[cw_serde]
pub struct ReferralCountResponse {
    pub count: u64,
}

#[cw_serde]
pub struct FeeResponse {
    pub fee: Option<Coin>,
}

#[cw_serde]
pub struct NameByTwitterIdResponse {
    pub name: String,
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
    pub unique_twitter_id: String,
}
