pub use crate::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_std::Empty;
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    ContractError, Cw721Contract, ExecuteMsg, Extension, InstantiateMsg as Cw721BaseInstantiateMsg,
    MintMsg, MinterResponse,
};

pub mod msg;
pub mod query;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:icns-name-ownership";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type ICNSNameOwnership<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use crate::query::admin;
    use crate::state::{Config, CONFIG};
    use cosmwasm_std::{
        entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    };

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
            transferable: msg.transferable,
        };

        CONFIG.save(deps.storage, &config)?;

        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        };

        ICNSNameOwnership::default().instantiate(
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
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, cw721_base::ContractError> {
        let config = CONFIG.load(deps.storage)?;
        if config.admin == info.sender || config.transferable {
            _execute(deps, env, info, msg)
        } else {
            Err(ContractError::Unauthorized {})
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Admin {} => to_binary(&admin(deps)?),
            _ => _query(deps, env, msg.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Addr;
    use cw721::OwnerOfResponse;
    use cw_multi_test::{BasicApp, Contract, ContractWrapper, Executor};

    pub fn name_ownership_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(entry::execute, entry::instantiate, entry::query);
        Box::new(contract)
    }

    #[test]
    fn non_transferable_name_ownership() {
        let mut app = BasicApp::default();
        let code_id = app.store_code(name_ownership_contract());
        let admin = Addr::unchecked("admin");
        let name_owner = Addr::unchecked("name_owner");
        let recipient = Addr::unchecked("recipient");
        let icns_name = "mock_name";

        // instantiate contract with transferable = false
        let contract_addr = app
            .instantiate_contract(
                code_id,
                admin.clone(),
                &InstantiateMsg {
                    admin: admin.to_string(),
                    name: "icns-name-ownership".to_string(),
                    symbol: "icns".to_string(),
                    minter: admin.to_string(),
                    transferable: false,
                },
                &[],
                "name_ownership",
                None,
            )
            .unwrap();

        // mint test icns_name
        app.execute_contract(
            admin,
            contract_addr.clone(),
            &ExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: icns_name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: None,
            }),
            &[],
        )
        .unwrap();

        // transfer must be unauthorized
        let err = app
            .execute_contract(
                name_owner,
                contract_addr,
                &ExecuteMsg::<Extension, Empty>::TransferNft {
                    recipient: recipient.to_string(),
                    token_id: icns_name.to_string(),
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            err.downcast_ref::<ContractError>().unwrap(),
            &ContractError::Unauthorized {}
        );
    }

    #[test]
    fn transferable_name_ownership() {
        let mut app = BasicApp::default();
        let code_id = app.store_code(name_ownership_contract());
        let admin = Addr::unchecked("admin");
        let name_owner = Addr::unchecked("name_owner");
        let recipient = Addr::unchecked("recipient");
        let icns_name = "mock_name";

        // instantiate contract with transferable = true
        let contract_addr = app
            .instantiate_contract(
                code_id,
                admin.clone(),
                &InstantiateMsg {
                    admin: admin.to_string(),
                    name: "icns-name-ownership".to_string(),
                    symbol: "icns".to_string(),
                    minter: admin.to_string(),
                    transferable: true,
                },
                &[],
                "name_ownership",
                None,
            )
            .unwrap();

        // mint test icns_name
        app.execute_contract(
            admin,
            contract_addr.clone(),
            &ExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: icns_name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: None,
            }),
            &[],
        )
        .unwrap();

        // transfer to recipient should succeed
        app.execute_contract(
            name_owner,
            contract_addr.clone(),
            &ExecuteMsg::<Extension, Empty>::TransferNft {
                recipient: recipient.to_string(),
                token_id: icns_name.to_string(),
            },
            &[],
        )
        .unwrap();

        // icns_name is now owned by recipient
        let res: OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr,
                &QueryMsg::OwnerOf {
                    token_id: icns_name.to_string(),
                    include_expired: None,
                },
            )
            .unwrap();

        assert_eq!(res.owner, recipient.to_string());
    }
}
