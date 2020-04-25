use super::*;

#[test]
fn test_dispute_valid() {
    testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp(), 4, 2, "test".to_string());

	contract.place_order(0, 0, to_dai(10), 70);
	contract.place_order(0, 3, to_dai(1), 10);

	testing_env!(get_context(alice(), current_block_timestamp()));
	contract.claim_fdai();
	contract.place_order(0, 1, to_dai(1), 10);
	contract.place_order(0, 2, to_dai(1), 10);

	testing_env!(get_context(carol(), market_end_timestamp()));
    contract.resolute_market(0, Some(0));
    testing_env!(get_context(alice(), market_end_timestamp()));
	contract.dispute_market(0, Some(1));
    testing_env!(get_context(judge(), market_end_timestamp()));
    contract.finalize_market(0, Some(0));


    let claimable_carol = contract.get_claimable(0, carol()) ;
    let claimable_alice = contract.get_claimable(0, alice()) ;

    assert_eq!(claimable_carol, to_dai(23) - 50);
    assert_eq!(claimable_alice, 0);
}

#[test]
#[should_panic(expected = "Divide result is zero")]
fn test_market_not_resoluted() {
	testing_env!(get_context(carol(), current_block_timestamp()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp(), 4, 2, "test".to_string());
	contract.dispute_market(0, Some(0))
}

