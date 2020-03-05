use super::*;

#[test]
fn test_categorical_market_matches() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), 100010101001010);

	contract.place_order(0, 0, 70000, 70);
	contract.place_order(0, 0, 10000, 70);
	contract.place_order(0, 1, 10000, 10); 
	contract.place_order(0, 2, 10000, 10); 
	contract.place_order(0, 3, 10000, 10); 

	let mut open_0_orders = contract.get_open_orders(0, 0, carol());
	let mut open_1_orders = contract.get_open_orders(0, 1, carol());
	let mut open_2_orders = contract.get_open_orders(0, 2, carol());
	let mut filled_0_orders = contract.get_filled_orders(0, 0, carol());
	let mut filled_1_orders = contract.get_filled_orders(0, 1, carol());
	let mut filled_2_orders = contract.get_filled_orders(0, 2, carol());

	// // uncomment for orderbook state check
	// println!("{:?}", open_0_orders);
	// println!("{:?}", open_1_orders);
	// println!("{:?}", open_2_orders);

}