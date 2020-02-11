use super::*;

#[test]
fn test_binary_order_match() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market(2, "Hi!".to_string(), 100010101001010);


	// simplest binary fill scenario
	contract.place_order(0, 0, 5000, 50);
	contract.place_order(0, 1, 5000, 50);
	
	// slightly more complex fill scnerio
	// contract.place_order(0, 0, 6666, 50);
	// contract.place_order(0, 1, 3444, 50);

	// contract.place_order(0, 0, 6000, 60); // 0: filled: 0
	// contract.place_order(0, 1, 3000, 30); // 1: filled: 0
	// contract.place_order(0, 1, 5000, 50); // 0: filled: 5000/6000 & 1 share | 1: filled: 0 & 0 shares | 2: filled: 5000/5000 & 1 share


}