use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};
use near_sdk::json_types::U128;

#[test]
fn test_invalid_market_payout_calc() {
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

	accounts[0].claim_fdai(runtime);
	accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp_ms(), 0, 0, "test".to_string());

	accounts[0].place_order(runtime, 0, 0, U128(7000), U128(70));
	accounts[0].place_order(runtime, 0, 1, U128(1000), U128(10));
	accounts[0].place_order(runtime, 0, 2, U128(1000), U128(10));
	accounts[0].place_order(runtime, 0, 3, U128(1000), U128(10));

	accounts[1].claim_fdai(runtime);

	accounts[1].place_order(runtime, 0, 0, U128(6000), U128(60));
	accounts[1].place_order(runtime, 0, 1, U128(2000), U128(20));
	accounts[1].place_order(runtime, 0, 2, U128(2000), U128(20));

    // TODO: HOW TO ADVANCE BLOCK?
	//testing_env!(get_context(carol(), market_end_timestamp_ns()));
	accounts[0].resolute_market(runtime, 0, None, to_dai(5));

	let claimable_carol = accounts[0].get_claimable(runtime, 0, accounts[0].get_account_id().to_string());
	let claimable_alice = accounts[0].get_claimable(runtime, 0, accounts[1].get_account_id().to_string());
	assert_eq!(claimable_carol, 10000 + u128::from(to_dai(5)));
	assert_eq!(claimable_alice, 10000);

	let open_orders_0 = accounts[0].get_open_orders(runtime, 0, 0);
	let open_orders_1 = accounts[0].get_open_orders(runtime, 0, 1);
	let open_orders_2 = accounts[0].get_open_orders(runtime, 0, 2);
	let open_orders_3 = accounts[0].get_open_orders(runtime, 0, 3);

	assert_eq!(open_orders_0.len(), 0);
	assert_eq!(open_orders_1.len(), 0);
	assert_eq!(open_orders_2.len(), 0);
	assert_eq!(open_orders_3.len(), 0);

	let filled_orders_0 = accounts[0].get_filled_orders(runtime, 0, 0);
	let filled_orders_1 = accounts[0].get_filled_orders(runtime, 0, 1);
	let filled_orders_2 = accounts[0].get_filled_orders(runtime, 0, 2);
	let filled_orders_3 = accounts[0].get_filled_orders(runtime, 0, 3);

	assert_eq!(filled_orders_0.len(), 2);
	assert_eq!(filled_orders_1.len(), 2);
	assert_eq!(filled_orders_2.len(), 2);
	assert_eq!(filled_orders_3.len(), 1);

}

#[test]
fn test_valid_market_payout_calc() {
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

	accounts[0].claim_fdai(runtime);
	accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(), market_end_timestamp_ms(), 0, 0, "test".to_string());

	accounts[0].place_order(runtime, 0, 0, U128(7000), U128(70));

	accounts[1].claim_fdai(runtime);
	accounts[1].place_order(runtime, 0, 1, U128(1000), U128(10));
	accounts[1].place_order(runtime, 0, 2, U128(2000), U128(20));

    // TODO: ADVANCE BLOCK NUM PROPERLY
	//testing_env!(get_context(carol(), market_end_timestamp_ns()));
	accounts[0].resolute_market(runtime, 0, Some(1), to_dai(5));

	let open_orders_0 = accounts[0].get_open_orders(runtime, 0, 0);
	let open_orders_1 = accounts[0].get_open_orders(runtime, 0, 1);
	let open_orders_2 = accounts[0].get_open_orders(runtime, 0, 2);

	assert_eq!(open_orders_0.len(), 0);
	assert_eq!(open_orders_1.len(), 0);
	assert_eq!(open_orders_2.len(), 0);

	let filled_orders_0 = accounts[0].get_filled_orders(runtime, 0, 0);
	let filled_orders_1 = accounts[0].get_filled_orders(runtime, 0, 1);
	let filled_orders_2 = accounts[0].get_filled_orders(runtime, 0, 2);

	assert_eq!(filled_orders_0.len(), 1);
	assert_eq!(filled_orders_1.len(), 1);
	assert_eq!(filled_orders_2.len(), 1);

	let claimable_carol = accounts[0].get_claimable(runtime, 0, accounts[0].get_account_id().to_string()) ;
	let claimable_alice = accounts[0].get_claimable(runtime, 0, accounts[1].get_account_id().to_string()) ;

	assert_eq!(claimable_carol, u128::from(to_dai(5)));
	assert_eq!(claimable_alice, 10000);
}
