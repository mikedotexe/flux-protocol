use near_sdk::{near_bindgen, env, ext_contract, Promise, callback_vec, callback};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};

mod market;
type Market = market::Market;
type Order = market::orderbook::order::Order;
type ResolutionWindow = market::ResolutionWindow;

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
struct Markets {
	creator: String,
	active_markets: BTreeMap<u64, Market>,
	nonce: u64,
	fdai_balances: HashMap<String, u128>, // Denominated in 1e18
	fdai_circulation: u128,
	fdai_in_protocol: u128,
	fdai_outside_escrow: u128,
	user_count: u64,
	max_fee_percentage: u128,
	creation_bond: u128,
}

// Prepaid gas for making a single simple call.
const SINGLE_CALL_GAS: u64 = 200000000000000;

// If the name is not provided, the namespace for generated methods in derived by applying snake
// case to the trait name, e.g. ext_fungible_token
#[ext_contract]
pub trait ExtFungibleToken {
    fn transfer_from(&mut self, owner_id: String, new_owner_id: String, amount: u128);
    fn transfer(&mut self, new_owner_id: String, amount: u128);
    fn get_total_supply(&self) -> u128;
    fn get_balance(&self, owner_id: AccountId) -> u128;
}

#[ext_contract(ext)]
pub trait ExtContract {
    fn check_not_claimed(&mut self);
    fn grant_fdai(&mut self, from: String);
    fn check_sufficient_balance(&mut self, spend: u128);
    fn update_fdai_metrics(&mut self);
    fn update_fdai_metrics_subtract(&mut self, amount: u128);
    fn update_fdai_metrics_add(&mut self, amount: u128);
    fn purchase_shares(&mut self, from: String, market_id: u64, outcome: u64, spend: u128, price: u128);
    fn resolute_approved(&mut self, market_id: u64, winning_outcome: Option<u64>, stake: u128);
}

#[near_bindgen]
impl Markets {
    pub fn deploy_fungible_token(&self, from: String, amount: u64) {
        Promise::new(from)
            .create_account()
            .transfer(amount as u128)
            .add_full_access_key(env::signer_account_pk())
            .deploy_contract(
                include_bytes!("../res/fungible_token.wasm").to_vec(),
            );
    }

	fn dai_token(
		&self
	) -> u128 {
		let base: u128 = 10;
		return base.pow(17)
	}

	// This is a demo method, it mints a currency to interact with markets until we have NDAI
	pub fn claim_fdai(
	    &mut self
    ) {
		let from = env::predecessor_account_id();
		ext_fungible_token::get_balance(from.to_string(), &from.to_string(), 0, SINGLE_CALL_GAS).then(
		    ext::check_not_claimed(&env::current_account_id(), 0, SINGLE_CALL_GAS)
        ).then(
		    ext::grant_fdai(from.to_string(), &env::current_account_id(), 0, SINGLE_CALL_GAS)
		).then(
		    ext::update_fdai_metrics(&env::current_account_id(), 0, SINGLE_CALL_GAS)
		);
	}

	#[callback_vec(amount)]
    pub fn check_not_claimed(&mut self, amount: u128) -> Result<bool, String> {
        if amount != 0 {
            return Ok(amount != 0);
        } else {
            return Err("user has already claimed fdai".to_string());
        }
    }

    #[callback_vec(check_result)]
    pub fn grant_fdai(&mut self, from: String, check_result: Result<bool, String>) {
        if let Ok(true) = check_result {
            let claim_amount = 100 * self.dai_token();
            ext_fungible_token::transfer_from(env::current_account_id(), from, claim_amount, &env::current_account_id(), 0, SINGLE_CALL_GAS);
        }
    }

    #[callback]
    pub fn update_fdai_metrics_claim(&mut self) {
        // TODO: Determine if above call was a success
        let claim_amount = 100 * self.dai_token();
        self.fdai_circulation = self.fdai_circulation + claim_amount as u128;
        self.fdai_outside_escrow = self.fdai_outside_escrow + claim_amount as u128;
        self.user_count = self.user_count + 1;
    }

	pub fn get_fdai_balance(&self, from: String) -> () {
	    // TODO: Return correct balance here
	    ext_fungible_token::get_balance(from.to_string(), &from.to_string(), 0, SINGLE_CALL_GAS).as_return();
		//return *self.fdai_balances.get(&from).unwrap();
	}

	pub fn create_market(
		&mut self,
		description: String,
		extra_info: String,
		outcomes: u64,
		outcome_tags: Vec<String>,
		categories: Vec<String>,
		end_time: u64,
		fee_percentage: u128,
		cost_percentage: u128,
		api_source: String
	) -> u64 {
		assert!(outcomes > 1);
		assert!(outcomes == 2 || outcomes == outcome_tags.len() as u64);
		assert!(outcomes < 20); // up for change
		assert!(end_time > env::block_timestamp() / 1000000);
		assert!(categories.len() < 6);
		assert!(fee_percentage <= self.max_fee_percentage);
		assert!(fee_percentage >= cost_percentage);

		if outcomes == 2 {assert!(outcome_tags.len() == 0)}
		// TODO check if end_time hasn't happened yet
		let account_id = env::predecessor_account_id();

		// TODO: Escrow bond account_id creator's account
		let new_market = Market::new(self.nonce, account_id, description, extra_info, outcomes, outcome_tags, categories, end_time, fee_percentage, cost_percentage, api_source);
		let market_id = new_market.id;
		self.active_markets.insert(self.nonce, new_market);
		self.nonce = self.nonce + 1;
		return market_id;
	}

	pub fn delete_market(
		&mut self,
		market_id: u64
	) {
		let account_id = env::predecessor_account_id();
		assert_eq!(account_id, self.creator, "markets can only be deleted by the market creator");
		self.active_markets.remove(&market_id);
	}

	pub fn place_order(
        &mut self,
        market_id: u64,
        outcome: u64,
        spend: u128,
        price: u128
    ) {
		let from = env::predecessor_account_id();
        ext_fungible_token::get_balance(from.to_string(), &from.to_string(), 0, SINGLE_CALL_GAS).then(
            ext::check_sufficient_balance(spend, &env::current_account_id(), 0, SINGLE_CALL_GAS)
        ).then(
            ext::purchase_shares(from, market_id, outcome, spend, price, &env::current_account_id(), 0, SINGLE_CALL_GAS)
        );
	}

    #[callback_vec(amount)]
    pub fn check_sufficient_balance(&mut self, spend: u128, amount: u128) -> Result<bool, String> {
        if amount >= spend {
            return Ok(amount >= spend);
        } else {
            return Err("user does not have sufficient balance".to_string());
        }
    }

    #[callback]
    pub fn purchase_shares(&mut self, from: String, market_id: u64, outcome: u64, spend: u128, price: u128) {
        let amount_of_shares = spend / price;
        let rounded_spend = amount_of_shares * price;
        let market = self.active_markets.get_mut(&market_id).unwrap();
        market.create_order(from.to_string(), outcome, amount_of_shares, rounded_spend, price);
        self.escrow_funds(rounded_spend);
    }

    // TODO: Subtract liquidity
	pub fn cancel_order(
        &mut self,
        market_id: u64,
        outcome: u64,
        order_id: u128
    ) {
		let account_id = env::predecessor_account_id();
		let market = self.active_markets.get_mut(&market_id).unwrap();
		assert_eq!(market.resoluted, false);
		let mut orderbook = market.orderbooks.get_mut(&outcome).unwrap();
		let order = orderbook.open_orders.get(&order_id).unwrap();
		assert!(account_id == order.creator);
		let to_return = orderbook.remove_order(order_id);
		self.payout(to_return, account_id);
    }

	pub fn resolute_market(
		&mut self,
		market_id: u64,
		winning_outcome: Option<u64>,
		stake: u128
	) {
	    let account_id = env::predecessor_account_id();
        ext_fungible_token::get_balance(account_id.to_string(), &account_id.to_string(), 0, SINGLE_CALL_GAS).then(
            ext::check_sufficient_balance(stake, &account_id.to_string(), 0, SINGLE_CALL_GAS)
        ).then(
            ext::resolute_approved(market_id, winning_outcome, stake, &account_id.to_string(), 0, SINGLE_CALL_GAS)
        );
	}

    #[callback]
    pub fn resolute_approved(
        &mut self,
        market_id: u64,
        winning_outcome: Option<u64>,
        stake: u128
    ) {
        let market = self.active_markets.get_mut(&market_id).expect("market doesn't exist");
        assert_eq!(market.resoluted, false);
        let change = market.resolute(winning_outcome, stake);
        self.escrow_funds(stake - change);
    }



	pub fn withdraw_dispute_stake(
		&mut self,
		market_id: u64,
		dispute_round: u64,
		outcome: Option<u64>
	) {
		let market = self.active_markets.get_mut(&market_id).expect("invalid market");
		let to_return = market.cancel_dispute_participation(dispute_round, outcome);
		self.payout(to_return, env::predecessor_account_id());
	}

	pub fn dispute_market(
		&mut self,
		market_id: u64,
		winning_outcome: Option<u64>,
		stake: u128
	) {
	    let account_id = env::predecessor_account_id();
        let market = self.active_markets.get_mut(&market_id).expect("market doesn't exist");
		let balance = self.fdai_balances.get(&account_id).unwrap_or(&0);
		assert!(balance >= &stake, "not enough balance to cover stake");
		let change = market.dispute(winning_outcome, stake);
        self.escrow_funds(stake - change);
	}

	pub fn finalize_market(
		&mut self,
		market_id: u64,
		winning_outcome: Option<u64>
	) {
		let market = self.active_markets.get_mut(&market_id).unwrap();
		assert_eq!(market.resoluted, true);
		if market.disputed {
			assert_eq!(env::predecessor_account_id(), self.creator, "only the judge can resolute disputed markets");
		} else {
			// Check that the first dispute window is closed
			let dispute_window = market.resolution_windows.last().expect("no dispute window found, something went wrong");
			assert!(env::block_timestamp() / 1000000 >= dispute_window.end_time || dispute_window.round == 2, "dispute window still open")
		}

        market.finalize(winning_outcome);
	}

	fn escrow_funds(
	    &mut self,
        amount: u128
    ) {
		let from = env::predecessor_account_id();
		// TODO: BE ABLE TO PARSE WHETHER THAT TRANSFER WAS SUCCESSFUL OR NOT
		ext_fungible_token::transfer(env::current_account_id(), amount, &from, 0, SINGLE_CALL_GAS).then(
            ext::update_fdai_metrics_subtract(amount, &env::current_account_id(), 0, SINGLE_CALL_GAS)
		);
	}

	#[callback_vec(amount)]
    pub fn update_fdai_metrics_subtract(&mut self, amount: u128) {
        // For monitoring supply - just for testnet
        self.fdai_outside_escrow = self.fdai_outside_escrow - amount as u128;
        self.fdai_in_protocol= self.fdai_outside_escrow + amount as u128;
    }

	fn payout(
	    &mut self,
	    amount: u128,
	    account_id: String
    ) {
        ext_fungible_token::transfer_from(env::current_account_id(), account_id.to_string(), amount, &env::current_account_id(), 0, SINGLE_CALL_GAS).then(
            ext::update_fdai_metrics_add(amount,  &env::current_account_id(), 0, SINGLE_CALL_GAS)
        );
    }

	#[callback_vec(amount)]
    pub fn update_fdai_metrics_add(&mut self, amount: u128) {
        // TODO: Determine if above call was a success
        // For monitoring supply - just for testnet
        self.fdai_outside_escrow = self.fdai_outside_escrow + amount as u128;
        self.fdai_in_protocol= self.fdai_outside_escrow - amount as u128;
    }

	pub fn get_active_resolution_window(
		&self,
		market_id: u64
	) -> Option<&ResolutionWindow> {
		let market = self.active_markets.get(&market_id).expect("market doesn't exist");
		if !market.resoluted {
			return None;
		}
		return Some(market.resolution_windows.last().expect("invalid dispute window"));

	}

	pub fn get_open_orders(
		&self,
		market_id: u64,
		outcome: u64
	) -> &HashMap<u128, Order> {
		let market = self.active_markets.get(&market_id).unwrap();
		let orderbook = market.orderbooks.get(&outcome).unwrap();
		return &orderbook.open_orders;
	}

	pub fn get_filled_orders(
		&self,
		market_id: u64,
		outcome: u64
	) -> &HashMap<u128, Order> {
		let market = self.active_markets.get(&market_id).unwrap();
		let orderbook = market.orderbooks.get(&outcome).unwrap();
		return &orderbook.filled_orders;
	}

	pub fn get_claimable(
		&self,
		market_id: u64,
		account_id: String
	) -> u128 {
		return self.active_markets.get(&market_id).unwrap().get_claimable_for(account_id);
	}

	pub fn claim_creator_fee(
		&mut self,
		market_id: u64
	) {
		let market = self.active_markets.get_mut(&market_id).expect("market doesn't exist");
		let creator = market.creator.to_string();
		assert_eq!(market.fee_claimed, false, "creator already claimed fees");
		assert_eq!(env::predecessor_account_id(), creator.to_string(), "only creator himself can claim the fees");
		// TODO: liquidity, as it is now is not the right metric, filled volume would be
		let fee_payout = market.liquidity * market.fee_percentage / 100;
		market.fee_claimed = true;
		self.payout(fee_payout, creator.to_string());
	}

	pub fn claim_earnings(
		&mut self,
		market_id: u64,
		account_id: String
	) {
		let market = self.active_markets.get_mut(&market_id).unwrap();
		assert!(env::block_timestamp() / 1000000 >= market.end_time, "market hasn't ended yet");
		assert_eq!(market.resoluted, true);
		assert_eq!(market.finalized, true);

		let claimable = market.get_claimable_for(account_id.to_string());
		market.reset_balances_for(account_id.to_string());
		market.delete_resolution_for(account_id.to_string());

		self.payout(claimable, account_id);
	}

	pub fn get_all_markets(
		&self
	) -> &BTreeMap<u64, Market> {
		return &self.active_markets;
	}

	pub fn get_markets_by_id(
		&self,
		market_ids: Vec<u64>
	) -> BTreeMap<u64, &Market> {
		let mut markets = BTreeMap::new();
		for market_id in market_ids {
			markets.insert(market_id, self.active_markets.get(&market_id).unwrap());
		}
		return markets;
	}

	pub fn get_specific_markets(
		&self,
		market_ids: Vec<u64>
	) -> BTreeMap<u64, &Market> {
		let mut markets = BTreeMap::new();
		for market_id in 0..market_ids.len() {
			markets.insert(market_id as u64, self.active_markets.get(&(market_id as u64)).unwrap());
		}
		return markets;
	}

	pub fn get_depth(
		&self,
		market_id: u64,
		outcome: u64,
		spend: u128,
		price: u128
	) -> u128 {
		let market = self.active_markets.get(&market_id).unwrap();
		return market.get_liquidity_available(outcome, spend, price);
	}

	pub fn get_liquidity(
		&self,
		market_id: u64,
		outcome: u64,
		price: u128
	) -> u128 {
		let market = self.active_markets.get(&market_id).unwrap();
		let orderbook = market.orderbooks.get(&outcome).unwrap();

		return orderbook.get_liquidity_at_price(price);
	}

	pub fn get_market(
		&self,
		id: u64
	) -> &Market {
		let market = self.active_markets.get(&id);
		return market.unwrap();
	}

	pub fn get_owner(
		&self
	) -> String {
		return self.creator.to_string();
	}

	pub fn get_market_price(
		&self,
		market_id: u64,
		outcome: u64
	) -> u128 {
		let market = self.active_markets.get(&market_id).unwrap();
		return market.get_market_price_for(outcome);
	}

	pub fn get_best_prices(
		&self,
		market_id: u64
	) -> BTreeMap<u64, u128> {
		let market = self.active_markets.get(&market_id).unwrap();
		return market.get_market_prices_for();
	}

	pub fn get_fdai_metrics(
		&self
	) -> (u128, u128, u128, u64) {
		return (self.fdai_circulation, self.fdai_in_protocol, self.fdai_outside_escrow, self.user_count);
	}

}

impl Default for Markets {
	fn default() -> Self {
		Self {
			creator: "flux-dev".to_string(),
			active_markets: BTreeMap::new(),
			nonce: 0,
			fdai_balances: HashMap::new(),
			fdai_circulation: 0,
			fdai_in_protocol: 0,
			fdai_outside_escrow: 0,
			user_count: 0,
			max_fee_percentage: 5,
			creation_bond: 0,
		}
	}
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{VMContext, testing_env};
    use near_crypto::{InMemorySigner, KeyType, Signer};
    use near_runtime_standalone::{init_runtime_and_signer, RuntimeStandalone};
    use near_primitives::{
        account::{AccessKey},
        errors::{RuntimeError, TxExecutionError},
        hash::CryptoHash,
        transaction::{ExecutionOutcome, ExecutionStatus, Transaction},
        types::{AccountId, Balance},
    };
    use std::collections::{HashMap};

    use serde_json::json;
    type TxResult = Result<ExecutionOutcome, ExecutionOutcome>;

    lazy_static::lazy_static! {
        static ref MARKETS_BYTES: &'static [u8] = include_bytes!("../res/flux_protocol.wasm").as_ref();
        static ref FUNGIBLE_TOKEN_BYTES: &'static [u8] = include_bytes!("../res/fungible_token.wasm").as_ref();
    }


    pub fn ntoy(near_amount: Balance) -> Balance {
        near_amount * 10u128.pow(24)
    }

    fn outcome_into_result(outcome: ExecutionOutcome) -> TxResult {
        match outcome.status {
            ExecutionStatus::SuccessValue(_) => Ok(outcome),
            ExecutionStatus::Failure(_) => Err(outcome),
            ExecutionStatus::SuccessReceiptId(_) => panic!("Unresolved ExecutionOutcome run runtime.resolve(tx) to resolve the final outcome of tx"),
            ExecutionStatus::Unknown => unreachable!()
        }
    }

    pub struct ExternalUser {
        account_id: AccountId,
        signer: InMemorySigner,
    }

    impl ExternalUser {
        pub fn new(account_id: AccountId, signer: InMemorySigner) -> Self {
            Self { account_id, signer }
        }

        pub fn markets_init_new(&self, runtime: &mut RuntimeStandalone) -> TxResult {
                let args = json!({}).to_string().as_bytes().to_vec();

                // TODO: REPLACE POOL_ACCOUNT_ID with the correct destination address for contract
                let tx = self
                    .new_tx(runtime, "flux-tests".to_string())
                    .create_account()
                    .deploy_contract(MARKETS_BYTES.to_vec())
                    .function_call("default".into(), args, 10000000000000000, 0)
                    .sign(&self.signer);
                let res = runtime.resolve_tx(tx).unwrap();
                runtime.process_all().unwrap();
                outcome_into_result(res)
            }

        fn new_tx(&self, runtime: &RuntimeStandalone, receiver_id: AccountId) -> Transaction {
            let nonce = runtime
                .view_access_key(&self.account_id, &self.signer.public_key())
                .unwrap()
                .nonce
                + 1;
            Transaction::new(
                self.account_id.clone(),
                self.signer.public_key(),
                receiver_id,
                nonce,
                CryptoHash::default(),
            )
        }

        pub fn claim_fdai(&self, runtime: &mut RuntimeStandalone) -> TxResult {
            let args = json!({})
                .to_string()
                .as_bytes()
                .to_vec();

            let tx = self
                .new_tx(runtime, "flux-tests".to_string())
                .function_call("claim_fdai".into(), args, 10000000000000000, 0)
                .sign(&self.signer);
            let res = runtime.resolve_tx(tx).unwrap();
            runtime.process_all().unwrap();
            outcome_into_result(res)
        }

        pub fn create_market(
            &self,
            runtime: &mut RuntimeStandalone,
            description: String,
            extra_info: String,
            outcomes: u64,
            outcome_tags: Vec<String>,
            categories: Vec<String>,
            end_time: u64,
            fee_percentage: u128,
            cost_percentage: u128,
            api_source: String,
        ) -> TxResult {
            let args = json!({
                "description": description,
                "extra_info": extra_info,
                "outcomes": outcomes,
                "outcome_tags": outcome_tags,
                "categories": categories,
                "end_time": end_time,
                "fee_percentage": fee_percentage,
                "cost_percentage": cost_percentage,
                "api_source": api_source,
            })
                .to_string()
                .as_bytes()
                .to_vec();
            // TODO: DECIDE WHERE TO SEND TRANSACTION
            let tx = self
                .new_tx(runtime, "flux-tests".to_string())
                .function_call("create_market".into(), args, 10000000000000000, 0)
                .sign(&self.signer);
            let res = runtime.resolve_tx(tx).unwrap();
            runtime.process_all().unwrap();
            outcome_into_result(res)
        }

        pub fn place_order(
            &self,
            runtime: &mut RuntimeStandalone,
            market_id: u64,
            outcome: u64,
            spend: u128,
            price: u128,
        ) -> TxResult {
            let args = json!({
                "market_id": market_id,
                "outcome": outcome,
                "spend": spend,
                "price": price,
            })
                .to_string()
                .as_bytes()
                .to_vec();
            // TODO: UPDATE WHERE TO SEND TX TO
            let tx = self
                .new_tx(runtime, "flux-tests".to_string())
                .function_call("place_order".into(), args, 10000000000000000, 0)
                .sign(&self.signer);
            let res = runtime.resolve_tx(tx).unwrap();
            runtime.process_all().unwrap();
            outcome_into_result(res)
        }

        pub fn get_open_orders(&self, runtime: &RuntimeStandalone, market_id: u64, outcome: u64) -> HashMap<u128, Order> {
            // TODO: SPECIFY WHAT ACCOUNT TO CALL TO
            let open_orders = runtime
                .view_method_call(
                    &("flux-tests".to_string()),
                    "get_open_orders",
                    json!({"market_id": market_id, "outcome": outcome})
                        .to_string()
                        .as_bytes(),
                )
                .unwrap()
                .0;
            //TODO: UPDATE THIS CASTING
            //&HashMap<u128, Order>::from(serde_json::from_slice::HashMap<u128, Order>(open_orders.as_slice()).unwrap())
            let data: HashMap<&str, serde_json::Value> = serde_json::from_slice(open_orders.as_slice()).unwrap();
            let open_orders_map: HashMap<u128, Order> = serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();
            return open_orders_map;
        }

        pub fn get_filled_orders(&self, runtime: &RuntimeStandalone, market_id: u64, outcome: u64) -> HashMap<u128, Order> {
            let filled_orders = runtime
                .view_method_call(
                    &("flux-tests".to_string()),
                    "get_filled_orders",
                    json!({"market_id": market_id, "outcome": outcome})
                        .to_string()
                        .as_bytes(),
                )
                .unwrap()
                .0;
            //TODO: UPDATE THIS CASTING
            let data: HashMap<&str, serde_json::Value> = serde_json::from_slice(filled_orders.as_slice()).unwrap();
            // do custom stuff
            let filled_orders_map: HashMap<u128, Order> = serde_json::from_value(serde_json::to_value(data).unwrap()).unwrap();
            return filled_orders_map;
            //&HashMap<&u128, Order>::from(serde_json::from_slice::HashMap<u128, Order>(filled_orders.as_slice()).unwrap())
        }

        pub fn create_external(
            &self,
            runtime: &mut RuntimeStandalone,
            new_account_id: AccountId,
            amount: Balance,
        ) -> Result<ExternalUser, ExecutionOutcome> {
            let new_signer =
                InMemorySigner::from_seed(&new_account_id, KeyType::ED25519, &new_account_id);
            let tx = self
                .new_tx(runtime, new_account_id.clone())
                .create_account()
                .add_key(new_signer.public_key(), AccessKey::full_access())
                .transfer(amount)
                .sign(&self.signer);
            let res = runtime.resolve_tx(tx);

            // TODO: this temporary hack, must be rewritten
            if let Err(err) = res.clone() {
                if let RuntimeError::InvalidTxError(tx_err) = err {
                    let mut out = ExecutionOutcome::default();
                    out.status = ExecutionStatus::Failure(TxExecutionError::InvalidTxError(tx_err));
                    return Err(out);
                } else {
                    unreachable!();
                }
            } else {
                outcome_into_result(res.unwrap())?;
                runtime.process_all().unwrap();
                Ok(ExternalUser {
                    account_id: new_account_id,
                    signer: new_signer,
                })
            }
        }

    }

    pub fn init_markets_contract() -> (RuntimeStandalone, ExternalUser) {
        let (mut runtime, signer) = init_runtime_and_signer(&"root".into());
        let root = ExternalUser::new("root".into(), signer);

        root.markets_init_new(&mut runtime).unwrap();
        return (runtime, root);
    }

	fn to_dai(amt: u128) -> u128 {
		let base = 10 as u128;
		return amt * base.pow(17);
	}

	fn judge() -> String {
		return "flux-dev".to_string();
	}

	fn alice() -> String {
		return "alice.near".to_string();
	}

	fn carol() -> String {
		return "carol.near".to_string();
	}

	fn bob() -> String {
		return "bob.near".to_string();
	}

	fn empty_string() -> String {
		return "".to_string();
	}

	fn categories () -> Vec<String> {
		return vec![];
	}

	fn outcome_tags(
		number_of_outcomes: u64
	) -> Vec<String> {
		let mut outcomes: Vec<String> = vec![];
		for _ in 0..number_of_outcomes {
			outcomes.push(empty_string());
		}
		return outcomes;
	}

	fn current_block_timestamp() -> u64 {
		return 123789;
	}

	fn market_creation_timestamp() -> u64 {
		return 12378;
	}
	fn market_end_timestamp_ns() -> u64 {
		return 12379000000;
	}
	fn market_end_timestamp_ms() -> u64 {
		return 12379;
	}

	fn get_context(
		predecessor_account_id: String,
		block_timestamp: u64
	) -> VMContext {

		VMContext {
			current_account_id: alice(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
			block_index: 0,
			epoch_height: 0,
            account_balance: 0,
			is_view: false,
            storage_usage: 0,
			block_timestamp: block_timestamp,
			account_locked_balance: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(11),
            random_seed: vec![0, 1, 2],
            output_data_receivers: vec![],
		}
	}






	mod init_tests;
	//mod market_order_tests;
	//mod binary_order_matching_tests;
	mod categorical_market_tests;
	//mod market_resolution_tests;
	//mod claim_earnings_tests;
	//mod market_depth_tests;
}
