use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};
use near_sdk::json_types::U128;

#[test]
fn test_liquidity_for_price() {
    let (ref mut runtime, ref root) = init_markets_contract();

    let mut accounts: Vec<ExternalUser> = vec![];
    for acc_no in 0..2 {
        let acc = if let Ok(acc) =
            root.create_external(runtime, format!("account_{}", acc_no), ntoy(30))
        {
            acc
        } else {
            break;
        };
        accounts.push(acc);
    }

	accounts[0].claim_fdai(runtime);
	accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), market_end_timestamp_ms(), 0, 0, "test".to_string());

	accounts[0].place_order(runtime, 0, 0, U128(6000), U128(50));
	accounts[0].place_order(runtime, 0, 0, U128(6000), U128(50));
	accounts[0].place_order(runtime, 0, 0, U128(6000), U128(20));
	accounts[0].place_order(runtime, 0, 0, U128(8000), U128(20));

	let liquidity_60 = accounts[0].get_liquidity(runtime, 0, 0, 60);
	let liquidity_50 = accounts[0].get_liquidity(runtime, 0, 0, 50);
	let liquidity_20 = accounts[0].get_liquidity(runtime, 0, 0, 20);

	assert_eq!(liquidity_60, 0);
	assert_eq!(liquidity_50, 12000 / 50);
	assert_eq!(liquidity_20, 14000 / 20);

	accounts[0].cancel_order(runtime, 0,0,0);
	accounts[0].cancel_order(runtime, 0,0,1);
	accounts[0].cancel_order(runtime, 0,0,3);

	let liquidity_50 = accounts[0].get_liquidity(runtime, 0, 0, 50);
	let liquidity_20 = accounts[0].get_liquidity(runtime, 0, 0, 20);

	assert_eq!(liquidity_50, 0);
	assert_eq!(liquidity_20, 6000 / 20);

	accounts[0].place_order(runtime, 0, 1, U128(8000), U128(80));

	let liquidity_20 = accounts[0].get_liquidity(runtime, 0, 0, 20);
	let liquidity_80 = accounts[0].get_liquidity(runtime, 0, 1, 80);

	assert_eq!(liquidity_20, 4000 / 20);
	assert_eq!(liquidity_80, 0);
}

#[test]
fn test_valid_binary_market_depth() {
    let (ref mut runtime, ref root) = init_markets_contract();

    let mut accounts: Vec<ExternalUser> = vec![];
    for acc_no in 0..2 {
        let acc = if let Ok(acc) =
            root.create_external(runtime, format!("account_{}", acc_no), ntoy(30))
        {
            acc
        } else {
            break;
        };
        accounts.push(acc);
    }

	accounts[0].claim_fdai(runtime);
	accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 3, outcome_tags(3), categories(), market_end_timestamp_ms(), 0, 0, "test.com".to_string());

	accounts[0].place_order(runtime, 0, 0, U128(5000), U128(50));
	accounts[0].place_order(runtime, 0, 0, U128(6000), U128(60));

	accounts[1].claim_fdai(runtime);
	accounts[1].place_order(runtime, 0, 1, U128(2000), U128(20));
	accounts[1].place_order(runtime, 0, 1, U128(3000), U128(30));
	let depth_0 = accounts[1].get_depth(runtime, 0, 2, 10000, 100);
	let depth_1 = accounts[1].get_depth(runtime, 0, 1, 1000, 11);

    assert_eq!(depth_0, 4000);
	assert_eq!(depth_1, 0);
}

