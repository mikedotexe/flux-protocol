use std::string::String;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Order {
	pub id: u128,
	pub creator: String,
	pub outcome: u64,
	pub spend: u128,
	pub amt_of_shares: u128,
	pub price_per_share: u128,
	pub filled: u128,
	pub shares_filled: u128,
	pub parent: Option<u128>,
	pub better_order_id: Option<u128>,
	pub worse_order_id: Option<u128>,
}


impl Order {
	pub fn new(creator: String, outcome: u64, id: u128, spend:u128, amt_of_shares: u128, price_per_share: u128, filled: u128, shares_filled: u128) -> Self {
		Order {
			id,
			creator,
			outcome,
			spend,
			amt_of_shares,
			price_per_share,
			filled,
			shares_filled,
			parent: None,
			better_order_id: None,
			worse_order_id: None,
		}
	}

	pub fn is_better_price_than(&self, compare_order: Order) -> bool {
		if self.price_per_share > compare_order.price_per_share {
			return true;
		} 
		return false;
	}
}
