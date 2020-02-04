use std::collections::BTreeMap;
use std::panic;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use near_bindgen::{near_bindgen};

pub mod order;
pub type Order = order::Order;

#[near_bindgen]
#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct Orderbook {
	pub root: Option<Order>,
	pub open_orders: BTreeMap<u64, Order>,
	pub filled_orders: BTreeMap<u64, Order>,
	pub market_order: Option<Order>,
	pub nonce: u64,
	pub outcome_id: u64
}
// TODO: Market orders are broken - don't update correctly
impl Orderbook {
	pub fn new(outcome: u64) -> Self {
		Self {
			root: None,
			open_orders: BTreeMap::new(),
			filled_orders: BTreeMap::new(), // Could be turned into Vec in current design
			market_order: None,
			nonce: 0,
			outcome_id: outcome,
		}
	}

	pub fn create_order(&mut self, from: String, outcome: u64, shares: u64, price: u64, shares_filled: u64) -> order::Order {
		let order_id = self.to_order_id();
		let new_order = order::Order::new(from, outcome, order_id, shares, price, shares_filled, None, None, None);
		return new_order;
	}

	pub fn add_new_order(&mut self, order: &mut order::Order) -> bool {
		if order.amount == order.amount_filled {
			self.filled_orders.insert(order.id, order.clone());
			return true;
		} 
		let updated_order = self.add_order(order);

		let market_order = self.get_market_order();
		if !self.market_order.is_none() && updated_order.price > market_order.unwrap().price {
			self.market_order = Some(updated_order);
		} 
		else if self.market_order.is_none() {
			self.market_order = Some(updated_order);
		}

		return true
	}


	pub fn add_order(&mut self, order: &mut Order) -> Order {
		let is_first_order = self.root.is_none();

		if is_first_order {
			order.prev = None;
			self.root = Some(order.clone());
			self.open_orders.insert(order.id, order.clone());
			self.market_order = Some(order.clone());
			return order.to_owned();
		}

		let order_parent_id = self.descend_tree_for_parent(order.price);
		self.open_orders.entry(order_parent_id).and_modify(|parent_order| {
			if order.price > parent_order.price {
				parent_order.better_order_id = Some(order.id);
			} else {
				parent_order.worse_order_id = Some(order.id);
			}
		});

		if order_parent_id == self.root.as_ref().unwrap().id {
			self.root = Some(self.open_orders.get(&order_parent_id).unwrap().to_owned());
		}
		
		order.prev = Some(order_parent_id);
		self.open_orders.insert(order.id, order.clone());
		return order.to_owned();
	}

	// TODO: Refactor? - could probably be split up
	// TODO: Rename 
	pub fn remove(&mut self, order_id: u64) -> &bool {
		let order = self.open_orders.get(&order_id).unwrap().to_owned();
		self.open_orders.remove(&order_id);
		let has_parent_order_id = !order.prev.is_none();
		let has_worse_order_id = !order.worse_order_id.is_none();
		let has_better_order_id = !order.better_order_id.is_none();
		
		if has_parent_order_id {
			let parent_order_id = order.prev.as_ref().unwrap();
			let root = self.root.as_mut().unwrap();
			self.open_orders.entry(*parent_order_id).and_modify(|parent_order| {
				if &order_id == parent_order.worse_order_id.as_ref().unwrap_or(&0) {
					parent_order.worse_order_id = None;
					if root.id == parent_order.id {
						root.worse_order_id = None;
					}
				} 
				else if &order_id == parent_order.better_order_id.as_ref().unwrap_or(&0) {
					parent_order.better_order_id = None;
					if root.id == parent_order.id {
						root.better_order_id = None;
					}
				} 
				else {
					panic!("Oops: the order's parent doesn't link back to this order!")
				}
			});
		} else {
			self.root = None;
		}

		if has_worse_order_id {
			let worse_order_id = order.worse_order_id.as_ref().unwrap();
			let mut worse_order = self.open_orders.get(worse_order_id).unwrap().to_owned();
			self.add_order(&mut worse_order);
		}

		if has_better_order_id {
			let better_order_id = order.better_order_id.as_ref().unwrap();
			let mut better_order = self.open_orders.get(better_order_id).unwrap().to_owned();
			self.add_order(&mut better_order);
		}

		let market_order_exists = !self.market_order.is_none();
		if market_order_exists {
			let market_order = self.market_order.as_ref().unwrap();
			if market_order.id == order_id {
				let new_market_order = self.get_new_market_order(Some(&order)).unwrap_or(market_order.clone());
				if new_market_order.id == market_order.id {
					self.market_order = None;
				} else {
					self.market_order = self.get_new_market_order(Some(&order));
				}
				
			}
		}

		return &true;
	}

	// Rename - be more specific
	// Check for refactor
	pub fn fill_matching_orders(&mut self, amount_of_shares: u64, price: u64) -> u64 {
		let root_optional = self.root.as_ref();
		if amount_of_shares == 0 || root_optional.is_none() { return amount_of_shares }
		let matching_price = 100 - price;
		let root = root_optional.unwrap();
		let matching_order_id_optional = self.get_order_by_price(&root, matching_price);
		
		if matching_order_id_optional.is_none() { return amount_of_shares }
		
		let matching_order_id = matching_order_id_optional.unwrap();
		let matching_order = self.open_orders.get(&matching_order_id).unwrap();
		let match_shares_to_fill = matching_order.amount - matching_order.amount_filled;

		let to_fill;
		if match_shares_to_fill >= amount_of_shares {
			to_fill = amount_of_shares;
		} 
		else {
			to_fill = match_shares_to_fill;
		}

		let matching_order_id = matching_order.id.clone();
		let matching_order_owner = matching_order.owner.to_string();
		self.fill_order(matching_order_id, to_fill);
		let left_to_fill = amount_of_shares - to_fill;

		return self.fill_matching_orders(left_to_fill, price);
	}

	fn fill_order(&mut self, order_id: u64, fill_amount: u64) {
		let mut filled = false;
		self.open_orders.entry(order_id).and_modify( |order| {
			order.amount_filled += fill_amount;
			if order.amount_filled == order.amount { filled = true; }
		});
		
		if filled {
			let order_copy = self.open_orders.get(&order_id).unwrap().clone();
			self.filled_orders.insert(order_id, order_copy);
			self.remove(order_id);
		}
	}

	// TODO: refactor?
	pub fn get_and_delete_earnings(&mut self, from: String, invalid: bool) -> (u64, u64) {

		let (value_in_open_orders, earnings, open_orders_to_delete, filled_orders_to_delete) = self.get_earnings(from, true, invalid);
		if open_orders_to_delete.len() > 0 {
			self.remove_orders(open_orders_to_delete);
		}

		if filled_orders_to_delete.len() > 0 {
			self.remove_filled_orders(filled_orders_to_delete);
		}

		return (value_in_open_orders, earnings);
	}

	// TODO: rename
	// refactor
	pub fn get_earnings(&self, from: String, and_delete: bool, invalid: bool) ->  (u64, u64, Vec<u64>, Vec<u64>) {
		let mut earnings = 0;
		let mut value_in_open_orders = 0;
		let mut open_orders_to_delete = vec![];
		let mut filled_orders_to_delete = vec![];

		for (_key, order) in &self.open_orders {
			if order.owner == from {
				if !invalid {
					earnings += order.amount_filled * 100;
					value_in_open_orders += (order.amount - order.amount_filled) * order.price;
				} else {
					earnings += order.amount * order.price;
					value_in_open_orders += (order.amount - order.amount_filled) * order.price;
				}
				if and_delete { open_orders_to_delete.push(order.id) }
			}
		}
		for(_key, order) in &self.filled_orders {
			if order.owner == from {
				if invalid {
					earnings += order.price * order.amount;
				} else {
					earnings += order.amount_filled * 100;
				}
				if and_delete { filled_orders_to_delete.push(order.id) }			
			}
		}
		return (value_in_open_orders, earnings, open_orders_to_delete, filled_orders_to_delete);
	}

	// TODO: Rename 
	pub fn remove_orders(&mut self, orders: Vec<u64>) {
		for order_id in orders {
			self.remove(order_id);
		}
	}

	fn remove_filled_orders(&mut self, to_remove: Vec<u64>) {
		for order_id in to_remove {
			self.filled_orders.remove(&order_id);
		}
	}
	
	// Recursive function that searches for a specific target_price within the orderbook BST
	pub fn get_order_by_price(&self, mut current_order: &Order, target_price: u64) -> Option<u64> {
		if current_order.price == target_price {
			return Some(current_order.id);
		}
		else if current_order.price < target_price && !current_order.better_order_id.is_none() {
			let next_order_id = current_order.better_order_id.as_ref().unwrap();
			let next_order = &mut self.open_orders.get(next_order_id).unwrap();
			return self.get_order_by_price(next_order, target_price);
		} 
		else if current_order.price > target_price && !current_order.worse_order_id.is_none() {
			let next_order_id = current_order.worse_order_id.as_ref().unwrap();
			let next_order = &mut self.open_orders.get(next_order_id).unwrap();
			return self.get_order_by_price(next_order, target_price);		
		}
		return None;
	}

	// ??
	pub fn descend_tree_for_parent(&mut self, price: u64) -> u64 {
		let root = self.root.as_ref().unwrap();
		let mut current_order_id = root.id;
		let mut next_order_id: Option<&u64> = self.get_next_order(&current_order_id, price);
		while !next_order_id.is_none() {
			current_order_id = *next_order_id.unwrap();
			next_order_id = self.get_next_order(&current_order_id, price);
		}
	
		return current_order_id;
	}

	// TODO: Rename
	pub fn to_order_id(&mut self) -> u64 {
		self.nonce += 1;
		return self.nonce;
	}

	pub fn get_open_orders(&self) -> &BTreeMap<u64, Order> {
		return &self.open_orders;
	}

	pub fn get_open_orders_for_user(&self, from: String) -> Vec<Order> {
		let open_orders = &self.open_orders;
		let mut user_orders = vec![];
		for (_order_id, order) in open_orders {
			if order.owner == from { user_orders.push(order.clone()); }
		}
		return user_orders;
	}

	pub fn get_filled_orders_for_user(&self, from: String) -> Vec<Order> {
		let filled_orders = &self.filled_orders;
		let mut user_orders = vec![];
		for (_order_id, order) in filled_orders {
			if order.owner == from { user_orders.push(order.clone()); }
		}
		return user_orders;
	}

	pub fn get_next_order(&mut self, current_order_id: &u64, new_order_price: u64) -> Option<&u64> {
		let current_order = self.open_orders.get(&current_order_id).unwrap();
		if new_order_price <= current_order.price {
			return current_order.worse_order_id.as_ref();
		} else {
			return current_order.better_order_id.as_ref();
		}
	}

	pub fn get_order_by_id(&self, id: &u64) -> &Order {
		return self.open_orders.get(id).unwrap();
	}

	pub fn get_market_order(&self) -> Option<&Order> {
		if !self.market_order.is_none() {
			let market_order = self.market_order.as_ref().unwrap();
			return Some(market_order);
		} else {
			return None
		}
	}

	fn get_new_market_order(&self, last_order: Option<&Order>) -> Option<Order> {
		let mut current_order: &Order;

		if last_order.is_none() {
			if self.root.is_none() {return None}
			current_order = &self.root.as_ref().unwrap();
		}
		else {
			current_order = last_order.unwrap();
		}

		if current_order.better_order_id.is_none() {
			return Some(current_order.clone());
		} else {
			let next_order_id = current_order.better_order_id.as_ref().unwrap();
			let next_order = self.open_orders.get(next_order_id);
			return Some(self.get_new_market_order(next_order).unwrap());
		}
	}
}