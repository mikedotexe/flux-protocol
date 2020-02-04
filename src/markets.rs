use std::string::String;
use near_bindgen::{near_bindgen, env};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

mod binary_market;
type BinaryMarket = binary_market::BinaryMarket;
type Order = binary_market::orderbook::order::Order;

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
struct Markets {
	creator: String,
	active_markets: BTreeMap<u64, BinaryMarket>,
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
			let new_market = BinaryMarket::new(self.nonce, from, outcomes, description.to_string(), end_time);
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

	pub fn place_order(&mut self, market_id: u64, outcome: u64, spend: u64, price_per_share: u64) -> bool {
		let from = env::predecessor_account_id();
		let balance = self.fdai_balances.get(&from).unwrap();
		assert!(balance >= &spend);
		
		let amount_of_shares = spend / price_per_share;
		let actualized_spend = amount_of_shares * price_per_share;

		self.active_markets.entry(market_id).and_modify(|market| {
			market.place_order(from.to_string(), outcome, amount_of_shares, actualized_spend, price_per_share);
		});

		self.subtract_balance(actualized_spend);
		return true;
	}

	pub fn resolute(&mut self, market_id: u64, payout: Vec<u64>, invalid: bool) -> bool {
		let from = env::predecessor_account_id();
		let mut resoluted = false;
		self.active_markets.entry(market_id).and_modify(|market| {
			assert_eq!(market.creator, from);
			market.resolute(payout, invalid);
		});
		return resoluted;
	}

	pub fn claim_earnings(&mut self, market_id: u64) {
		let from = env::predecessor_account_id();
		let mut earnings = 0;
		self.active_markets.entry(market_id).and_modify(|market| {
			earnings = market.claim_earnings(from.to_string());
		});

		assert!(earnings > 0);
		self.add_balance(earnings)
	}

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

	pub fn get_open_orders(&self, market_id: u64, outcome: u64, from: String) -> Vec<Order> {
		let market = self.active_markets.get(&market_id).unwrap();
		return market.get_open_orders_for_user(from, outcome);
	}
	
	pub fn get_filled_orders(&self, market_id: u64, outcome: u64, from: String) -> Vec<Order> {
		let market = self.active_markets.get(&market_id).unwrap();
		return market.get_filled_orders_for_user(from, outcome);
	}

	pub fn get_earnings(&self, market_id: u64, from: String) -> u64 {
		return self.active_markets.get(&market_id).unwrap().get_earnings(from, false);	
	}

	pub fn get_owner(&self) -> &String {
		return &self.creator;
	}

	pub fn get_all_markets(&self) -> &BTreeMap<u64, BinaryMarket> { 
		return &self.active_markets;
	}

	pub fn get_market(&self, id: u64) -> &BinaryMarket {
		let market = self.active_markets.get(&id);
		return market.unwrap();
	}

	pub fn get_market_order(&self, market_id: u64, outcome: u64)  -> Option<&Order> {
		let market = self.active_markets.get(&market_id);
		return market.unwrap().orderbooks[&outcome].get_market_order();
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

    #[test]
	fn test_contract_creation() {
		testing_env!(get_context(carol()));
		let mut contract = Markets::default(); 
	}

    #[test]
	fn test_market_creation() {
		testing_env!(get_context(carol()));
		let mut contract = Markets::default(); 
		contract.create_market(2, "Hi!".to_string(), 100010101001010);
	}


	#[test]
	fn test_market_orders() {
		testing_env!(get_context(carol()));
		
		let mut contract = Markets::default();
		contract.claim_fdai();
		contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
		// Placing "no" order
		contract.place_order(0, 0, 10000, 50);									
		let market_no_order = contract.get_market_order(0, 0);
		assert_eq!(market_no_order.is_none(), false);
		
		contract.place_order(0, 1, 9000, 50);
		contract.place_order(0, 1, 1000, 50);

		let market_no_order = contract.get_market_order(0, 0);
		let market_yes_order = contract.get_market_order(0, 1);
		assert_eq!(market_no_order.is_none(), true);
		assert_eq!(market_yes_order.is_none(), true);
	}	

	#[test]
	fn test_fdai_balances() {
		testing_env!(get_context(carol()));
		
		let mut contract = Markets::default();
		contract.claim_fdai();
		let mut balance = contract.get_fdai_balance(carol());
		let base: u64 = 10;
		let mut expected_balance = 100 * base.pow(17);
		let initial_balance = expected_balance;

		assert_eq!(balance, &expected_balance);

		contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
		contract.place_order(0, 0, 40000, 40);
		balance = contract.get_fdai_balance(carol());
		expected_balance  = expected_balance - 40000;
		assert_eq!(balance, &expected_balance);
		

		testing_env!(get_context(bob()));
		contract.claim_fdai();

		contract.place_order(0, 1, 60000, 60);
		balance = contract.get_fdai_balance(bob());
		expected_balance = initial_balance - 60000;
		assert_eq!(balance, &expected_balance);

		testing_env!(get_context(carol()));
		contract.resolute(0, vec![10000, 0], false);
		contract.claim_earnings(0);
		
		balance = contract.get_fdai_balance(carol());
		expected_balance = initial_balance + 60000;
		assert_eq!(balance, &expected_balance);
		
		testing_env!(get_context(bob()));
		balance = contract.get_fdai_balance(bob());
		expected_balance = initial_balance - 60000;
		assert_eq!(balance, &expected_balance);
	}

	#[test]
	fn test_payout_open_orders_on_loss() {
		testing_env!(get_context(carol()));
		
		let mut contract = Markets::default();
		contract.claim_fdai();
		let mut balance = contract.get_fdai_balance(carol());
		let base: u64 = 10;
		let mut expected_balance = 100 * base.pow(17);
		let initial_balance = expected_balance;

		assert_eq!(balance, &expected_balance);

		contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
		contract.place_order(0, 0, 10000, 10);
		contract.place_order(0, 0, 20000, 50);
		
		testing_env!(get_context(bob()));
		contract.claim_fdai();
		
		contract.place_order(0, 1, 21000, 50);
		contract.place_order(0, 1, 10000, 90);
		
		testing_env!(get_context(carol()));
		contract.resolute(0, vec![10000, 0], false); // carol wins
		// contract.claim_earnings(0);
		
		let claimable_carol = contract.get_earnings(0, carol());
		let claimable_bob = contract.get_earnings(0, bob());
		let expected_carol = 20000 + 40000;
		let expected_bob = 1000;
		let carol_delta = expected_carol - claimable_carol;
		let bob_delta = expected_bob - claimable_bob;
		assert!(carol_delta <= 100);
		assert!(bob_delta <= 100);

	}

	#[test]
	fn test_invalid_market() {
		testing_env!(get_context(carol()));
		
		let mut contract = Markets::default();
		contract.claim_fdai();
		contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
		contract.place_order(0, 0, 7321893, 70);

		testing_env!(get_context(bob()));
		contract.claim_fdai();
		contract.place_order(0, 1, 1232173, 30);

		testing_env!(get_context(carol()));
		contract.resolute(0, vec![5000, 5000], true);
		let carol_earnings = contract.get_earnings(0, carol());
		let bob_earnings = contract.get_earnings(0, bob());

		println!("carol earnings: {} bob earnigns: {}", carol_earnings, bob_earnings);
		// assert_eq!(bob_earnings, 50000);
		let carol_old_balance = contract.get_fdai_balance(carol());
		contract.claim_earnings(0);
		println!(" ");
		let carol_new_balance = contract.get_fdai_balance(carol());
		println!("Carol's new balance {}" , carol_new_balance);
	}
	
	#[test]
	fn test_get_open_orders() {
		testing_env!(get_context(carol()));
		
		let mut contract = Markets::default(); 
		contract.claim_fdai();
		contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 0, 40000, 40);

		let open_orders = contract.get_open_orders(0, 0, carol());
		assert_eq!(open_orders.len(), 5);
	}

	#[test]
	fn test_get_filled_orders() {
		testing_env!(get_context(carol()));
		
		let mut contract = Markets::default();
		contract.claim_fdai();
		contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 1, 60000, 60);

		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 0, 40000, 40);
		contract.place_order(0, 0, 40000, 40);

		let open_orders = contract.get_open_orders(0, 0, carol());
		let filled_orders = contract.get_filled_orders(0, 0, carol());
		assert_eq!(open_orders.len(), 4);
		assert_eq!(filled_orders.len(), 1);
	}

	#[test]
	fn test_decimal_division_results() {
		testing_env!(get_context(carol()));
		
		let mut contract = Markets::default();
		contract.claim_fdai();
		contract.create_market(2, "Hi!".to_string(), 100010101001010);
		
		contract.place_order(0, 0, 1782361, 77);									

		testing_env!(get_context(bob()));
		contract.claim_fdai();

		contract.place_order(0, 1, 123123123, 23);

		testing_env!(get_context(carol()));
		contract.resolute(0, vec![0, 10000], false);
		
		testing_env!(get_context(bob()));
		contract.claim_earnings(0);
		let bob_balance = contract.get_fdai_balance(bob());

		testing_env!(get_context(carol()));
		let carol_balance = contract.get_fdai_balance(carol());
	}	

}
