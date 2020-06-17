use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};
use near_sdk::json_types::U128;

#[test]
fn test_market_orders() {
    let (ref mut runtime, ref root) = init_markets_contract();

    let mut accounts: Vec<ExternalUser> = vec![];
    for acc_no in 0..2 {
        let acc = if let Ok(acc) =
            root.create_external(runtime, format!("account_{}", acc_no), ntoy(30))
        {
            acc
        } else {
            break;
        };
        accounts.push(acc);
    }

    accounts[0].token_deploy_call_new(runtime, accounts[0].get_account_id().to_string(),  U128(10000000000000000)).unwrap();

	accounts[0].claim_fdai(runtime);
	accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 2, outcome_tags(0), categories(), market_end_timestamp_ms(), 0, 0, "test".to_string());

	// simplest binary fill scenario
	accounts[0].place_order(runtime, 0, 1, U128(5000), U128(50)); // 0
	accounts[0].place_order(runtime, 0, 1, U128(5000), U128(50)); // 1

	let mut yes_market_price = accounts[0].get_market_price(runtime, 0, 0);
	assert_eq!(yes_market_price, 50);

	accounts[0].place_order(runtime, 0, 1, U128(5000), U128(60)); // 2
	yes_market_price = accounts[0].get_market_price(runtime, 0, 0);
	assert_eq!(yes_market_price, 40);

	accounts[0].cancel_order(runtime, 0, 1, 2);
	yes_market_price = accounts[0].get_market_price(runtime, 0, 0);
	assert_eq!(yes_market_price, 50);

	accounts[0].cancel_order(runtime, 0, 1, 1);
	yes_market_price = accounts[0].get_market_price(runtime, 0, 0);
	assert_eq!(yes_market_price, 50);

	accounts[0].cancel_order(runtime, 0, 1, 0);
	yes_market_price = accounts[0].get_market_price(runtime, 0, 0);
	assert_eq!(yes_market_price, 100);

}
