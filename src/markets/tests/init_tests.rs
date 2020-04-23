use super::*;

#[test]
fn test_contract_creation() {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
}

#[test]
fn test_market_creation() {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp(), 0, 0, "test".to_string());
}
