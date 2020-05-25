use super::*;

fn init_tests() -> Markets {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp_ms(), 4, 50, "test".to_string());
	return contract;
}

#[test]
fn creator_fee_test() {
	let mut contract = init_tests();
	let one_dai = to_dai(1);

	contract.place_order(0, 0, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);

	contract.place_order(0, 1, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);

	let markets = contract.get_markets_by_id(vec![0]);
	assert_eq!(markets[&0].filled_volume, 10 * one_dai);

	
}