use cosmwasm_schema::cw_serde;

/// Message type for `instantiate` entry_point
// TODO: change this to array
#[cw_serde]
pub struct InstantiateMsg {
    pub name_nft_addr: String,
    pub verifier_addrs: Vec<String>,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Register {
        name: String,
        owner: String,
        // tuple of (bech32 prefix, address)
        addresses: Vec<(String, String)>,
    },
    AddVerifier {
        verifier_addr: String,
    },
    RemoveVerifier {
        verifier_addr: String,
    },
}
