use crate::{
    entry,
    msg::{ExecuteMsg, ICNSNameExecuteMsg},
    InstantiateMsg,
};

use cosmwasm_std::{Addr, DepsMut, Empty, MessageInfo, Response};
use cw_multi_test::{BasicApp, Contract, ContractWrapper, Executor};

pub fn name_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(entry::execute, entry::instantiate, entry::query);
    Box::new(contract)
}

mod reciever {
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{entry_point, Binary, Deps, Env, StdError, StdResult};
    use cw721::Cw721ReceiveMsg;

    use super::*;

    #[entry_point]
    // #[allow(dead_code)]
    pub fn instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: (),
    ) -> Result<Response, StdError> {
        Ok(Response::default())
    }

    #[cw_serde]
    pub enum ExecuteMsg {
        ReceiveNft(Cw721ReceiveMsg),
    }

    #[entry_point]
    pub fn execute(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> StdResult<Response> {
        Ok(Response::default())
    }

    #[entry_point]
    pub fn query(_deps: Deps, _env: Env, _msgg: ()) -> StdResult<Binary> {
        Err(StdError::GenericErr {
            msg: "nothing here".to_string(),
        })
    }
}

pub fn mock_reciever_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(reciever::execute, reciever::instantiate, reciever::query);
    Box::new(contract)
}

pub struct TestEnv {
    pub app: BasicApp,
    pub code_id: u64,
    pub contract_addr: Addr,
    pub admins: Vec<Addr>,
    pub registrar: Addr,
}

pub struct TestEnvBuilder {
    pub admins: Vec<Addr>,
    pub registrar: Addr,
    pub transferrable: bool,
    pub setup_minter: bool,
}

impl Default for TestEnvBuilder {
    fn default() -> Self {
        Self {
            admins: vec![Addr::unchecked("admin")],
            registrar: Addr::unchecked("registrar"),
            transferrable: false,
            setup_minter: true,
        }
    }
}

impl TestEnvBuilder {
    pub fn with_transferrable(self, transferrable: bool) -> Self {
        Self {
            transferrable,
            ..self
        }
    }

    pub fn with_no_minter(self) -> Self {
        Self {
            setup_minter: false,
            ..self
        }
    }

    pub fn build(self) -> TestEnv {
        let mut app = BasicApp::default();
        let code_id = app.store_code(name_contract());

        let sender = self.admins[0].clone();

        // change admin address vec to string vec without losing ownership
        let mut admin_strs = Vec::new();
        self.admins.iter().for_each(|addr| {
            admin_strs.push(addr.to_string());
        });

        let contract_addr = app
            .instantiate_contract(
                code_id,
                sender,
                &InstantiateMsg {
                    admins: admin_strs,
                    transferrable: self.transferrable,
                },
                &[],
                "name",
                None,
            )
            .unwrap();

        if self.setup_minter {
            app.execute_contract(
                self.admins[0].clone(),
                contract_addr.clone(),
                &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetMinter {
                    minter_address: self.registrar.to_string(),
                }),
                &[],
            )
            .unwrap();
        }

        TestEnv {
            app,
            code_id,
            contract_addr,
            admins: self.admins,
            registrar: self.registrar,
        }
    }
}
