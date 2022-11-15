#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Coin, Deps, DepsMut, Uint128};
    use cw_utils::Duration;

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{
        ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse, ReverseResolveRecordResponse,
    };
    use crate::state::Config;

    #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        // mock_init_with_price(
        //     deps.as_mut(),
        //     "token",
        //     Uint128::from(3u128),
        //     Uint128::from(100u128),
        //     Duration::Time(7_776_000),
        // );

        // assert_config_state(
        //     deps.as_ref(),
        //     Config {
        //         required_denom: "token".to_string(),
        //         register_price: Uint128::from(3u128),
        //         annual_tax_bps: Uint128::from(100u128),
        //         owner_grace_period: Duration::Time(7_776_000),
        //     },
        // );
    }
}