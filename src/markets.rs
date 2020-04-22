use near_bindgen::{near_bindgen, env};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};

mod market;
type Market = market::Market;
type Order = market::orderbook::order::Order;

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

#[near_bindgen]
impl Markets {

	fn dai_token(&self) -> u128 {
		let base: u128 = 10;
		return base.pow(17);
	}

	// This is a demo method, it mints a currency to interact with markets until we have NDAI
	pub fn add_to_creators_funds(&mut self, amount: u128) {
		let from = env::predecessor_account_id();
		assert_eq!(from, self.creator);

		*self.fdai_balances.get_mut(&from).unwrap() += amount;

		// Monitoring total supply - just for testnet
		self.fdai_circulation = self.fdai_circulation + amount as u128;
		self.fdai_outside_escrow = self.fdai_outside_escrow + amount as u128;
	}

	// This is a demo method, it mints a currency to interact with markets until we have NDAI
	pub fn claim_fdai(&mut self) {
		let from = env::predecessor_account_id();
		let can_claim = self.fdai_balances.get(&from).is_none();
		assert!(can_claim, "user has already claimed fdai");

		let claim_amount = 100 * self.dai_token();
		self.fdai_balances.insert(from, claim_amount);

		// Monitoring total supply - just for testnet
		self.fdai_circulation = self.fdai_circulation + claim_amount as u128;
		self.fdai_outside_escrow = self.fdai_outside_escrow + claim_amount as u128;
		self.user_count = self.user_count + 1;
	}

	pub fn get_fdai_balance(&self, from: String) -> u128 {
		return *self.fdai_balances.get(&from).unwrap();
	}

	pub fn create_market(&mut self, description: String, extra_info: String, outcomes: u64, outcome_tags: Vec<String>, categories: Vec<String>, end_time: u64, fee_percentage: u128, cost_percentage: u128, api_source: String) -> u64 {
		assert!(outcomes > 1);
		assert!(outcomes == 2 || outcomes == outcome_tags.len() as u64);
		assert!(outcomes < 20); // up for change
		assert!(end_time > env::block_timestamp() / 1000000);
		assert!(categories.len() < 6);
		assert!(fee_percentage <= self.max_fee_percentage);
		assert!(fee_percentage >= cost_percentage);

		if outcomes == 2 {assert!(outcome_tags.len() == 0)}
		// TODO check if end_time hasn't happened yet
		let from = env::predecessor_account_id();
		//println!("Contract being created from: {}", from.to_string());
		// TODO: Escrow bond from creator's account
		let new_market = Market::new(self.nonce, from, description, extra_info, outcomes, outcome_tags, categories, end_time, fee_percentage, cost_percentage, api_source);
		let market_id = new_market.id;
		self.active_markets.insert(self.nonce, new_market);
		self.nonce = self.nonce + 1;
		return market_id;
	}

	pub fn delete_market(&mut self, market_id: u64) {
		let from = env::predecessor_account_id();
		assert_eq!(from, self.creator, "markets can only be deleted by the market creator");
		self.active_markets.remove(&market_id);
	}

	pub fn place_order(&mut self, market_id: u64, outcome: u64, spend: u128, price: u128) {
		let from = env::predecessor_account_id();
		let balance = self.fdai_balances.get(&from).unwrap();
		assert!(balance >= &spend);

		let amount_of_shares = spend / price;
		let rounded_spend = amount_of_shares * price;
		let market = self.active_markets.get_mut(&market_id).unwrap();
		market.place_order(from.to_string(), outcome, amount_of_shares, rounded_spend, price);

		self.subtract_balance(rounded_spend);
	}

	pub fn cancel_order(&mut self, market_id: u64, outcome: u64, order_id: u128) {
		let from = env::predecessor_account_id();
		let market = self.active_markets.get_mut(&market_id).unwrap();
		assert_eq!(market.resoluted, false);
		let mut orderbook = market.orderbooks.get_mut(&outcome).unwrap();
		let order = orderbook.open_orders.get(&order_id).unwrap();
		assert!(from == order.creator);
		orderbook.remove_order(order_id);
    }

	pub fn resolute(&mut self, market_id: u64, winning_outcome: Option<u64>) {
		let from = env::predecessor_account_id();
		let market = self.active_markets.get_mut(&market_id).unwrap();
		assert_eq!(market.resoluted, false);

		let bond = market.resolute_bond;
		let balance = self.fdai_balances.get(&from).unwrap();
        assert!(balance >= &bond);

		market.resolute(from, winning_outcome, bond);
		self.subtract_balance(bond);
	}

	pub fn dispute(&mut self, market_id: u64, winning_outcome: Option<u64>, bond: u128) {
	    let from = env::predecessor_account_id();
        let market = self.active_markets.get_mut(&market_id).unwrap();
        assert_eq!(market.resoluted, true);
        assert_eq!(market.disputed, false);

        let return_amount = market.dispute(from, winning_outcome, bond);
        self.subtract_balance(bond-return_amount);
	}

	pub fn finalize_market(&mut self, market_id: u64, winning_outcome: Option<u64>) {
	    let market = self.active_markets.get_mut(&market_id).unwrap();
	    assert_eq!(market.resoluted, true);
	    //assert_eq!(env::predecessor_account_id(), )
        market.finalize(env::predecessor_account_id(), winning_outcome);
	}

	fn subtract_balance(&mut self, amount: u128) {
		let from = env::predecessor_account_id();
		let balance = self.fdai_balances.get(&from).unwrap();
		let new_balance = *balance - amount;
		self.fdai_balances.insert(from, new_balance);

		// For monitoring supply - just for testnet
		self.fdai_outside_escrow = self.fdai_outside_escrow - amount as u128;
		self.fdai_in_protocol= self.fdai_outside_escrow + amount as u128;
	}

	fn add_balance(&mut self, amount: u128) {
		let from = env::predecessor_account_id();
		let balance = self.fdai_balances.get(&from).unwrap();
		let new_balance = *balance + amount;
		self.fdai_balances.insert(from, new_balance);

		// For monitoring supply - just for testnet
		self.fdai_outside_escrow = self.fdai_outside_escrow + amount as u128;
		self.fdai_in_protocol= self.fdai_outside_escrow - amount as u128;
	}

	pub fn get_open_orders(&self, market_id: u64, outcome: u64) -> &HashMap<u128, Order> {
		let market = self.active_markets.get(&market_id).unwrap();
		let orderbook = market.orderbooks.get(&outcome).unwrap();
		return &orderbook.open_orders;
	}

	pub fn get_filled_orders(&self, market_id: u64, outcome: u64) -> &HashMap<u128, Order> {
		let market = self.active_markets.get(&market_id).unwrap();
		let orderbook = market.orderbooks.get(&outcome).unwrap();
		return &orderbook.filled_orders;
	}

	pub fn get_claimable(&self, market_id: u64, from: String) -> u128 {
		return self.active_markets.get(&market_id).unwrap().get_claimable(from);
	}

	pub fn claim_earnings(&mut self, market_id: u64, account_id: String) {
		let market = self.active_markets.get_mut(&market_id).unwrap();
		assert!(env::block_timestamp() >= market.end_time, "market hasn't ended yet");
		assert_eq!(market.resoluted, true);
		assert_eq!(market.finalized, true);

		let claimable = market.get_claimable(account_id.to_string());
		market.delete_orders_for(account_id.to_string());
		market.delete_resolution_for(account_id.to_string());

		self.add_balance(claimable);
	}

	pub fn get_all_markets(&self) -> &BTreeMap<u64, Market> {
		return &self.active_markets;
	}

	pub fn get_markets_by_id(&self, market_ids: Vec<u64>) -> BTreeMap<u64, &Market> {
		let mut markets = BTreeMap::new();
		for market_id in market_ids {
			markets.insert(market_id, self.active_markets.get(&market_id).unwrap());
		}
		return markets;
	}

	pub fn get_specific_markets(&self, market_ids: Vec<u64>) -> BTreeMap<u64, &Market> {
		let mut markets = BTreeMap::new();
		for market_id in 0..market_ids.len() {
			markets.insert(market_id as u64, self.active_markets.get(&(market_id as u64)).unwrap());
		}
		return markets;
	}

	pub fn get_liquidity(&self, market_id: u64, outcome: u64, spend: u128, price: u128) -> u128 {
		let mut market = self.active_markets.get(&market_id).unwrap();
		return market.get_liquidity(outcome, spend, price);
	}

	pub fn get_market(&self, id: u64) -> &Market {
		let market = self.active_markets.get(&id);
		return market.unwrap();
	}

	pub fn get_owner(&self) -> String {
		return self.creator.to_string();
	}

	pub fn get_market_price(&self, market_id: u64, outcome: u64) -> u128 {
		let market = self.active_markets.get(&market_id).unwrap();
		return market.get_market_price(outcome);
	}

	pub fn get_best_prices(&self, market_id: u64) -> BTreeMap<u64, u128> {
		let market = self.active_markets.get(&market_id).unwrap();
		return market.get_market_prices();
	}

	pub fn get_fdai_metrics(&self) -> (u128, u128, u128, u64) {
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
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{VMContext, VMConfig, testing_env};

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

	fn outcome_tags(number_of_outcomes: u64) -> Vec<String> {
		let mut outcomes: Vec<String> = vec![];
		for _ in 0..number_of_outcomes {
			outcomes.push(empty_string());
		}
		return outcomes;
	}

	fn current_block_timestamp() -> u64 {
		return 123789;
	}
	fn market_end_timestamp() -> u64 {
		return current_block_timestamp() + 1;
	}

	fn get_context(predecessor_account_id: String, block_timestamp: u64) -> VMContext {
		VMContext {
			current_account_id: alice(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
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
	mod market_order_tests;
	mod binary_order_matching_tests;
	mod categorical_market_tests;
	mod market_resolution_tests;
	mod claim_earnings_tests;
	mod market_depth_tests;
	mod market_dispute_tests;
}
