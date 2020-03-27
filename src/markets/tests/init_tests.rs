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
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), 100010101001010);
}