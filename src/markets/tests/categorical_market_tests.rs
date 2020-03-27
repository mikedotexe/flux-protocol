use super::*;

#[test]
fn test_categorical_market_matches() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 3, outcome_tags(3), categories(), 100010101001010);

	contract.place_order(0, 0, 5000000000000000000, 25);
	contract.place_order(0, 1, 5000000000000000000, 65);
	contract.place_order(0, 2, 5000000000000000000, 5);
	contract.place_order(0, 0, 100000000000000000, 31);

	let open_0_orders = contract.get_open_orders(0, 0, carol());
    let open_1_orders = contract.get_open_orders(0, 1, carol());
    let open_2_orders = contract.get_open_orders(0, 2, carol());
    let filled_0_orders = contract.get_filled_orders(0, 0, carol());
    let filled_1_orders = contract.get_filled_orders(0, 1, carol());
    let filled_2_orders = contract.get_filled_orders(0, 2, carol());


	// // uncomment for orderbook state check
	// println!("{:?}", open_0_orders);
	// println!("{:?}", open_1_orders);
	// println!("{:?}", open_2_orders);

	// assertions for the orderbook lengths
	assert_eq!(open_0_orders.len(), 2);
	assert_eq!(open_1_orders.len(), 1);
	assert_eq!(open_2_orders.len(), 1);
	assert_eq!(filled_0_orders.len(), 0);
	assert_eq!(filled_1_orders.len(), 0);
	assert_eq!(filled_2_orders.len(), 0);
}