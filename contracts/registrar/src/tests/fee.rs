#![cfg(test)]

use super::helpers::test_only_admin;
use crate::msg::{ExecuteMsg, FeeResponse, QueryMsg};
use cosmwasm_std::Coin;

#[test]
fn only_admin_can_set_fee() {
    test_only_admin(
        ExecuteMsg::SetFee {
            fee: Some(Coin::new(999999999, "uosmo")),
        },
        QueryMsg::Fee {},
        FeeResponse { fee: None },
        FeeResponse {
            fee: Some(Coin::new(999999999, "uosmo")),
        },
    );
}
