use super::*;
use crate::markets::tests::utils::{init_markets_contract, ExternalUser, ntoy};
use near_sdk::json_types::U128;

#[test]
fn test_payout() {

    // == Init markets & token contract ==
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
    let account_0_id = accounts[0].get_account_id();
    let account_1_id = accounts[1].get_account_id();

    accounts[0].token_deploy_call_new(runtime, accounts[0].get_account_id().to_string(),  U128(10000000000000000));

    // == Begin Test ==
    accounts[0].claim_fdai(runtime);
    accounts[0].create_market(runtime, "Hi!".to_string(), empty_string(), 4, outcome_tags(4), categories(),  market_end_timestamp_ms(), 0, 0, "test".to_string());

	accounts[0].place_order(runtime, 0, 0, U128(10000), U128(70));
	accounts[0].place_order(runtime, 0, 3, U128(1000), U128(10));

	accounts[1].claim_fdai(runtime).unwrap();
	accounts[1].place_order(runtime, 0, 1, U128(1000), U128(10));
	accounts[1].place_order(runtime, 0, 2, U128(1000), U128(10));

	accounts[0].resolute_market(runtime, 0, None, to_dai(5));

	let initially_claimable_carol = accounts[0].get_claimable(runtime, 0, account_0_id.to_string());
	println!("Initially claimable CAROL");
	println!("{:?}", initially_claimable_carol);
	let initially_claimable_alice = accounts[0].get_claimable(runtime, 0, account_1_id.to_string());
	println!("Initially claimable ALICE");
	println!("{:?}", initially_claimable_carol);

	let initial_balance_carol = accounts[0].get_fdai_balance(runtime, account_0_id.to_string());
	let initial_balance_alice = accounts[0].get_fdai_balance(runtime, account_1_id.to_string());
	// TODO: Find way to accelerate block number;
	//testing_env!(get_context(carol(), market_end_timestamp_ns() + 1800000000000));

    accounts[0].finalize_market(runtime, 0, Some(0));
	accounts[0].claim_earnings(runtime, 0, account_0_id.to_string());
	accounts[0].claim_earnings(runtime, 0, account_1_id.to_string());

    // TODO: If failing, make sure that the correct account is calling the correct function
	let claimable_after_claim_carol = accounts[0].get_claimable(runtime, 0, account_0_id.to_string());
	println!("Claimable after CAROL");
    println!("{:?}", claimable_after_claim_carol);
	let claimable_after_claim_alice = accounts[0].get_claimable(runtime, 0, account_1_id.to_string());
	println!("Claimable after ALICE");
    println!("{:?}", claimable_after_claim_alice);

	let updated_balance_carol = accounts[0].get_fdai_balance(runtime, account_0_id.to_string());
	let updated_balance_alice = accounts[0].get_fdai_balance(runtime, account_1_id.to_string());

	assert_eq!(updated_balance_carol, initially_claimable_carol + initial_balance_carol);
	assert_eq!(updated_balance_alice, initially_claimable_alice + initial_balance_alice);

	assert_eq!(claimable_after_claim_carol, 0);
	assert_eq!(claimable_after_claim_alice, 0);
}
