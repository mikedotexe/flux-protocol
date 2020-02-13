use std::string::String;
use near_bindgen::{near_bindgen, env};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

mod market;
type Market = market::Market;
type Order = market::orderbook::order::Order;

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
struct Markets {
	creator: String,
	active_markets: BTreeMap<u64, Market>,
	nonce: u64,
	fdai_balances: HashMap<String, u64>, // Denominated in 1e18
	fdai_circulation: u128,
	fdai_in_protocol: u128,
	fdai_outside_escrow: u128,
	user_count: u64
}

#[near_bindgen]
impl Markets {

	fn dai_token(&self) -> u64 {
		let base: u64 = 10;
		return base.pow(17);
	}

	// This is a demo method, it mints a currency to interact with markets until we have NDAI
	pub fn add_to_creators_funds(&mut self, amount: u64) {
		let from = env::predecessor_account_id();
		assert_eq!(from, self.creator);

		*self.fdai_balances.get_mut(&from).unwrap() += amount;

		// Monitoring total supply - just for testnet
		self.fdai_circulation = self.fdai_circulation + amount as u128;
		self.fdai_outside_escrow = self.fdai_outside_escrow + amount as u128;
	}

	// This is a demo method, it mints a currency to interact with markets until we have NDAI
	pub fn claim_fdai(&mut self) -> bool{
		let from = env::predecessor_account_id();
		let can_claim = self.fdai_balances.get(&from).is_none();
		assert!(can_claim);
		
		let claim_amount = 100 * self.dai_token();
		self.fdai_balances.insert(from, claim_amount);

		// Monitoring total supply - just for testnet
		self.fdai_circulation = self.fdai_circulation + claim_amount as u128;
		self.fdai_outside_escrow = self.fdai_outside_escrow + claim_amount as u128;
		self.user_count = self.user_count + 1;
		return true;
	}

	pub fn get_fdai_balance(&self, from: String) -> &u64 {
		return self.fdai_balances.get(&from).unwrap();
	}
	
	pub fn create_market(&mut self, outcomes: u64, description: String, end_time: u64) -> bool {
		// TODO: Do some market validation
		let from = env::predecessor_account_id();
		// if from == self.creator {
			let new_market = Market::new(self.nonce, from, outcomes, description.to_string(), end_time);
			self.active_markets.insert(self.nonce, new_market);
			self.nonce = self.nonce + 1;
			return true;
		// } else {
		// 	return false;
		// }
	}

	pub fn delete_market(&mut self, market_id: u64) -> bool {
		let from = env::predecessor_account_id();

		if  from == self.creator {
			self.active_markets.remove(&market_id);
			return true;
		} else {
			return false;
		}
	}

	pub fn place_order(&mut self, market_id: u64, outcome: u64, spend: u64, price_per_share: u64) {
		let from = env::predecessor_account_id();
		let balance = self.fdai_balances.get(&from).unwrap();
		assert!(balance >= &spend);
		
		let amount_of_shares = spend / price_per_share;
		let rounded_spend = amount_of_shares * price_per_share;
		let market = self.active_markets.get_mut(&market_id).unwrap();
		market.place_order(from.to_string(), outcome, amount_of_shares, rounded_spend, price_per_share);

		self.subtract_balance(rounded_spend);
	}

	// pub fn cancel_order(&mut self, market_id: u64, outcome: u64, order_id: u64) {
	// 	let from = env::predecessor_account_id();
	// 	let orderbook = self.active_markets.get_mut(&market_id).unwrap().orderbooks.get_mut(&outcome).unwrap();
	// 	let order = orderbook.open_orders.get(&order_id).unwrap();
	// 	assert_eq!(order.creator, from);
	// 	orderbook.remove_order(order_id);
	// }

	pub fn resolute(&mut self, market_id: u64, payout: Vec<u64>, invalid: bool) -> bool {
		let from = env::predecessor_account_id();
		let mut resoluted = false;
		self.active_markets.entry(market_id).and_modify(|market| {
			assert_eq!(market.creator, from);
			market.resolute(payout, invalid);
		});
		return resoluted;
	}

	// pub fn claim_earnings(&mut self, market_id: u64) {
	// 	let from = env::predecessor_account_id();
	// 	let mut earnings = 0;
	// 	self.active_markets.entry(market_id).and_modify(|market| {
	// 		earnings = market.claim_earnings(from.to_string());
	// 	});

	// 	assert!(earnings > 0);
	// 	self.add_balance(earnings)
	// }

	fn subtract_balance(&mut self, amount: u64) {
		let from = env::predecessor_account_id();
		let balance = self.fdai_balances.get(&from).unwrap();
		let new_balance = *balance - amount;
		self.fdai_balances.insert(from, new_balance);

		// For monitoring supply - just for testnet
		self.fdai_outside_escrow = self.fdai_outside_escrow - amount as u128;
		self.fdai_in_protocol= self.fdai_outside_escrow + amount as u128;
	}

	fn add_balance(&mut self, amount: u64) {
		let from = env::predecessor_account_id();
		let balance = self.fdai_balances.get(&from).unwrap();
		let new_balance = *balance + amount;
		self.fdai_balances.insert(from, new_balance);

		// For monitoring supply - just for testnet
		self.fdai_outside_escrow = self.fdai_outside_escrow + amount as u128;
		self.fdai_in_protocol= self.fdai_outside_escrow - amount as u128;
	}

	// pub fn get_open_orders(&self, market_id: u64, outcome: u64, from: String) -> &BTreeMap<u64, Order> {
	// 	let market = self.active_markets.get(&market_id).unwrap();
	// 	let orderbook = market.orderbooks.get(&outcome).unwrap();
	// 	return &orderbook.open_orders;
	// }
	
	// pub fn get_filled_orders(&self, market_id: u64, outcome: u64, from: String) -> &BTreeMap<u64, Order> {
	// 	let market = self.active_markets.get(&market_id).unwrap();
	// 	let orderbook = market.orderbooks.get(&outcome).unwrap();
	// 	return &orderbook.filled_orders;
	// }

	// pub fn get_earnings(&self, market_id: u64, from: String) -> u64 {
	// 	return self.active_markets.get(&market_id).unwrap().get_earnings(from, false);	
	// }

	pub fn get_owner(&self) -> &String {
		return &self.creator;
	}

	pub fn get_all_markets(&self) -> &BTreeMap<u64, Market> { 
		return &self.active_markets;
	}

	pub fn get_market(&self, id: u64) -> &Market {
		let market = self.active_markets.get(&id);
		return market.unwrap();
	}

	pub fn get_market_order(&self, market_id: u64, outcome: u64)  -> Option<u64> {
		let market = self.active_markets.get(&market_id);
		return market.unwrap().orderbooks[&outcome].market_order;
	}

	pub fn get_fdai_metrics(&self) -> (u128, u128, u128, u64) {
		return (self.fdai_circulation, self.fdai_in_protocol, self.fdai_outside_escrow, self.user_count);
	}

}

impl Default for Markets {
	fn default() -> Self {
		Self {
			creator: "klopt".to_string(),
			active_markets: BTreeMap::new(),
			nonce: 0,
			fdai_balances: HashMap::new(),	
			fdai_circulation: 0,
			fdai_in_protocol: 0,
			fdai_outside_escrow: 0,
			user_count: 0
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

	fn get_context(predecessor_account_id: String) -> VMContext {
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
			block_timestamp: 123789,
			account_locked_balance: 0,
            attached_deposit: 500000000,
            prepaid_gas: 10u64.pow(9),
            random_seed: vec![0, 1, 2],
            output_data_receivers: vec![],
		}
	}

	// mod init_tests;
	// mod bst_tests;
	// mod market_order_tests;
	// mod order_matching_tests;
	mod categorical_market_tests;

	// #[test]
	// fn test_categorical_market_orders() {
	// 	testing_env!(get_context(carol()));
		
	// 	let mut contract = Markets::default();
	// 	contract.claim_fdai();
	// 	contract.create_market(3, "Hi!".to_string(), 100010101001010);
	// 	contract.place_order(0, 0, 100000, 500);
	// 	contract.place_order(0, 0, 100000, 500);
	// 	contract.place_order(0, 1, 100000, 500);

	// 	let open_orders = contract.get_market(0).orderbooks.get(&0);

	// 	println!("{:?}", open_orders);
		
	// }	


}
