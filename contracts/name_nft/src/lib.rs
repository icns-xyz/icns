pub use crate::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_std::Empty;
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    ContractError, Cw721Contract, ExecuteMsg as CW721BaseExecuteMsg, Extension,
    InstantiateMsg as Cw721BaseInstantiateMsg, MintMsg, MinterResponse,
};

mod checks;
pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:icns-name-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type ICNSNameNFTContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

pub mod entry {
    use super::*;
    use crate::checks::{
        check_send_from_admin, check_send_from_registrar, check_transferrable, pass_any,
    };
    use crate::execute::{set_admin, set_transferrable, update_resolvers};
    use crate::msg::ExecuteMsg;
    use crate::query::{admin, resolvers, transferrable};
    use crate::state::{Config, CONFIG};
    use cosmwasm_std::{
        entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    };

    const NAME: &str = "icns-name";
    const SYMBOL: &str = "icns";

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let admin_addr: Addr = deps.api.addr_validate(&msg.admin)?;

        let config = Config {
            admin: admin_addr,
            transferrable: msg.transferrable,
        };

        CONFIG.save(deps.storage, &config)?;

        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
            minter: msg.registrar,
        };

        ICNSNameNFTContract::default().instantiate(
            deps.branch(),
            env,
            info,
            cw721_base_instantiate_msg,
        )?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, cw721_base::ContractError> {
        match msg {
            ExecuteMsg::CW721Base(msg) => {
                match msg {
                    // TransferNft and SendNft are supported only if transferrable is set to true
                    msg @ CW721BaseExecuteMsg::TransferNft { .. }
                    | msg @ CW721BaseExecuteMsg::SendNft { .. } => {
                        pass_any(&[
                            // transferrability is configurable.
                            check_transferrable(deps.as_ref()),
                            // allow registrar to transfer as part of registration process.
                            check_send_from_registrar(deps.as_ref(), &info.sender),
                        ])?;

                        _execute(deps, env, info, msg)
                    }

                    // approval related msgs are allowed as is
                    msg @ CW721BaseExecuteMsg::Approve { .. }
                    | msg @ CW721BaseExecuteMsg::Revoke { .. }
                    | msg @ CW721BaseExecuteMsg::ApproveAll { .. }
                    | msg @ CW721BaseExecuteMsg::RevokeAll { .. } => _execute(deps, env, info, msg),

                    // minting is allowed as is
                    msg @ CW721BaseExecuteMsg::Mint(_) => _execute(deps, env, info, msg),

                    // buring is disabled
                    CW721BaseExecuteMsg::Burn { .. } => Err(ContractError::Unauthorized {}),

                    // cw721_base extension is not being used
                    CW721BaseExecuteMsg::Extension { .. } => unimplemented!(),
                }
            }
            ExecuteMsg::ICNSName(msg) => match msg {
                msg::ICNSNameExecuteMsg::SetAdmin { admin } => {
                    check_send_from_admin(deps.as_ref(), &info.sender)?;
                    set_admin(&admin, deps)
                }
                msg::ICNSNameExecuteMsg::SetTransferrable { transferrable } => {
                    check_send_from_admin(deps.as_ref(), &info.sender)?;
                    set_transferrable(transferrable, deps)
                }
                msg::ICNSNameExecuteMsg::UpdateResolvers { add, remove } => {
                    check_send_from_admin(deps.as_ref(), &info.sender)?;
                    let add = validate_addresses(deps.as_ref(), &add)?;
                    let remove = validate_addresses(deps.as_ref(), &remove)?;

                    update_resolvers(add, remove, deps)
                }
            },
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Admin {} => to_binary(&admin(deps)?),
            QueryMsg::Transferrable {} => to_binary(&transferrable(deps)?),
            QueryMsg::Resolvers {} => to_binary(&resolvers(deps)?),
            _ => _query(deps, env, msg.into()),
        }
    }

    fn validate_addresses(deps: Deps, addrs: &[String]) -> StdResult<Vec<Addr>> {
        addrs
            .iter()
            .map(|addr| deps.api.addr_validate(addr))
            .collect()
    }
}

#[cfg(test)]
mod tests;
