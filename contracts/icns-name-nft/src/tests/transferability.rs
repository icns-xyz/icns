#![cfg(test)]

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, ICNSNameExecuteMsg, TransferrableResponse},
    tests::helpers::{TestEnv, TestEnvBuilder},
    QueryMsg,
};

use cosmwasm_std::to_binary;
use cosmwasm_std::Addr;
use cw721::OwnerOfResponse;
use cw721_base::MintMsg;
use cw_multi_test::{BasicApp, Executor};

mod non_transferrable {
    use crate::{error::ContractError, msg::Metadata, tests::helpers::mock_reciever_contract};

    use super::*;

    #[test]
    fn should_not_allow_transfer() {
        let TestEnv {
            mut app,
            contract_addr,
            registrar,
            ..
        } = TestEnvBuilder::default().with_transferrable(false).build();

        let name_owner = Addr::unchecked("name_owner");
        let recipient = Addr::unchecked("recipient");
        let name = "alice";

        // registrar mint test icns_name
        app.execute_contract(
            registrar,
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
        .unwrap();

        // transfer must be unauthorized
        let err = app
            .execute_contract(
                name_owner,
                contract_addr,
                &ExecuteMsg::TransferNft {
                    recipient: recipient.to_string(),
                    token_id: name.to_string(),
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            err.downcast_ref::<ContractError>().unwrap(),
            &ContractError::TransferNotAllowed {}
        );
    }

    #[test]
    fn should_not_allow_send() {
        let TestEnv {
            mut app,
            contract_addr,
            registrar,
            ..
        } = TestEnvBuilder::default().with_transferrable(false).build();

        let name_owner = Addr::unchecked("name_owner");
        let recipient_contract = Addr::unchecked("recipient_contract");
        let name = "alice";

        // registrar mint test icns_name
        app.execute_contract(
            registrar,
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
        .unwrap();

        // send must be unauthorized
        let err = app
            .execute_contract(
                name_owner,
                contract_addr,
                &ExecuteMsg::SendNft {
                    contract: recipient_contract.to_string(),
                    token_id: name.to_string(),
                    msg: to_binary("").unwrap(),
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            err.downcast_ref::<ContractError>().unwrap(),
            &ContractError::TransferNotAllowed {}
        );
    }

    #[test]
    fn should_not_allow_registrar_to_transfer() {
        let TestEnv {
            mut app,
            contract_addr,
            registrar,
            ..
        } = TestEnvBuilder::default().with_transferrable(false).build();

        let name_owner = Addr::unchecked("name_owner");
        let name = "alice";

        // registrar mint test name
        app.execute_contract(
            registrar.clone(),
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name.to_string(),
                owner: registrar.to_string(),
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
        .unwrap();

        // transfer must be unauthorized
        let err = app
            .execute_contract(
                registrar,
                contract_addr,
                &ExecuteMsg::TransferNft {
                    recipient: name_owner.to_string(),
                    token_id: name.to_string(),
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            err.downcast_ref::<ContractError>().unwrap(),
            &ContractError::TransferNotAllowed {}
        );
    }

    #[test]
    fn should_allow_admin_to_send() {
        let TestEnv {
            mut app,
            contract_addr,
            registrar,
            admins,
            ..
        } = TestEnvBuilder::default().with_transferrable(false).build();

        let admin = admins[0].clone();

        // registrar mint test name
        app.execute_contract(
            registrar.clone(),
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: admin.to_string(),
                owner: admin.to_string(),
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
        .unwrap();

        app.execute_contract(
                admins[0].clone(),
                contract_addr.clone(),
                &ExecuteMsg::TransferNft {
                    recipient: admin.to_string(),
                    token_id: admin.to_string(),
                },
                &[],
        )
        .unwrap();

        // send to recipient should succeed
        let reciever_code_id = app.store_code(mock_reciever_contract());
        let reciever_contract_addr = app
            .instantiate_contract(
                reciever_code_id,
                admins[0].clone(),
                &(),
                &[],
                "name_clone",
                None,
            )
            .unwrap();

        app.execute_contract(
            admins[0].clone(),
            contract_addr.clone(),
            &ExecuteMsg::SendNft {
                contract: reciever_contract_addr.to_string(),
                token_id: admin.to_string(),
                msg: to_binary(&()).unwrap(),
            },
        &[],
        )
        .unwrap();
    }
}

mod transferrable {
    use crate::{msg::Metadata, tests::helpers::mock_reciever_contract};

    use super::*;

    #[test]
    fn should_allow_transfer() {
        let TestEnv {
            mut app,
            contract_addr,
            registrar,
            ..
        } = TestEnvBuilder::default().with_transferrable(true).build();

        let name_owner = Addr::unchecked("name_owner");
        let recipient = Addr::unchecked("recipient");
        let name = "alice";

        // mint test name
        app.execute_contract(
            registrar,
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
        .unwrap();

        // transfer to recipient should succeed
        app.execute_contract(
            name_owner,
            contract_addr.clone(),
            &ExecuteMsg::TransferNft {
                recipient: recipient.to_string(),
                token_id: name.to_string(),
            },
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
            registrar,
            admins,
            ..
        } = TestEnvBuilder::default().with_transferrable(true).build();

        let name_owner = Addr::unchecked("name_owner");

        let name = "alice";

        // mint test name
        app.execute_contract(
            registrar,
            contract_addr.clone(),
            &ExecuteMsg::Mint(MintMsg {
                token_id: name.to_string(),
                owner: name_owner.to_string(),
                token_uri: None,
                extension: Metadata { referral: None },
            }),
            &[],
        )
        .unwrap();

        let reciever_code_id = app.store_code(mock_reciever_contract());
        let reciever_contract_addr = app
            .instantiate_contract(
                reciever_code_id,
                admins[0].clone(),
                &(),
                &[],
                "name_clone",
                None,
            )
            .unwrap();

        // send to recipient should succeed
        app.execute_contract(
            name_owner,
            contract_addr.clone(),
            &ExecuteMsg::SendNft {
                contract: reciever_contract_addr.to_string(),
                token_id: name.to_string(),
                msg: to_binary(&()).unwrap(),
            },
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
        registrar,
        admins,
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
            &ExecuteMsg::Extension {
                msg: ICNSNameExecuteMsg::SetTransferrable { transferrable },
            },
            &[],
        )
    };

    assert!(!transferrable(&app));

    // transferrable can't be set by random person
    let err = set_transferrable(&mut app, Addr::unchecked("random_person"), true).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );
    assert!(!transferrable(&app));

    // transferrable can't be set by registrar
    let err = set_transferrable(&mut app, registrar, true).unwrap_err();
    assert_eq!(
        err.downcast_ref::<ContractError>().unwrap(),
        &cw721_base::ContractError::Unauthorized {}.into()
    );
    assert!(!transferrable(&app));

    // transferrable can only be set by admin
    set_transferrable(&mut app, admins[0].clone(), true).unwrap();
    assert!(transferrable(&app));

    set_transferrable(&mut app, admins[0].clone(), false).unwrap();
    assert!(!transferrable(&app));
}
