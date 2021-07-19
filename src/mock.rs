#![cfg(test)]

use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{to_binary, OwnedDeps, Querier, QuerierResult, SystemResult, Uint128};
use cw20::BalanceResponse;

pub fn mock_dependencies_cw20_balance(
    balance: Uint128,
) -> OwnedDeps<MockStorage, MockApi, BalMockQuerier> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: BalMockQuerier { amount: balance },
    }
}

pub struct BalMockQuerier {
    amount: Uint128,
}

impl Querier for BalMockQuerier {
    fn raw_query(&self, _: &[u8]) -> QuerierResult {
        let balance_res = BalanceResponse {
            balance: self.amount,
        };

        SystemResult::Ok(to_binary(&balance_res).into())
    }
}
