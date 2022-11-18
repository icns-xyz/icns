use crate::{entry, InstantiateMsg};

use cosmwasm_std::{Addr, Empty};
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
                    name: "icns-name".to_string(),
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
