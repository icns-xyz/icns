use cosmrs::{bip32, crypto::secp256k1::SigningKey, tendermint::signature::Secp256k1Signature};
use cosmwasm_std::{Binary, Empty};
use cw_multi_test::{Contract, ContractWrapper};

pub fn name_nft_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        icns_name_nft::entry::execute,
        icns_name_nft::entry::instantiate,
        icns_name_nft::entry::query,
    );
    Box::new(contract)
}

pub fn registrar_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn from_mnemonic(phrase: &str, derivation_path: &str) -> SigningKey {
    let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)
        .unwrap()
        .to_seed("");
    let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse().unwrap()).unwrap();
    xprv.into()
}

pub trait ToBinary {
    fn to_binary(&self) -> Binary;
}

impl ToBinary for SigningKey {
    fn to_binary(&self) -> Binary {
        Binary(self.public_key().to_bytes())
    }
}

impl ToBinary for Secp256k1Signature {
    fn to_binary(&self) -> Binary {
        Binary(self.to_vec())
    }
}

pub mod fixtures {
    use super::*;

    const DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";
    pub fn verifier1() -> SigningKey {
        from_mnemonic("notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius", DERIVATION_PATH)
    }

    pub fn verifier2() -> SigningKey {
        from_mnemonic("quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty", DERIVATION_PATH)
    }
    pub fn verifier3() -> SigningKey {
        from_mnemonic("symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb", DERIVATION_PATH)
    }
    pub fn verifier4() -> SigningKey {
        from_mnemonic("bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty", DERIVATION_PATH)
    }
    pub fn non_verifier() -> SigningKey {
        from_mnemonic("prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder", DERIVATION_PATH)
    }
}
