use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Binary, Coin, Decimal};
use cw_storage_plus::{Item, Map};

use crate::ContractError;
pub static CONFIG_KEY: &[u8] = b"config";

#[cw_serde]
pub struct Config {
    /// name_nft address to send msg to
    pub name_nft: Addr,
    /// sec1 encoded pubkey bytes of verifier, used for signature verfication
    pub verifier_pubkeys: Vec<Binary>,
    /// number of verification that needs to pass in order to mint name
    pub verification_threshold_percentage: Decimal,
    /// fee required for claiming name
    pub fee: Option<Coin>,
}

impl Config {
    pub fn check_pass_threshold(
        &self,
        passed_verification: impl Into<Decimal>,
    ) -> Result<(), ContractError> {
        let passed_verification: Decimal = passed_verification.into();
        let pct = passed_verification
            .checked_div(Decimal::new((self.verifier_pubkeys.len() as u64).into()))
            .map_err(|e| match e {
                cosmwasm_std::CheckedFromRatioError::DivideByZero => ContractError::NoVerifier {},
                cosmwasm_std::CheckedFromRatioError::Overflow => {
                    panic!("check pass verification calculation overflowed")
                }
            })?;

        if pct < self.verification_threshold_percentage {
            return Err(ContractError::ValidVerificationIsBelowThreshold {
                expected_over: self.verification_threshold_percentage,
                actual: pct,
            });
        }

        Ok(())
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REFERRAL: Map<String, u64> = Map::new("referral");
pub const UNIQUE_TWITTER_ID: Map<String, String> = Map::new("unique_twitter_id");
