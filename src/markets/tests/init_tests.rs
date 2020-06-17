use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};
use near_sdk::json_types::U128;

#[test]
fn test_contract_creation() {
    let (ref mut runtime, ref root) = init_markets_contract();
}

#[test]
fn test_market_creation() {
    // == Init markets & token contract ==
    let (ref mut runtime, ref root) = init_markets_contract();
    let acc = root.create_external(runtime, format!("account_{}", 0), ntoy(30)).ok().unwrap();

    acc.token_deploy_call_new(runtime, acc.get_account_id().to_string(),  U128(10000000000000000));
	acc.create_market(runtime, "Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp_ms(), 0, 0, "test".to_string());
}
