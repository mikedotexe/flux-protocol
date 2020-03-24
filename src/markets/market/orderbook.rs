use std::collections::{BTreeMap};
use std::cmp;
use std::panic;
use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{near_bindgen};
use serde::{Deserialize, Serialize};

pub mod order;
pub type Order = order::Order;

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct Orderbook {
	pub root: Option<u128>,
	pub market_order: Option<u128>,
	pub open_orders: BTreeMap<u128, Vec<Order>>,
	pub filled_orders: BTreeMap<u128, Vec<Order>>,
	pub spend_by_user: BTreeMap<String, u128>,
	pub nonce: u128,
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

    // Grabs latest nonce
	fn new_order_id(&mut self) -> u128 {
		let id = self.nonce;
		self.nonce = self.nonce + 1;
		return id;
	}

    // Places order in orderbook
	pub fn place_order(&mut self, from: String, outcome: u64, spend: u128, amt_of_shares: u128, price_per_share: u128, filled: u128, amt_of_shares_filled: u128) {
		let order_id = self.new_order_id();
		let mut new_order = Order::new(from.to_string(), outcome, order_id, spend, amt_of_shares, price_per_share, filled, amt_of_shares_filled);
		*self.spend_by_user.entry(from.to_string()).or_insert(0) += spend;

        // If all of spend is filled, state order is fully filled
		if filled >= spend {
			let entry = *self.filled_orders.entry(price_per_share).or_insert(Vec::new());
			entry.push(new_order);
			return;
		}

        // If there is a remaining order, set this new order as the new market rate
		self.set_market_order(price_per_share);

        // Insert order into order map
		let entry = *self.open_orders.entry(price_per_share).or_insert(Vec::new());
		entry.push(new_order);
	}

    // Updates current market order price
	fn set_market_order(&mut self, price_per_share: u128) {
		let current_market_order_price = self.market_order;
		if current_market_order_price.is_none() {
			self.market_order = Some(price_per_share);
		} else {
			let Some((current_market_price, current_market_order)) = self.open_orders.first_key_value();
			if *current_market_price < price_per_share {
				self.market_order = Some(price_per_share);
			}
		}
	}

    // Remove order from orderbook -- added price_per_share - might be a problem at markets/market level, also if invalid order id passed behaviour undefined
	pub fn remove_order(&mut self, order_id: u128, price_per_share: u128) -> u128 {
		// Get orders at price
		let order_vec = self.open_orders.get_mut(&price_per_share).unwrap();
        let outstanding_spend;

        for i in 0..order_vec.len() {
            if order_vec[i].id == order_id {
                outstanding_spend = order_vec[i].spend - order_vec[i].filled;
                *self.spend_by_user.get_mut(&order_vec[i].creator).unwrap() -= outstanding_spend;

                if order_vec[i].amt_of_shares_filled > 0 {
                    let entry = *self.filled_orders.entry(price_per_share).or_insert(Vec::new());
                	entry.push(order_vec[i].clone());
                }

                order_vec.swap_remove(i);
                if order_vec.is_empty() {
                    self.open_orders.remove(&price_per_share);
                    let Some((min_key, min_value)) = self.open_orders.first_key_value();
                    self.market_order = Some(*min_key);
                }
                break;
            }
        }
		return outstanding_spend;
	}

	// TODO: Should catch these rounding errors earlier, right now some "dust" will be lost.
	pub fn fill_market_order(&mut self, mut amt_of_shares_to_fill: u128) {
		let Some((current_order_key, current_order_vec)) = self.open_orders.first_key_value();
		// Iteratively fill market orders until done
		for i in 0..current_order_vec.len() {
		    if amt_of_shares_to_fill > 0 {
                let shares_remaining_in_order = current_order_vec[i].amt_of_shares - current_order_vec[i].amt_of_shares_filled;
                let filling = cmp::min(shares_remaining_in_order, amt_of_shares_to_fill);

                current_order_vec[i].amt_of_shares_filled += filling;
                current_order_vec[i].filled += filling * current_order_vec[i].price_per_share;

                if current_order_vec[i].spend - current_order_vec[i].filled < 100 { // some rounding erros here might cause some stack overflow bugs that's why this is build in.
                    self.remove_order(current_order_vec[i].id, current_order_vec[i].price_per_share);
                }

                amt_of_shares_to_fill -= filling;
		    } else {
		        break
		    }
		}
	}

	pub fn calc_claimable_amt(&self, from: String) -> u128 {
		let mut claimable = 0;
		for (_, order_vec) in self.open_orders.iter() {
		    for i in 0..order_vec.len() {
		        if order_vec[i].creator == from {
                    claimable += order_vec[i].amt_of_shares_filled * 100;
                }
		    }
		}
		for (_, order_vec) in self.filled_orders.iter() {
		    for i in 0..order_vec.len() {
		        if order_vec[i].creator == from {
                    claimable += order_vec[i].amt_of_shares_filled * 100;
                }
		    }
		}
		return claimable;
	}

	pub fn delete_orders_for(&mut self, from: String) {
		for (_, order_vec) in &mut self.open_orders {
		    for i in 0..order_vec.len() {
                if order_vec[i].creator == from {
                    self.remove_order(order_vec[i].id, order_vec[i].price_per_share);
                }
            }
		}

		for (_, order_vec) in &mut self.filled_orders {
		    for i in 0..order_vec.len() {
			    if order_vec[i].creator == from {
				    self.remove_filled_order(order_vec[i].id, order_vec[i].price_per_share);
			    }
			}
		}
	}

    fn remove_filled_order(self, order_id : u128, price_per_share : u128) {
        // Get filled orders at price
        let filled_order_vec = self.filled_orders.get_mut(&price_per_share).unwrap();
        for i in 0..filled_order_vec.len() {
            if filled_order_vec[i].id == order_id {
                filled_order_vec.swap_remove(i);
                if filled_order_vec.is_empty() {
                    self.filled_orders.remove(&price_per_share);
                }
                return;
            }
        }
        return;
    }

	pub fn get_open_order_value_for(&self, from: String) -> u128 {
		let mut claimable = 0;
		for (_, order_vec) in self.open_orders.iter() {
		    for i in 0..order_vec.len() {
		        if order_vec[i].creator == from {
                    claimable += order_vec[i].spend - order_vec[i].filled;
                }
		    }
		}
		return claimable;
	}

	pub fn get_spend_by(&self, from: String) -> u128 {
		return *self.spend_by_user.get(&from).unwrap_or(&0);
	}
}