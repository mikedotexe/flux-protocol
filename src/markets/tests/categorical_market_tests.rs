use super::*;

#[test]
fn test_categorical_market_matches() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market(3, "Hi!".to_string(), 100010101001010);

	println!("should log [100, 100]");
	contract.place_order(0, 0, 5000, 50);
	println!("");
	
	println!("should log [50, 100]");
	contract.place_order(0, 1, 5000, 20); 
	println!("");
	
	println!("should log [20, 50]");
	contract.place_order(0, 2, 5000, 30);
	println!("");

}