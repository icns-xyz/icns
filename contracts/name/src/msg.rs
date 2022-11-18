use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw721_base::msg::QueryMsg as Cw721QueryMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub minter: String,
    pub transferrable: bool,
}

#[cw_serde]
#[serde(untagged)]
pub enum ExecuteMsg {
    CW721Base(cw721_base::ExecuteMsg<cw721_base::Extension, Empty>),
    ICNSName(ICNSNameExecuteMsg),
}

#[cw_serde]
pub enum ICNSNameExecuteMsg {
    SetAdmin { admin: String },
    SetTransferrable { transferrable: bool },
}

#[cw_serde]
pub enum QueryMsg {
    Admin {},
    Transferrable {},
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
}

impl From<QueryMsg> for Cw721QueryMsg<Empty> {
    fn from(msg: QueryMsg) -> Cw721QueryMsg<Empty> {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            _ => unreachable!("cannot convert {:?} to Cw721QueryMsg", msg),
        }
    }
}

#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}

#[cw_serde]
pub struct TransferrableResponse {
    pub transferrable: bool,
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, to_binary, Binary};

    use super::*;

    #[test]
    fn execute_msg_serde_should_conform_cw721_standard() {
        let execute_msg_binary: Binary =
            r#"{"transfer_nft":{"recipient":"recp","token_id":"name"}}"#
                .as_bytes()
                .into();
        let execute_msg: ExecuteMsg = from_binary(&execute_msg_binary).unwrap();

        assert_eq!(
            std::str::from_utf8(to_binary(&execute_msg).unwrap().as_slice()),
            std::str::from_utf8(execute_msg_binary.as_slice())
        )
    }

    #[test]
    fn execute_msg_serde_should_include_set_admin_extension() {
        let execute_msg_binary: Binary =
            r#"{"set_admin":{"admin":"admin_address"}}"#.as_bytes().into();
        let execute_msg: ExecuteMsg = from_binary(&execute_msg_binary).unwrap();

        assert_eq!(
            std::str::from_utf8(to_binary(&execute_msg).unwrap().as_slice()),
            std::str::from_utf8(execute_msg_binary.as_slice())
        )
    }
}
