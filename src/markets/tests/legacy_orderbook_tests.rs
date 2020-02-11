// #[test]
	// fn test_market_orders() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default();
	// 	contract.claim_fdai();
	// 	contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
	// 	// Placing "no" order
	// 	contract.place_order(0, 0, 10000, 50);									
	// 	let market_no_order = contract.get_market_order(0, 0);
	// 	assert_eq!(market_no_order.is_none(), false);
		
	// 	contract.place_order(0, 1, 9000, 50);
	// 	contract.place_order(0, 1, 1000, 50);

	// 	let market_no_order = contract.get_market_order(0, 0);
	// 	let market_yes_order = contract.get_market_order(0, 1);
	// 	assert_eq!(market_no_order.is_none(), true);
	// 	assert_eq!(market_yes_order.is_none(), true);
	// }	

	// #[test]
	// fn test_fdai_balances() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default();
	// 	contract.claim_fdai();
	// 	let mut balance = contract.get_fdai_balance(carol());
	// 	let base: u64 = 10;
	// 	let mut expected_balance = 100 * base.pow(17);
	// 	let initial_balance = expected_balance;

	// 	assert_eq!(balance, &expected_balance);

	// 	contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
	// 	contract.place_order(0, 0, 40000, 40);
	// 	balance = contract.get_fdai_balance(carol());
	// 	expected_balance  = expected_balance - 40000;
	// 	assert_eq!(balance, &expected_balance);
		

	// 	testing_env!(get_context(bob()));
	// 	contract.claim_fdai();

	// 	contract.place_order(0, 1, 60000, 60);
	// 	balance = contract.get_fdai_balance(bob());
	// 	expected_balance = initial_balance - 60000;
	// 	assert_eq!(balance, &expected_balance);

	// 	testing_env!(get_context(carol()));
	// 	contract.resolute(0, vec![10000, 0], false);
	// 	contract.claim_earnings(0);
		
	// 	balance = contract.get_fdai_balance(carol());
	// 	expected_balance = initial_balance + 60000;
	// 	assert_eq!(balance, &expected_balance);
		
	// 	testing_env!(get_context(bob()));
	// 	balance = contract.get_fdai_balance(bob());
	// 	expected_balance = initial_balance - 60000;
	// 	assert_eq!(balance, &expected_balance);
	// }

	// #[test]
	// fn test_payout_open_orders_on_loss() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default();
	// 	contract.claim_fdai();
	// 	let mut balance = contract.get_fdai_balance(carol());
	// 	let base: u64 = 10;
	// 	let mut expected_balance = 100 * base.pow(17);
	// 	let initial_balance = expected_balance;

	// 	assert_eq!(balance, &expected_balance);

	// 	contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
	// 	contract.place_order(0, 0, 10000, 10);
	// 	contract.place_order(0, 0, 20000, 50);
		
	// 	testing_env!(get_context(bob()));
	// 	contract.claim_fdai();
		
	// 	contract.place_order(0, 1, 21000, 50);
	// 	contract.place_order(0, 1, 10000, 90);
		
	// 	testing_env!(get_context(carol()));
	// 	contract.resolute(0, vec![10000, 0], false); // carol wins
	// 	// contract.claim_earnings(0);
		
	// 	let claimable_carol = contract.get_earnings(0, carol());
	// 	let claimable_bob = contract.get_earnings(0, bob());
	// 	let expected_carol = 20000 + 40000;
	// 	let expected_bob = 1000;
	// 	let carol_delta = expected_carol - claimable_carol;
	// 	let bob_delta = expected_bob - claimable_bob;
	// 	assert!(carol_delta <= 100);
	// 	assert!(bob_delta <= 100);

	// }

	// #[test]
	// fn test_invalid_market() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default();
	// 	contract.claim_fdai();
	// 	contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
	// 	contract.place_order(0, 0, 7321893, 70);

	// 	testing_env!(get_context(bob()));
	// 	contract.claim_fdai();
	// 	contract.place_order(0, 1, 1232173, 30);

	// 	testing_env!(get_context(carol()));
	// 	contract.resolute(0, vec![5000, 5000], true);
	// 	let carol_earnings = contract.get_earnings(0, carol());
	// 	let bob_earnings = contract.get_earnings(0, bob());

	// 	println!("carol earnings: {} bob earnigns: {}", carol_earnings, bob_earnings);
	// 	// assert_eq!(bob_earnings, 50000);
	// 	let carol_old_balance = contract.get_fdai_balance(carol());
	// 	contract.claim_earnings(0);
	// 	println!(" ");
	// 	let carol_new_balance = contract.get_fdai_balance(carol());
	// 	println!("Carol's new balance {}" , carol_new_balance);
	// }
	
	// #[test]
	// fn test_get_open_orders() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default(); 
	// 	contract.claim_fdai();
	// 	contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 0, 40000, 40);

	// 	let open_orders = contract.get_open_orders(0, 0, carol());
	// 	assert_eq!(open_orders.len(), 5);
	// }

	// #[test]
	// fn test_get_filled_orders() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default();
	// 	contract.claim_fdai();
	// 	contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 1, 60000, 60);

	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 0, 40000, 40);
	// 	contract.place_order(0, 0, 40000, 40);

	// 	let open_orders = contract.get_open_orders(0, 0, carol());
	// 	let filled_orders = contract.get_filled_orders(0, 0, carol());
	// 	assert_eq!(open_orders.len(), 4);
	// 	assert_eq!(filled_orders.len(), 1);
	// }

	// #[test]
	// fn test_decimal_division_results() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default();
	// 	contract.claim_fdai();
	// 	contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
	// 	contract.place_order(0, 0, 1782361, 77);									

	// 	testing_env!(get_context(bob()));
	// 	contract.claim_fdai();

	// 	contract.place_order(0, 1, 123123123, 23);

	// 	testing_env!(get_context(carol()));
	// 	contract.resolute(0, vec![0, 10000], false);
		
	// 	testing_env!(get_context(bob()));
	// 	contract.claim_earnings(0);
	// 	let bob_balance = contract.get_fdai_balance(bob());

	// 	testing_env!(get_context(carol()));
	// 	let carol_balance = contract.get_fdai_balance(carol());
	// }	