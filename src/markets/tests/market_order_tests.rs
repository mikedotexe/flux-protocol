use super::*;

#[test]
fn test_market_orders() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), 100010101001010);

	// simplest binary fill scenario
	contract.place_order(0, 1, 5000, 50); // 0 
	contract.place_order(0, 1, 5000, 50); // 1

	let mut yes_market_price = contract.get_market_price(0, 0);
	assert_eq!(yes_market_price, 50);

	contract.place_order(0, 1, 5000, 60); // 2
	yes_market_price= contract.get_market_price(0, 0);
	println!("{}", yes_market_price);
	assert_eq!(yes_market_price, 40);

	contract.cancel_order(0, 1, 2, 60);
	yes_market_price = contract.get_market_price(0, 0);
	assert_eq!(yes_market_price, 50);

	contract.cancel_order(0, 1, 1, 50);
	yes_market_price = contract.get_market_price(0, 0);
	assert_eq!(yes_market_price, 50);
	
	contract.cancel_order(0, 1, 0, 50);
	yes_market_price = contract.get_market_price(0, 0);
	assert_eq!(yes_market_price, 100);

}