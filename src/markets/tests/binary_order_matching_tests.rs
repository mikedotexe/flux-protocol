use super::*;
#[test]
fn simplest_binary_order_matching_test() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), 100010101001010);

	contract.place_order(0, 0, 5000, 50);
	contract.place_order(0, 1, 5000, 50);

	let mut open_no_orders = contract.get_open_orders(0, 0, carol());
	let mut open_yes_orders = contract.get_open_orders(0, 1, carol());
	assert_eq!(open_no_orders.len(), 0);
	assert_eq!(open_yes_orders.len(), 0);
	let mut filled_no_orders = contract.get_filled_orders(0, 0, carol());
	let mut filled_yes_orders = contract.get_filled_orders(0, 1, carol());
	assert_eq!(filled_no_orders.len(), 1);
	assert_eq!(filled_yes_orders.len(), 1);
}

fn partial_binary_order_matching_test() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), 100010101001010);

	contract.place_order(0, 0, 5000, 50);
	contract.place_order(0, 1, 5000, 50);

	contract.place_order(0, 1, 5000, 50);
	contract.place_order(0, 1, 2750, 50);
	contract.place_order(0, 0, 7777, 50);

	let mut open_no_orders = contract.get_open_orders(0, 0, carol());
	let mut open_yes_orders = contract.get_open_orders(0, 1, carol());
	assert_eq!(open_no_orders.len(), 0);
	assert_eq!(open_yes_orders.len(), 0);
	let mut filled_no_orders = contract.get_filled_orders(0, 0, carol());
	let mut filled_yes_orders = contract.get_filled_orders(0, 1, carol());
	assert_eq!(filled_no_orders.len(), 1);
	assert_eq!(filled_yes_orders.len(), 2);
}

#[test]
fn rounding_binary_order_matching_test() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), 100010101001010);
	contract.place_order(0, 0, 8000, 80); // filled completely
	contract.place_order(0, 1, 3000, 30); // filled @ 20 && 30
	contract.place_order(0, 0, 2500, 70); 
	contract.place_order(0, 1, 120, 30); 

	let mut open_no_orders = contract.get_open_orders(0, 0, carol());
	let mut open_yes_orders = contract.get_open_orders(0, 1, carol());
	let mut filled_no_orders = contract.get_filled_orders(0, 0, carol());
	let mut filled_yes_orders = contract.get_filled_orders(0, 1, carol());
	
	assert_eq!(open_no_orders.len(), 0);
	assert_eq!(open_yes_orders.len(), 0);
	assert_eq!(filled_no_orders.len(), 2);
	assert_eq!(filled_yes_orders.len(), 2);

}