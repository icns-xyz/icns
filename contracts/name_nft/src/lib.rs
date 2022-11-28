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
    use crate::checks::{check_admin, check_transferrable};
    use crate::execute::{add_admin, remove_admin, set_minter_address, set_transferrable};
    use crate::msg::ExecuteMsg;
    use crate::query::{admin, is_admin, transferrable};
    use crate::state::{Config, CONFIG};

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw721::ContractInfoResponse;

    const NAME: &str = "icns-name";
    const SYMBOL: &str = "icns";

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        // let admin_addr: Addr = deps.api.addr_validate(&msg.admin)?;
        let mut admin_addrs = Vec::new();
        for admin in msg.admins {
            admin_addrs.push(deps.api.addr_validate(&admin)?);
        }

        let config = Config {
            admins: admin_addrs,
            transferrable: msg.transferrable,
        };

        CONFIG.save(deps.storage, &config)?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let name_nft = ICNSNameNFTContract::default();

        let info = ContractInfoResponse {
            name: NAME.to_owned(),
            symbol: SYMBOL.to_owned(),
        };

        name_nft.contract_info.save(deps.storage, &info)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
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
                        // transferrability is configurable.
                        check_transferrable(deps.as_ref())?;
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
                msg::ICNSNameExecuteMsg::AddAdmin { admin_address } => {
                    check_admin(deps.as_ref(), &info.sender)?;
                    add_admin(&admin_address, deps)
                }
                msg::ICNSNameExecuteMsg::RemoveAdmin { admin_address } => {
                    check_admin(deps.as_ref(), &info.sender)?;
                    remove_admin(&admin_address, deps)
                }
                msg::ICNSNameExecuteMsg::SetTransferrable { transferrable } => {
                    check_admin(deps.as_ref(), &info.sender)?;
                    set_transferrable(transferrable, deps)
                }
                msg::ICNSNameExecuteMsg::SetMinter { minter_address } => {
                    check_admin(deps.as_ref(), &info.sender)?;
                    set_minter_address(&minter_address, deps)
                }
            },
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Admin {} => to_binary(&admin(deps)?),
            QueryMsg::Transferrable {} => to_binary(&transferrable(deps)?),
            QueryMsg::IsAdmin { address } => to_binary(&is_admin(deps, address)?),
            _ => _query(deps, env, msg.into()),
        }
    }
}

#[cfg(test)]
mod tests;
