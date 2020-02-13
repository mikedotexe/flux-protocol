use std::collections::{BTreeMap, HashMap};
use std::panic;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use near_bindgen::{near_bindgen};

pub mod order;
pub type Order = order::Order;

#[near_bindgen]
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct Orderbook {
	pub root: Option<u64>,
	pub market_order: Option<u64>,
	pub target_price_to_orders: BTreeMap<u64, BTreeMap<u64, Vec<u64>>>,
	pub order_to_usage: BTreeMap<u64, Vec<String>>,
	pub nonce: u64,
	pub outcome_id: u64
}
// TODO: Market orders are broken - don't update correctly
impl Orderbook {
	pub fn new(outcome: u64) -> Self {
		Self {
			root: None,
			target_price_to_orders: BTreeMap::new(),
			order_to_usage: BTreeMap::new(),
			market_order: None,
			nonce: 0,
			outcome_id: outcome,
		}
	}

	fn new_order_id(&mut self) -> u64 {
		let id = self.nonce;
		self.nonce = self.nonce + 1;
		return id;
	}

	// pub fn place_order(&mut self, from: String, outcome: u64, spend: u64, amt_of_shares: u64, price_per_share: u64, filled: u64, amt_of_shares_filled: u64) {
	// 	let order_id = self.new_order_id();
	// 	let mut new_order = Order::new(from, outcome, order_id, spend, amt_of_shares, price_per_share, filled, amt_of_shares_filled);

	// 	if filled == spend {
	// 		// self.filled_orders.insert(order_id, new_order);
	// 		return;
	// 	}

	// 	self.set_market_order(order_id, price_per_share);

	// 	if self.root.is_none() {
	// 		self.root = Some(new_order.to_owned().id);
	// 	} else {
	// 		new_order = self.find_and_add_parent(new_order.to_owned());
	// 	}

	// 	self.open_orders.insert(order_id, new_order);
	// }

	// fn set_market_order(&mut self, order_id: u64, price_per_share: u64) {
	// 	let current_market_order_id = self.market_order;
	// 	if current_market_order_id.is_none() {
	// 		self.market_order = Some(order_id);
	// 	} else {
	// 		let current_market_order = self.open_orders.get(&current_market_order_id.unwrap()).unwrap();
	// 		if current_market_order.price_per_share < price_per_share {
	// 			self.market_order = Some(order_id);
	// 		}
	// 	}
	// }

	// fn remove_market_order(&mut self) {
	// 	let market_order = self.open_orders.get(&self.market_order.unwrap()).unwrap();
	// 	if market_order.better_order_id.is_none() {
	// 		if market_order.worse_order_id.is_none() {
	// 			if market_order.parent.is_none() {
	// 				self.market_order = None;
	// 			} else {
	// 				self.market_order = market_order.parent;
	// 			}
	// 		} else {
	// 			self.market_order = market_order.worse_order_id;
	// 		}
	// 	} else {
	// 		self.market_order = market_order.better_order_id;
	// 	}
	// }

	pub fn remove_order(&mut self, order_id: u64) {

		// // if the order has been filled a littlebit push it to filled_orders
		// let order = self.open_orders.get_mut(&order_id).unwrap();
		// let parent = order.parent;
		// let better_order_id = order.better_order_id;
		// let worse_order_id = order.worse_order_id;
		
		// if order.amt_of_shares_filled > 0 {
		// 	// self.filled_orders.insert(order.id, order.to_owned());
		// }

		// if (Some(order_id) == self.market_order) {
		// 	self.remove_market_order();
		// }
		// // If removed order is root
		// if parent.is_none() {
		// 	self.root = better_order_id;

		// 	if !better_order_id.is_none() {
		// 		let better_order = self.open_orders.get_mut(&better_order_id.unwrap()).unwrap();
		// 		better_order.parent = None;
		// 	}
		// 	if !worse_order_id.is_none() {
		// 		self.update_and_replace_order(worse_order_id);
		// 	}
		// } else {
		// 	let parent = self.open_orders.get_mut(&parent.unwrap()).unwrap();
		// 	if parent.better_order_id == Some(order_id) {
		// 		parent.better_order_id = better_order_id;
		// 		self.update_and_replace_order(worse_order_id);
		// 	} else if parent.worse_order_id == Some(order_id) {
		// 		parent.worse_order_id = worse_order_id;
		// 		self.update_and_replace_order(better_order_id);
		// 	}
		// }

		// self.open_orders.remove(&order_id);
	}

	fn update_and_replace_order(&mut self, order_id: Option<u64>) {
		if !order_id.is_none() {
			
			// let order = self.open_orders.get_mut(&order_id.unwrap()).unwrap().to_owned();
			// let updated_order = self.find_and_add_parent(order);
			// self.open_orders.insert(updated_order.id, updated_order);
		}

	}

	// pub fn fill_matching_orders(&mut self, mut amt_of_shares_to_fill: u64, max_price: u64) -> u64 {
	// 	if amt_of_shares_to_fill == 0 { return 0 ;} 
	// 	else if self.market_order.is_none() {  return amt_of_shares_to_fill; }
		
	// 	let current_order = self.open_orders.get_mut(&self.market_order.unwrap()).unwrap();
	// 	if current_order.price_per_share < max_price { return amt_of_shares_to_fill }

	// 	let match_to_fill = (current_order.spend - current_order.filled) / max_price; // Some "dust" creation here - need a way to round orders better.

	// 	if match_to_fill <= amt_of_shares_to_fill {
	// 		current_order.filled += match_to_fill * max_price;
	// 		current_order.amt_of_shares_filled += match_to_fill;
	// 		amt_of_shares_to_fill -= match_to_fill;
	// 		if current_order.filled == current_order.spend || match_to_fill == 0{
	// 			self.remove_order(self.market_order.unwrap()); 
	// 		}
	// 	} else {
	// 		current_order.filled += amt_of_shares_to_fill * max_price;
	// 		current_order.amt_of_shares_filled += amt_of_shares_to_fill;
	// 		amt_of_shares_to_fill -= amt_of_shares_to_fill;
	// 	}
		
	// 	return self.fill_matching_orders(amt_of_shares_to_fill, max_price);
	// }

	fn delete_orders(&mut self, orders: Vec<u64>) {
		for order_id in orders {
			self.remove_order(order_id)
		}
	}

	pub fn fill_order(&mut self, order: &mut Order, fill_amt: u64, shares_filled: u64) {
		order.amt_of_shares_filled = order.amt_of_shares_filled + shares_filled;
		order.filled = order.filled + fill_amt;
	}

	// Consider recursion
	// fn find_and_add_parent(&mut self, new_order: Order) -> Order {
	// 	// let mut order_id_optional = self.root;
	// 	// let mut parent_order = None;
	// 	// let mut updated_order = new_order.to_owned();

	// 	// while parent_order.is_none() {
	// 	// 	let order = self.open_orders.get(&order_id_optional.unwrap()).unwrap();

	// 	// 	// Else statement code is duplicate.
	// 	// 	if order.is_better_price_than(new_order.to_owned()) {
	// 	// 		if !order.worse_order_id.is_none() {
	// 	// 			order_id_optional = order.worse_order_id;
	// 	// 		} else {
	// 	// 			parent_order = Some(order);
	// 	// 			updated_order.parent = Some(order.id);
	// 	// 		}
	// 	// 	} else {
	// 	// 		if !order.better_order_id.is_none() {
	// 	// 			order_id_optional = order.better_order_id;
	// 	// 		} else {
	// 	// 			parent_order = Some(order);
	// 	// 			updated_order.parent = Some(order.id);
	// 	// 		}
	// 	// 	}

	// 	}
		
	// 	self.add_child(parent_order.unwrap().id, updated_order.to_owned());
	// 	return updated_order;
	// }

	// fn add_child(&mut self, parent_id: u64, child: Order) {
	// 	let parent_order = self.open_orders.get_mut(&parent_id).unwrap();
	
	// 	if parent_order.is_better_price_than(child.to_owned()) {
	// 		parent_order.worse_order_id = Some(child.id);
	// 	} else {
	// 		parent_order.better_order_id = Some(child.id)
	// 	}
	// }
}