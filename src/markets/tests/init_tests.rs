use super::*;

#[test]
fn test_contract_creation() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default(); 
}

#[test]
fn test_market_creation() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default(); 
	contract.create_market(2, "Hi!".to_string(), 100010101001010);
}