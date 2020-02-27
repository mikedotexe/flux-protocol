use std::collections::{BTreeMap, HashMap};
use std::panic;
use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{near_bindgen};
use serde::{Deserialize, Serialize};

pub mod order;
pub type Order = order::Order;

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct Orderbook {
	pub root: Option<u64>,
	pub market_order: Option<u64>,
	pub open_orders: BTreeMap<u64, Order>,
	pub filled_orders: BTreeMap<u64, Order>,
	pub spend_by_user: BTreeMap<String, u64>,
	pub nonce: u64,
	pub outcome_id: u64
}
// TODO: Market orders are broken - don't update correctly
impl Orderbook {
	pub fn new(outcome: u64) -> Self {
		Self {
			root: None,
			open_orders: BTreeMap::new(),
			filled_orders: BTreeMap::new(),
			spend_by_user: BTreeMap::new(),
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

	pub fn place_order(&mut self, from: String, outcome: u64, spend: u64, amt_of_shares: u64, price_per_share: u64, filled: u64, amt_of_shares_filled: u64) {
		let order_id = self.new_order_id();
		let mut new_order = Order::new(from.to_string(), outcome, order_id, spend, amt_of_shares, price_per_share, filled, amt_of_shares_filled);
		*self.spend_by_user.entry(from.to_string()).or_insert(0) += spend;

		if filled >= spend {
			self.filled_orders.insert(order_id, new_order);
			return;
		}

		self.set_market_order(order_id, price_per_share);

		if self.root.is_none() {
			self.root = Some(new_order.clone().id);
		} else {
			new_order = self.find_and_add_parent(new_order.clone());
		}

		self.open_orders.insert(order_id, new_order);
	}

	fn set_market_order(&mut self, order_id: u64, price_per_share: u64) {
		let current_market_order_id = self.market_order;
		if current_market_order_id.is_none() {
			self.market_order = Some(order_id);
		} else {
			let current_market_order = self.open_orders.get(&current_market_order_id.unwrap()).unwrap();
			if current_market_order.price_per_share < price_per_share {
				self.market_order = Some(order_id);
			}
		}
	}

	fn remove_market_order(&mut self) {
		let market_order = self.open_orders.get(&self.market_order.unwrap()).unwrap();
		if market_order.better_order_id.is_none() {
			if market_order.worse_order_id.is_none() {
				if market_order.parent.is_none() {
					self.market_order = None;
				} else {
					self.market_order = market_order.parent;
				}
			} else {
				self.market_order = market_order.worse_order_id;
			}
		} else {
			self.market_order = market_order.better_order_id;
		}
	}

	pub fn remove_order(&mut self, order_id: u64) -> u64 {
		let order = self.open_orders.get_mut(&order_id).unwrap();
		let outstanding_spend = order.spend - order.filled;
		*self.spend_by_user.get_mut(&order.creator).unwrap() -= outstanding_spend;
		let parent = order.parent;
		let better_order_id = order.better_order_id;
		let worse_order_id = order.worse_order_id;
		if order.amt_of_shares_filled > 0 {
			self.filled_orders.insert(order.id, order.clone());
		}
		
		if (Some(order_id) == self.market_order) {
			self.remove_market_order();
		}
		// If removed order is root
		if parent.is_none() {
			
			self.root = better_order_id;
			
			if !better_order_id.is_none() {
				let better_order = self.open_orders.get_mut(&better_order_id.unwrap()).unwrap();
				better_order.parent = None;
				if !worse_order_id.is_none() {
					self.update_and_replace_order(worse_order_id);
				}
			} else if !worse_order_id.is_none() {
				self.root = worse_order_id;
				let worse_order = self.open_orders.get_mut(&worse_order_id.unwrap()).unwrap();
				worse_order.parent = None;
			}
		} 
		else {
			let parent = self.open_orders.get_mut(&parent.unwrap()).unwrap();
			if parent.better_order_id == Some(order_id) {
				parent.better_order_id = better_order_id;
				self.update_and_replace_order(worse_order_id);
			} else if parent.worse_order_id == Some(order_id) {
				parent.worse_order_id = worse_order_id;
				self.update_and_replace_order(better_order_id);
			}
		}
		
		
		self.open_orders.remove(&order_id);
		return outstanding_spend;
	}

	fn update_and_replace_order(&mut self, order_id: Option<u64>) {
		if !order_id.is_none() {
			let order = self.open_orders.get_mut(&order_id.unwrap()).unwrap().to_owned();
			let updated_order = self.find_and_add_parent(order);
			self.open_orders.insert(updated_order.id, updated_order);
		}

	}
	
	// TODO: Should catch these rounding errors earlier, right now some "dust" will be lost.
	pub fn fill_market_order(&mut self, mut amt_of_shares_to_fill: u64) {
		let current_order = self.open_orders.get_mut(&self.market_order.unwrap()).unwrap();
		current_order.amt_of_shares_filled += amt_of_shares_to_fill;
		current_order.filled += amt_of_shares_to_fill * current_order.price_per_share;
		if current_order.spend - current_order.filled < 100 { // some rounding erros here might cause some stack overflow bugs that's why this is build in.
			self.remove_order(self.market_order.unwrap()); 
		}
	}


	fn find_and_add_parent(&mut self, new_order: Order) -> Order {
		let mut order_id_optional = self.root;
		let mut parent_order = None;
		let mut updated_order = new_order.clone();
		
		while parent_order.is_none() {
			let order = self.open_orders.get(&order_id_optional.unwrap()).unwrap();
			if order.is_better_price_than(new_order.clone()) {
				if !order.worse_order_id.is_none() {
					order_id_optional = order.worse_order_id;
				} else {
					parent_order = Some(order);
					updated_order.parent = Some(order.id);
				}
			} else {
				if !order.better_order_id.is_none() {
					order_id_optional = order.better_order_id;
				} else {
					parent_order = Some(order);
					updated_order.parent = Some(order.id);
				}
			}

		}
		
		self.add_child(parent_order.unwrap().id, updated_order.clone());
		return updated_order;
	}

	fn add_child(&mut self, parent_id: u64, child: Order) {
		let parent_order = self.open_orders.get_mut(&parent_id).unwrap();
	
		if parent_order.is_better_price_than(child.to_owned()) {
			parent_order.worse_order_id = Some(child.id);
		} else {
			parent_order.better_order_id = Some(child.id)
		}
	}

	pub fn calc_claimable_amt(&self, from: String) -> u64 {
		let mut claimable = 0;
		for (_, order) in self.open_orders.iter() {
			if order.creator == from {
				claimable += order.amt_of_shares_filled * 100;
			}
		}
		for (_, order) in self.filled_orders.iter() {
			if order.creator == from {
				claimable += order.amt_of_shares_filled * 100;
			}
		}
		return claimable;
	}

	pub fn delete_orders_for(&mut self, from: String) {
		let mut open_orders_to_delete = vec![];
		let mut filled_orders_to_delete = vec![];

		for (_, order) in &mut self.open_orders {
			if order.creator == from {
				open_orders_to_delete.push(order.id);
			}
		}

		for (_, order) in &mut self.filled_orders {
			if order.creator == from {
				filled_orders_to_delete.push(order.id);
			}
		}

		self.delete_open_orders(open_orders_to_delete);
		self.delete_filled_orders(filled_orders_to_delete);
	}

	fn delete_filled_orders(&mut self, order_ids: Vec<u64>) {
		for order_id in order_ids {
			self.filled_orders.remove(&order_id);
		}
	}
	fn delete_open_orders(&mut self, order_ids: Vec<u64>) {
		for order_id in order_ids {
			self.open_orders.remove(&order_id);
		}
	}

	pub fn get_open_order_value_for(&self, from: String) -> u64 {
		let mut claimable = 0;
		for (_, order) in self.open_orders.iter() {
			if order.creator == from {
				claimable += order.spend - order.filled;
			}
		}
		return claimable;
	}

	pub fn get_spend_by(&self, from: String) -> u64 {
		return *self.spend_by_user.get(&from).unwrap_or(&0);
	}
}