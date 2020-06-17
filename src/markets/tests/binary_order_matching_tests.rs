use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};
use near_sdk::json_types::U128;

#[test]
fn simplest_binary_order_matching_test() {
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

    accounts[0].token_deploy_call_new(runtime, accounts[0].get_account_id().to_string(),  U128(10000000000000000)).unwrap();

	accounts[0].claim_fdai(runtime);
	accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), market_end_timestamp_ms(), 0, 0, "test".to_string());

	accounts[0].place_order(runtime, 0, 0, U128(5000), U128(50));
	accounts[0].place_order(runtime, 0, 1, U128(5000), U128(50));

	let open_no_orders = accounts[0].get_open_orders(runtime, 0, 0);
	let open_yes_orders = accounts[0].get_open_orders(runtime, 0, 1);
	assert_eq!(open_no_orders.len(), 0);
	assert_eq!(open_yes_orders.len(), 0);
	let filled_no_orders = accounts[0].get_filled_orders(runtime, 0, 0);
	let filled_yes_orders = accounts[0].get_filled_orders(runtime, 0, 1);
	assert_eq!(filled_no_orders.len(), 1);
	assert_eq!(filled_yes_orders.len(), 1);
}

fn partial_binary_order_matching_test() {
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

	accounts[0].place_order(runtime, 0, 0, U128(5000), U128(50));
	accounts[0].place_order(runtime, 0, 1, U128(5000), U128(50));

	accounts[0].place_order(runtime, 0, 1, U128(5000), U128(50));
	accounts[0].place_order(runtime, 0, 1, U128(2750), U128(50));
	accounts[0].place_order(runtime, 0, 0, U128(7777), U128(50));

	let open_no_orders = accounts[0].get_open_orders(runtime, 0, 0);
	let open_yes_orders = accounts[0].get_open_orders(runtime, 0, 1);
	assert_eq!(open_no_orders.len(), 0);
	assert_eq!(open_yes_orders.len(), 0);
	let filled_no_orders = accounts[0].get_filled_orders(runtime, 0, 0);
	let filled_yes_orders = accounts[0].get_filled_orders(runtime, 0, 1);
	assert_eq!(filled_no_orders.len(), 1);
	assert_eq!(filled_yes_orders.len(), 2);
}
