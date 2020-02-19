use super::*;

#[test]
fn test_invalid_market_payout_calc() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market(4, "Hi!".to_string(), 100010101001010);

	let rounded_10k_at_70 = (10000 / 70) * 70;
	let mut amount_filled_after_cancel = 10000 - 2840;
	let expected_carol_balance = (2 * rounded_10k_at_70 + 3 * 10000) - amount_filled_after_cancel;
	contract.place_order(0, 0, 10000, 70);
	contract.place_order(0, 0, 10000, 70);
	contract.place_order(0, 1, 10000, 10); 
	contract.place_order(0, 2, 10000, 10); 
	contract.place_order(0, 3, 10000, 10);
	
	testing_env!(get_context(alice()));
	contract.claim_fdai();

	let rounded_10k_at_60 = (10000 / 60) * 60;
	contract.place_order(0, 0, 10000, 60);
	contract.place_order(0, 1, 10000, 20); 
	contract.place_order(0, 1, 10000, 20); 
	
	amount_filled_after_cancel = 10000 - 8880;
	let expected_alice_balance = (1 * rounded_10k_at_60 + 2 * 10000) - amount_filled_after_cancel;
	contract.cancel_order(0,1, 1);

	testing_env!(get_context(carol()));
	contract.cancel_order(0,1, 0);
	contract.resolute(0, None);
	let claimable_carol = contract.get_claimable(0, carol());
	let claimable_alice = contract.get_claimable(0, alice());
	assert_eq!(claimable_carol, expected_carol_balance);
	assert_eq!(claimable_alice, expected_alice_balance);
}



fn test_valid_market_payout_calc() {
	testing_env!(get_context(carol()));
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market(4, "Hi!".to_string(), 100010101001010);
	let rounded_10k_at_70 = (10000 / 70) * 70;
	let expected_carol_balance = (2 * rounded_10k_at_70 + 3 * 10000) - (10000 - 2840);
	contract.place_order(0, 0, 10000, 70);
	contract.place_order(0, 0, 10000, 70);
	contract.place_order(0, 1, 10000, 10); 
	contract.place_order(0, 2, 10000, 10); 
	contract.place_order(0, 3, 10000, 10);
	
	testing_env!(get_context(alice()));
	contract.claim_fdai();

	let rounded_10k_at_60 = (10000 / 60) * 60;
	let mut expected_alice_balance = 2 * rounded_10k_at_60 + 1 * 10000;
	contract.place_order(0, 0, 10000, 60);
	contract.place_order(0, 1, 10000, 20); 
	contract.place_order(0, 1, 10000, 20); 
	
	expected_alice_balance = (1 * rounded_10k_at_60 + 2 * 10000) - (10000 - 8880);
	let mut open_1_orders = contract.get_open_orders(0, 1, carol());
	println!("{:?}", open_1_orders);
	contract.cancel_order(0,1, 1);

	testing_env!(get_context(carol()));
	contract.cancel_order(0,1, 0);
	contract.resolute(0, None);
	let claimable_carol = contract.get_claimable(0, carol());
	let claimable_alice = contract.get_claimable(0, alice());
	assert_eq!(claimable_carol, expected_carol_balance);
	assert_eq!(claimable_alice, expected_alice_balance);
}