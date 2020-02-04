use std::string::String;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct Order {
	pub id: u64,
	pub owner: String,
	pub outcome: u64,
	pub amount: u64,
	pub price: u64,
	pub amount_filled: u64,
	pub prev: Option<u64>,
	pub better_order_id: Option<u64>,
	pub worse_order_id: Option<u64>,
}


impl Order {
	pub fn new(owner: String, outcome: u64, id: u64, amount: u64, price: u64, amount_filled: u64, prev: Option<u64>, better_order_id: Option<u64>, worse_order_id: Option<u64>) -> Self {
		Order {
			id,
			owner,
			outcome,
			amount,
			price,
			amount_filled,
			prev,
			better_order_id,
			worse_order_id,
		}
	}

	pub fn better_price_than(&self, compare_order: Order) -> bool {
		if self.price > compare_order.price {
			return true;
		} 
		return false;
	}
}
