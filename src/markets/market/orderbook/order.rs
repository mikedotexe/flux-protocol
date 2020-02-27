use std::string::String;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Order {
	pub id: u64,
	pub creator: String,
	pub outcome: u64,
	pub spend: u64,
	pub amt_of_shares: u64,
	pub price_per_share: u64,
	pub filled: u64,
	pub amt_of_shares_filled: u64,
	pub parent: Option<u64>,
	pub better_order_id: Option<u64>,
	pub worse_order_id: Option<u64>,
}


impl Order {
	pub fn new(creator: String, outcome: u64, id: u64, spend:u64, amt_of_shares: u64, price_per_share: u64, filled: u64, amt_of_shares_filled: u64) -> Self {
		Order {
			id,
			creator,
			outcome,
			spend,
			amt_of_shares,
			price_per_share,
			filled,
			amt_of_shares_filled,
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
