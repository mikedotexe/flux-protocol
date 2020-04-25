use super::*;

fn init_tests() -> Markets {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp(), 4, 2, "test".to_string());
	return contract;
}

// #[test]
// fn test_dispute_valid() {
// 	let mut contract = init_tests();

// 	contract.place_order(0, 0, to_dai(10), 70);
// 	contract.place_order(0, 3, to_dai(1), 10);

// 	testing_env!(get_context(alice(), current_block_timestamp()));
// 	contract.claim_fdai();
// 	contract.place_order(0, 1, to_dai(1), 10);
// 	contract.place_order(0, 2, to_dai(1), 10);

// 	testing_env!(get_context(carol(), market_end_timestamp()));
//     contract.resolute_market(0, Some(0));
//     testing_env!(get_context(alice(), market_end_timestamp()));
// 	contract.dispute_market(0, Some(1));
//     testing_env!(get_context(judge(), market_end_timestamp()));
//     contract.finalize_market(0, Some(0));


//     let claimable_carol = contract.get_claimable(0, carol()) ;
//     let claimable_alice = contract.get_claimable(0, alice()) ;

//     assert_eq!(claimable_carol, to_dai(23) - 50);
//     assert_eq!(claimable_alice, 0);
// }

// #[test]
// #[should_panic(expected = "market isn't resoluted yet")]
// fn test_market_not_resoluted() {
// 	let mut contract = init_tests();
// 	contract.dispute_market(0, Some(0));
// }

// #[test]
// #[should_panic(expected = "market is already finalized")]
// fn test_finalized_market() {
// 	let mut contract = init_tests();
// 	testing_env!(get_context(carol(), market_end_timestamp()));
// 	contract.resolute_market(0, Some(0));
// 	testing_env!(get_context(judge(), market_end_timestamp() + 1800));
// 	contract.finalize_market(0, None);
// 	contract.dispute_market(0, Some(1));
// }
// #[test]
// #[should_panic(expected = "dispute window still open")]
// fn test_market_finalization_pre_dispute_window_close() {
// 	let mut contract = init_tests();
// 	testing_env!(get_context(carol(), market_end_timestamp()));
//     contract.resolute_market(0, Some(0));
// 	contract.finalize_market(0, None);
// }

// #[test]
// #[should_panic(expected = "dispute window is closed, market can be finalized")]
// fn test_dispute_after_dispute_window() {
// 	let mut contract = init_tests();
// 	testing_env!(get_context(carol(), market_end_timestamp()));
// 	contract.resolute_market(0, Some(0));
// 	testing_env!(get_context(carol(), market_end_timestamp() + 1801));
// 	contract.dispute_market(0, None);
// }

// #[test]
// #[should_panic(expected = "only the owner can resolute disputed markets")]
// fn test_finalize_as_not_owner() {
// 	let mut contract = init_tests();
// 	testing_env!(get_context(carol(), market_end_timestamp()));
// 	contract.resolute_market(0, Some(0));
// 	contract.dispute_market(0, None);
// 	testing_env!(get_context(carol(), market_end_timestamp() + 1800));
// 	contract.finalize_market(0, None);
// }

#[test]
#[should_panic]
fn test_invalid_dispute_outcome() {
	let mut contract = init_tests();
	testing_env!(get_context(carol(), market_end_timestamp()));
	contract.resolute_market(0, Some(4));
	contract.dispute_market(0, Some(4));
}

// Test cases
// try to escalete dispute
// disputes should be able to be crowdfunded