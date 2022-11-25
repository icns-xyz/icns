use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::msg::ExecuteMsg;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

// pub fn admin(deps: Deps) -> Result<Vec<String>, ContractError> {
//     let cfg = CONFIG.load(deps.storage)?;
//     let name_address = cfg.name_address;

//     // query admin from icns-name-nft contract
//     let query_msg = QueryMsgName::Admin {};
//     let res: AdminResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//         contract_addr: name_address.to_string(),
//         msg: to_binary(&query_msg)?,
//     }))?;

//     Ok(res.admins)
// }
