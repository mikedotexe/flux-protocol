use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};
use near_sdk::json_types::U128;

#[test]
fn test_categorical_market_automated_matcher() {

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

    // Call claim_fdai, create market
    accounts[0].get_fdai_metrics(runtime);

    // best prices - market price = 10
    accounts[0].place_order(runtime, 0, 1, U128(6000), U128(60));

    // worse prices - market price = 25
    accounts[0].place_order(runtime, 0, 0, U128(2500), U128(25));
    accounts[0].place_order(runtime, 0, 1, U128(5000), U128(50));

    // alice fills all orders
    accounts[1].place_order(runtime, 0, 2, U128(3500), U128(25));

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
	assert_eq!(filled_0_orders.len(), 2);
	assert_eq!(filled_1_orders.len(), 2);
	assert_eq!(filled_2_orders.len(), 1);
}
