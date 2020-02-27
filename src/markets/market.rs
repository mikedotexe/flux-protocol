use std::string::String;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use near_bindgen::{near_bindgen, env};
use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};

pub mod orderbook;
type Orderbook = orderbook::Orderbook;
type Order = orderbook::Order;

// TODO: Endtime has to be in blocks instead of ms / s
// TODO: Add additional_info
// TODO: Add resolution_url
// TODO: Add outcome_tags if outcomes > 2
// TODO: Add check: outcomes have to be < 20
#[near_bindgen]
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct Market {
	pub id: u64,
	pub description: String,
	pub extra_info: String,
	pub creator: String,
	pub outcomes: u64,
	pub outcome_tags: Vec<String>,
	pub end_time: u64,
	pub orderbooks: BTreeMap<u64, orderbook::Orderbook>,
	pub winning_outcome: Option<u64>,
	pub resoluted: bool,
	pub liquidity: u64
}

impl Market {
	pub fn new(id: u64, from: String, description: String, extra_info: String, outcomes: u64, outcome_tags: Vec<String>, end_time: u64) -> Self {
		let mut empty_orderbooks = BTreeMap::new();

		for i in 0..outcomes {
			empty_orderbooks.insert(i, Orderbook::new(i));
		}

		Self {
			id,
			description,
			extra_info,
			creator: from,
			outcomes,
			outcome_tags,
			end_time, 
			orderbooks: empty_orderbooks,
			winning_outcome: None,
			resoluted: false,
			liquidity: 0
		}
	}

	pub fn place_order(&mut self, from: String, outcome: u64, amt_of_shares: u64, spend: u64, price_per_share: u64) {
		assert_eq!(self.resoluted, false);
		let (spend_filled, shares_filled) = self.fill_matches(outcome, spend, price_per_share, amt_of_shares);
		let total_spend = spend - spend_filled;
		let shares_filled = amt_of_shares - shares_filled;
		let orderbook = self.orderbooks.get_mut(&outcome).unwrap();
		orderbook.place_order(from, outcome, spend, amt_of_shares, price_per_share, total_spend, shares_filled);
	}

	fn fill_matches(&mut self, outcome: u64, mut spend: u64, price_per_share: u64, mut shares_filled: u64) -> (u64, u64) {
		let market_price = self.get_market_price(outcome);
		if price_per_share < market_price || spend == 0 { return (spend, shares_filled); }
		let orderbook_ids = self.get_inverse_orderbook_ids(outcome);
		let shares_fillable = self.get_min_shares_fillable(outcome);
		let mut shares_to_fill = shares_filled;

		if shares_fillable < shares_to_fill {
			shares_to_fill = shares_fillable;
		}
		
		for orderbook_id in orderbook_ids {
			let orderbook = self.orderbooks.get_mut(&orderbook_id).unwrap();
			if !orderbook.market_order.is_none() {
				orderbook.fill_market_order(shares_to_fill);
			}
		}

		spend -= shares_to_fill * market_price;
		shares_filled -= shares_to_fill;
		
		return self.fill_matches(outcome, spend, price_per_share, shares_filled);
	}

	pub fn get_min_shares_fillable(&self, outcome: u64) -> u64 {
		let mut shares = None;
		let orderbook_ids = self.get_inverse_orderbook_ids(outcome);
		for orderbook_id in orderbook_ids {
			let orderbook = self.orderbooks.get(&orderbook_id).unwrap();
			let market_order_optional = orderbook.market_order;

			if !market_order_optional.is_none() {
				let market_order = orderbook.open_orders.get(&market_order_optional.unwrap()).unwrap();
				let left_to_fill = market_order.spend - market_order.filled;
				let shares_to_fill = left_to_fill  / market_order.price_per_share;
				if shares.is_none() || shares_to_fill < shares.unwrap() {
					shares = Some(shares_to_fill);
				}
			} 
		}

		return shares.unwrap();
	}

	pub fn get_market_price(&self, outcome: u64) -> u64 {
		let orderbook_ids = self.get_inverse_orderbook_ids(outcome);
		let mut market_price = 100;

 		for orderbook_id in orderbook_ids {
			let orderbook = self.orderbooks.get(&orderbook_id).unwrap();
			let market_order_optional = orderbook.market_order;

			if !market_order_optional.is_none() {
				let market_order = orderbook.open_orders.get(&market_order_optional.unwrap()).unwrap();
				market_price -= market_order.price_per_share;
			}
		}

		return market_price;
	}

	fn get_inverse_orderbook_ids(&self, principle_outcome: u64) -> Vec<u64> {
		let mut orderbooks = vec![];

		for i in 0..self.outcomes {
			if i != principle_outcome {
				orderbooks.push(i);
			}
		}

		return orderbooks;
	}

	pub fn resolute(&mut self,from: String, winning_outcome: Option<u64>) {
		// TODO: Make sure market can only be resoluted after end time
		assert_eq!(self.resoluted, false);
		assert_eq!(from, self.creator);
		assert!(winning_outcome == None || winning_outcome.unwrap() < self.outcomes);
		self.winning_outcome = winning_outcome;
		self.resoluted = true;
	}
	
	pub fn get_claimable(&self, from: String) -> u64 {
		assert_eq!(self.resoluted, true);
		let invalid = self.winning_outcome.is_none();
		let mut claimable = 0;

		if invalid {
			for (key, orderbook) in self.orderbooks.iter() {
				claimable += orderbook.get_spend_by(from.to_string());
			}
		} else {
			for (key, orderbook) in self.orderbooks.iter() {
				claimable += orderbook.get_open_order_value_for(from.to_string());
			}
			let winning_orderbook = self.orderbooks.get(&self.winning_outcome.unwrap()).unwrap();
			claimable += winning_orderbook.calc_claimable_amt(from);
		}
		return claimable;
	}

	pub fn delete_orders_for(&mut self, from: String) {
		for orderbook_id in 0..self.outcomes {
			let orderbook = self.orderbooks.get_mut(&orderbook_id).unwrap();
			orderbook.delete_orders_for(from.to_string());
		}
	}

	fn to_user_outcome_id(&self, user: String, outcome: u64) -> String {
		return format!("{}{}", user, outcome.to_string());
	}

	fn is_valid_payout(&self, payout_multipliers: &Vec<u64>, invalid: &bool) -> bool {
		return (payout_multipliers[0] == 10000 && payout_multipliers[1] == 0 && invalid == &false) || (payout_multipliers[0] == 0 && payout_multipliers[1] == 10000 && invalid == &false) || (payout_multipliers[0] == 5000 && payout_multipliers[1] == 5000 && invalid == &true);
	}
}

