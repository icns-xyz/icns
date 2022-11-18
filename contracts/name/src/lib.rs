pub use crate::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_std::Empty;
pub use cw721_base::{
    entry::{execute as _execute, query as _query},
    ContractError, Cw721Contract, ExecuteMsg as CW721BaseExecuteMsg, Extension,
    InstantiateMsg as Cw721BaseInstantiateMsg, MintMsg, MinterResponse,
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
    use crate::msg::ExecuteMsg;
    use crate::query::admin;
    use crate::state::{Config, CONFIG};
    use cosmwasm_std::{
        entry_point, from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo,
        Response, StdResult,
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
        msg: ExecuteMsg,
    ) -> Result<Response, cw721_base::ContractError> {
        let config = CONFIG.load(deps.storage)?;
        match msg {
            ExecuteMsg::CW721Base(msg) => {
                let MinterResponse { minter } = from_binary(&_query(
                    deps.as_ref(),
                    env.clone(),
                    cw721_base::QueryMsg::Minter {},
                )?)?;

                if config.admin == info.sender || minter == info.sender || config.transferable {
                    _execute(deps, env, info, msg)
                } else {
                    Err(ContractError::Unauthorized {})
                }
            }
            ExecuteMsg::ICNSName(msg) => match msg {
                msg::ICNSNameExecuteMsg::SetAdmin { admin } => {
                    if config.admin == info.sender {
                        CONFIG.update(deps.storage, |config| -> StdResult<_> {
                            Ok(Config {
                                admin: deps.api.addr_validate(&admin)?,
                                ..config
                            })
                        })?;
                        Ok(Response::new()
                            .add_attribute("method", "set_admin")
                            .add_attribute("admin", admin))
                    } else {
                        Err(ContractError::Unauthorized {})
                    }
                }
            },
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
    use crate::msg::{AdminResponse, ExecuteMsg, ICNSNameExecuteMsg};

    use super::*;
    use cosmwasm_std::Addr;
    use cw721::OwnerOfResponse;
    use cw_multi_test::{BasicApp, Contract, ContractWrapper, Executor};

    pub fn name_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(entry::execute, entry::instantiate, entry::query);
        Box::new(contract)
    }

    pub struct Env {
        pub app: BasicApp,
        pub code_id: u64,
        pub contract_addr: Addr,
        pub admin: Addr,
        pub registry: Addr,
    }

    pub struct EnvBuilder {
        pub admin: Addr,
        pub registry: Addr,
        pub transferable: bool,
    }

    impl Default for EnvBuilder {
        fn default() -> Self {
            Self {
                admin: Addr::unchecked("admin"),
                registry: Addr::unchecked("registry"),
                transferable: false,
            }
        }
    }

    impl EnvBuilder {
        pub fn with_transferable(self, transferable: bool) -> Self {
            Self {
                transferable,
                ..self
            }
        }

        pub fn build(self) -> Env {
            let mut app = BasicApp::default();
            let code_id = app.store_code(name_contract());

            let contract_addr = app
                .instantiate_contract(
                    code_id,
                    self.admin.clone(),
                    &InstantiateMsg {
                        admin: self.admin.to_string(),
                        name: "icns-name-ownership".to_string(),
                        symbol: "icns".to_string(),
                        minter: self.registry.to_string(),
                        transferable: self.transferable,
                    },
                    &[],
                    "name_ownership",
                    None,
                )
                .unwrap();

            Env {
                app,
                code_id,
                contract_addr,
                admin: self.admin,
                registry: self.registry,
            }
        }
    }

    #[test]
    fn non_transferable_name() {
        let name_owner = Addr::unchecked("name_owner");
        let recipient = Addr::unchecked("recipient");
        let name = "alice";

        let Env {
            mut app,
            contract_addr,
            registry,
            ..
        } = EnvBuilder::default().with_transferable(false).build();

        // registry mint test icns_name
        app.execute_contract(
            registry,
            contract_addr.clone(),
            &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .unwrap();

        // transfer must be unauthorized
        let err = app
            .execute_contract(
                name_owner,
                contract_addr,
                &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::TransferNft {
                    recipient: recipient.to_string(),
                    token_id: name.to_string(),
                }),
                &[],
            )
            .unwrap_err();

        assert_eq!(
            err.downcast_ref::<ContractError>().unwrap(),
            &ContractError::Unauthorized {}
        );
    }

    #[test]
    fn transferable_name() {
        let Env {
            mut app,
            contract_addr,
            registry,
            ..
        } = EnvBuilder::default().with_transferable(true).build();

        let name_owner = Addr::unchecked("name_owner");
        let recipient = Addr::unchecked("recipient");
        let name = "alice";

        // mint test name
        app.execute_contract(
            registry,
            contract_addr.clone(),
            &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
                token_id: name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: None,
            })),
            &[],
        )
        .unwrap();

        // transfer to recipient should succeed
        app.execute_contract(
            name_owner,
            contract_addr.clone(),
            &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::TransferNft {
                recipient: recipient.to_string(),
                token_id: name.to_string(),
            }),
            &[],
        )
        .unwrap();

        // icns_name is now owned by recipient
        let res: OwnerOfResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr,
                &QueryMsg::OwnerOf {
                    token_id: name.to_string(),
                    include_expired: None,
                },
            )
            .unwrap();

        assert_eq!(res.owner, recipient.to_string());
    }

    #[test]
    fn only_admin_can_set_new_admin() {
        let mut app = BasicApp::default();
        let code_id = app.store_code(name_contract());
        let admin = Addr::unchecked("admin");
        let new_admin = Addr::unchecked("new_admin");

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

        // set new admin by non admin should fail
        let err = app
            .execute_contract(
                new_admin.clone(),
                contract_addr.clone(),
                &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetAdmin {
                    admin: new_admin.to_string(),
                }),
                &[],
            )
            .unwrap_err();

        assert_eq!(
            err.downcast_ref::<ContractError>().unwrap(),
            &ContractError::Unauthorized {}
        );

        // admin should not be changed
        let res: AdminResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Admin {})
            .unwrap();

        assert_eq!(res.admin, admin.to_string());

        // set new admin by admin should succeed
        app.execute_contract(
            admin,
            contract_addr.clone(),
            &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetAdmin {
                admin: new_admin.to_string(),
            }),
            &[],
        )
        .unwrap();

        let res: AdminResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Admin {})
            .unwrap();

        assert_eq!(res.admin, new_admin.to_string());
    }
}
