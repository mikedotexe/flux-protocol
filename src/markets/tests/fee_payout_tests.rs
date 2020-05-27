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

	testing_env!(get_context(alice(), current_block_timestamp()));
	contract.claim_fdai();

	contract.place_order(0, 0, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);

	contract.place_order(0, 1, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);
	contract.place_order(0, 1, 5 * one_dai, 50, None);

	let markets = contract.get_markets_by_id(vec![0]);
	assert_eq!(markets[&0].filled_volume, 10 * one_dai);

	testing_env!(get_context(bob(), market_end_timestamp_ns()));
	contract.claim_fdai();
	contract.resolute_market(0, Some(1), 5 * one_dai);
	testing_env!(get_context(bob(), market_end_timestamp_ns() + 1800000000000));
	contract.finalize_market(0, Some(1));

	testing_env!(get_context(alice(), market_end_timestamp_ns() + 1800000000000));
	let market = contract.get_markets_by_id(vec![0])[&0];
	let claimable_creator = contract.get_claimable(0, carol());
	let expected_creator_fee = 0;

	assert_eq!(expected_creator_fee, claimable_creator);

	// let creator_balance_before_claim = contract.get_fdai_balance(alice());
	let claimable_trader = contract.get_claimable(0, alice());
	let expected_claimable_trader = 20 * one_dai + (20 * one_dai - (market.resolution_fee_percentage * 10 * one_dai / 100) - (market.creator_fee_percentage * 10 * one_dai / 100));
	assert_eq!(claimable_trader, expected_claimable_trader);

}