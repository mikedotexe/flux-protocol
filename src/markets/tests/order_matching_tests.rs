use super::*;

#[test]
fn test_binary_order_match() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market(2, "Hi!".to_string(), 100010101001010);


	// simplest binary fill scenario
	// contract.place_order(0, 0, 5000, 50);
	// contract.place_order(0, 1, 5000, 50);

	// let mut open_no_orders = contract.get_open_orders(0, 0, carol());
	// let mut open_yes_orders = contract.get_open_orders(0, 1, carol());
	// assert_eq!(open_no_orders.len(), 0);
	// assert_eq!(open_yes_orders.len(), 0);
	// let mut filled_no_orders = contract.get_filled_orders(0, 0, carol());
	// let mut filled_yes_orders = contract.get_filled_orders(0, 1, carol());
	// assert_eq!(filled_no_orders.len(), 1);
	// assert_eq!(filled_yes_orders.len(), 1);

	// simple binary fill scenario over multiple orders
	// contract.place_order(0, 1, 5000, 50);
	// contract.place_order(0, 1, 2750, 50);
	// contract.place_order(0, 0, 7777, 50);

	// let mut open_no_orders = contract.get_open_orders(0, 0, carol());
	// let mut open_yes_orders = contract.get_open_orders(0, 1, carol());
	// println!("open: {:?}", open_no_orders);
	// println!("open: {:?}", open_yes_orders);
	// assert_eq!(open_no_orders.len(), 0);
	// assert_eq!(open_yes_orders.len(), 0);
	// let mut filled_no_orders = contract.get_filled_orders(0, 0, carol());
	// let mut filled_yes_orders = contract.get_filled_orders(0, 1, carol());
	// println!("filled: {:?}", filled_no_orders);
	// println!("filled: {:?}", filled_yes_orders);

	// assert_eq!(filled_no_orders.len(), 1);
	// assert_eq!(filled_yes_orders.len(), 2);

	// Always fill the best price at the new limit orders' rate 
	// TODO: Discuss weither this makes sense 

	contract.place_order(0, 0, 8000, 80); // not filled completely
	contract.place_order(0, 1, 3000, 30); // filled @ 20
	contract.place_order(0, 1, 240, 20); // filled @ 20
	contract.place_order(0, 1, 240, 20); // filled @ 20

	let mut open_no_orders = contract.get_open_orders(0, 0, carol());
	let mut open_yes_orders = contract.get_open_orders(0, 1, carol());
	let mut filled_no_orders = contract.get_filled_orders(0, 0, carol());
	let mut filled_yes_orders = contract.get_filled_orders(0, 1, carol());

	println!("open no orders: {:?}", open_no_orders);
	println!("open yes orders: {:?}", open_yes_orders);
	println!("filled no orders: {:?}", filled_no_orders);
	println!("filled yes orders: {:?}", filled_yes_orders);
	
	assert_eq!(open_no_orders.len(), 0);
	assert_eq!(open_yes_orders.len(), 1);
	assert_eq!(filled_no_orders.len(), 1);
	assert_eq!(filled_yes_orders.len(), 2);

}