#![cfg(test)]

use crate::{
    msg::{ExecuteMsg, ICNSNameExecuteMsg, TransferrableResponse},
    tests::helpers::{TestEnv, TestEnvBuilder},
    QueryMsg,
};

use cosmwasm_std::to_binary;
use cosmwasm_std::{Addr, Empty};
use cw721::OwnerOfResponse;
use cw721_base::{ContractError, ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};
use cw_multi_test::{BasicApp, Executor};

mod non_transferrable {
    use super::*;

    #[test]
    fn should_not_allow_transfer() {
        let TestEnv {
            mut app,
            contract_addr,
            registry,
            ..
        } = TestEnvBuilder::default().with_transferrable(false).build();

        let name_owner = Addr::unchecked("name_owner");
        let recipient = Addr::unchecked("recipient");
        let name = "alice";

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
    fn should_not_allow_send() {
        let TestEnv {
            mut app,
            contract_addr,
            registry,
            ..
        } = TestEnvBuilder::default().with_transferrable(false).build();

        let name_owner = Addr::unchecked("name_owner");
        let recipient_contract = Addr::unchecked("recipient_contract");
        let name = "alice";

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

        // send must be unauthorized
        let err = app
            .execute_contract(
                name_owner,
                contract_addr,
                &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::SendNft {
                    contract: recipient_contract.to_string(),
                    token_id: name.to_string(),
                    msg: to_binary("").unwrap(),
                }),
                &[],
            )
            .unwrap_err();

        assert_eq!(
            err.downcast_ref::<ContractError>().unwrap(),
            &ContractError::Unauthorized {}
        );
    }
}

mod transferrable {
    use crate::{tests::helpers::mock_reciever_contract, InstantiateMsg};

    use super::*;

    #[test]
    fn should_allow_transfer() {
        let TestEnv {
            mut app,
            contract_addr,
            registry,
            ..
        } = TestEnvBuilder::default().with_transferrable(true).build();

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

        // name is now owned by recipient
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
    fn should_allow_send() {
        let TestEnv {
            mut app,
            contract_addr,
            registry,
            admin,
            ..
        } = TestEnvBuilder::default().with_transferrable(true).build();

        let name_owner = Addr::unchecked("name_owner");

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

        let reciever_code_id = app.store_code(mock_reciever_contract());
        let reciever_contract_addr = app
            .instantiate_contract(reciever_code_id, admin, &(), &[], "name_clone", None)
            .unwrap();

        // send to recipient should succeed
        app.execute_contract(
            name_owner,
            contract_addr.clone(),
            &ExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::SendNft {
                contract: reciever_contract_addr.to_string(),
                token_id: name.to_string(),
                msg: to_binary(&()).unwrap(),
            }),
            &[],
        )
        .unwrap();

        // name is now owned by recipient
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

        assert_eq!(res.owner, reciever_contract_addr.to_string());
    }
}

#[test]
fn only_admin_can_set_transferrable() {
    let TestEnv {
        mut app,
        contract_addr,
        registry,
        admin,
        ..
    } = TestEnvBuilder::default().with_transferrable(false).build();

    let transferrable = |app: &BasicApp| {
        let TransferrableResponse { transferrable } = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Transferrable {})
            .unwrap();
        transferrable
    };

    let set_transferrable = |app: &mut BasicApp, sender: Addr, transferrable: bool| {
        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecuteMsg::ICNSName(ICNSNameExecuteMsg::SetTransferrable { transferrable }),
            &[],
        )
    };

    assert!(!transferrable(&app));

    // transferrable can't be set by random person
    let err = set_transferrable(&mut app, Addr::unchecked("random_person"), true).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert!(!transferrable(&app));

    // transferrable can't be set by registry
    let err = set_transferrable(&mut app, registry, true).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &ContractError::Unauthorized {}
    );
    assert!(!transferrable(&app));

    // transferrable can only be set by admin
    set_transferrable(&mut app, admin.clone(), true).unwrap();
    assert!(transferrable(&app));

    set_transferrable(&mut app, admin, false).unwrap();
    assert!(!transferrable(&app));
}
