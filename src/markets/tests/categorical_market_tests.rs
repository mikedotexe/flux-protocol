use super::*;

#[test]
fn test_categorical_market_matches() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market(3, "Hi!".to_string(), 100010101001010);

	contract.place_order(0, 0, 100, 50);
	contract.place_order(0, 1, 100, 50); 

	// market prices testing: 
	// contract.place_order(0, 0, 100, 50);
	// contract.place_order(0, 1, 100, 20); 
	// contract.place_order(0, 2, 100, 20);

	// let mut market_0_price = contract.get_market_price(0, 0);
	// let mut market_1_price = contract.get_market_price(0, 1);
	// let mut market_2_price = contract.get_market_price(0, 2);
	// assert_eq!(market_0_price, 60);
	// assert_eq!(market_1_price, 30);
	// assert_eq!(market_2_price, 30);

	// contract.place_order(0,2, 5000, 29);

	// market_1_price = contract.get_market_price(0, 21);
	// assert_eq!(market_1_price, 1);


}