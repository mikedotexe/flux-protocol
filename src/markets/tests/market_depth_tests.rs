use super::*;

#[test]
fn test_valid_market_depth() {
	testing_env!(get_context(carol(), current_block_timestamp()));	
	let mut contract = Markets::default();
	contract.claim_fdai();
	contract.create_market("Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp());

	contract.place_order(0, 0, 6000, 60);

	testing_env!(get_context(alice(), current_block_timestamp()));
	contract.claim_fdai();
	contract.place_order(0, 1, 1000, 10);
	contract.place_order(0, 2, 1000, 23);
	contract.place_order(0, 2, 2000, 22);

    let market_0 = contract.active_markets.get(&0).unwrap();
    let orderbook_0 = market_0.orderbooks.get(&0).unwrap();
    let mut yes_market_price = contract.get_market_price(0, 0);
    let depth_0 = orderbook_0.get_liquidity(1000, 75);

	let market_1 = contract.active_markets.get(&0).unwrap();
    let orderbook_1 = market_1.orderbooks.get(&1).unwrap();
    let depth_1 = orderbook_1.get_liquidity(1000, 11);

    let market_2 = contract.active_markets.get(&0).unwrap();
    let orderbook_2 = market_2.orderbooks.get(&2).unwrap();
    let depth_2 = orderbook_2.get_liquidity(5000, 24);

    assert_eq!(depth_0, (60 , 16, 960));
	assert_eq!(depth_1, (10, 100, 1000));
	assert_eq!(depth_2, (23, 133, 2969));
}
