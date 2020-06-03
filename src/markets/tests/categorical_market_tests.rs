use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};

#[test]
fn test_categorical_market_automated_matcher() {
	//testing_env!(get_context(carol(), current_block_timestamp()));

    // TODO: Initialize block timestamp
    let (ref mut runtime, ref root) = init_markets_contract();

    // Call claim_fdai, create market, place orders
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

    accounts[0].token_init_new(runtime, accounts[0].get_account_id().to_string(), 10000000000000000).unwrap();

    // Call claim_fdai, create market
    accounts[0].claim_fdai(runtime).unwrap();

    accounts[0].get_fdai_metrics(runtime);

    accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 3, outcome_tags(3), categories(),  market_end_timestamp_ms(), 0, 0, "test".to_string()).unwrap();

    // best prices - market price = 10
    accounts[0].place_order(runtime, 0, 0, 3000, 30);
    accounts[0].place_order(runtime, 0, 1, 6000, 60);

    // worse prices - market price = 25
    accounts[0].place_order(runtime, 0, 0, 2500, 25);
    accounts[0].place_order(runtime, 0, 1, 5000, 50);

    // alice fills all orders
    accounts[1].claim_fdai(runtime).unwrap();
    accounts[1].place_order(runtime, 0, 2, 3500, 25);

    let open_0_orders = accounts[1].get_open_orders(runtime, 0, 0);
    let open_1_orders = accounts[1].get_open_orders(runtime, 0, 1);
    let open_2_orders = accounts[1].get_open_orders(runtime, 0, 2);

    let filled_0_orders = accounts[1].get_filled_orders(runtime, 0, 0);
    let filled_1_orders = accounts[1].get_filled_orders(runtime, 0, 1);
	let filled_2_orders = accounts[1].get_filled_orders(runtime, 0, 2);

	//// uncomment for orderbook state check
	// println!("open orders outcome 0: {:?}", open_0_orders);
	// println!("____________________________________________________");
	// println!("open orders outcome 1: {:?}", open_1_orders);
	// println!("____________________________________________________");
	// println!("open orders outcome 2: {:?}", open_2_orders);
	// println!("____________________________________________________");
	// println!("filled orders outcome 0: {:?}", filled_0_orders);
	// println!("____________________________________________________");
	// println!("filled orders outcome 1: {:?}", filled_1_orders);
	// println!("____________________________________________________");
	// println!("filled orders outcome 2: {:?}", filled_2_orders);
	// println!("____________________________________________________");

	// assertions for the orderbook lengths
	assert_eq!(open_0_orders.len(), 0);
	assert_eq!(open_1_orders.len(), 0);
	assert_eq!(open_2_orders.len(), 0);
	assert_eq!(filled_0_orders.len(), 0);
	assert_eq!(filled_1_orders.len(), 0);
	assert_eq!(filled_2_orders.len(), 0);
}
