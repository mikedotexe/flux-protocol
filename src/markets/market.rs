use std::string::String;
use std::collections::BTreeMap;
use std::collections::HashMap;
use near_bindgen::{near_bindgen, env};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub mod orderbook;
type Orderbook = orderbook::Orderbook;
type Order = orderbook::Order;

// TODO: Endtime has to be in blocks instead of ms / s
// TODO: Add resolution_url
// TODO: When filling an order for a price > market price the tx fails
// TODO: Share denomination seems off by 1 decimal - don't know if frontend or backend fix
#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct Market {
	pub id: u64,
	pub description: String,
	pub extra_info: String,
	pub creator: String,
	pub outcomes: u64,
	pub outcome_tags: Vec<String>,
	pub categories: Vec<String>,
	pub last_price_for_outcomes: HashMap<u64, u128>,
	pub creation_time: u64,
	pub end_time: u64,
	pub orderbooks: BTreeMap<u64, orderbook::Orderbook>,
	pub winning_outcome: Option<u64>,
	pub resoluted: bool,
	pub liquidity: u128,
	pub fee_percentage: u128,
	pub cost_percentage: u128,
	pub api_source: String,
	pub fees_collected: u128
}

impl Market {
	pub fn new(id: u64, from: String, description: String, extra_info: String, outcomes: u64, outcome_tags: Vec<String>, categories: Vec<String>, end_time: u64, fee_percentage: u128, cost_percentage: u128, api_source: String) -> Self {
		let mut empty_orderbooks = BTreeMap::new();
		// TODO get blocktime at creation

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
			categories,
			last_price_for_outcomes: HashMap::new(),
			creation_time: env::block_timestamp(),
			end_time,
			orderbooks: empty_orderbooks,
			winning_outcome: None,
			resoluted: false,
			liquidity: 0,
			fee_percentage,
			cost_percentage,
			api_source,
			fees_collected: 0,
		}
	}

	pub fn place_order(&mut self, from: String, outcome: u64, amt_of_shares: u128, spend: u128, price: u128) {
		assert!(spend > 0);
		assert!(price > 0 && price < 100);
		assert_eq!(self.resoluted, false);
		assert!(env::block_timestamp() < self.end_time);
		let fee_adjusted_spend = spend*(100-self.fee_percentage)/100;
		let (spend_filled, shares_filled) = self.fill_matches(outcome, fee_adjusted_spend, price, 0);
		let total_spend = fee_adjusted_spend - spend_filled;
		self.liquidity += fee_adjusted_spend;
		let shares_filled = shares_filled;
		let orderbook = self.orderbooks.get_mut(&outcome).unwrap();
		orderbook.place_order(from, outcome, fee_adjusted_spend, amt_of_shares, price, total_spend, shares_filled);
		self.fees_collected += spend - fee_adjusted_spend
	}

	fn fill_matches(&mut self, outcome: u64, mut spend: u128, price: u128, mut shares_filled: u128) -> (u128, u128) {
		let market_price = self.get_market_price(outcome);
		let mut shares_to_fill = spend / market_price;
		if price < market_price || spend < 100 { return (spend, shares_filled); }
		let orderbook_ids = self.get_inverse_orderbook_ids(outcome);
		let shares_fillable = self.get_min_shares_fillable(outcome);
		self.last_price_for_outcomes.insert(outcome, market_price);

		if shares_fillable < shares_to_fill {
			shares_to_fill = shares_fillable;
		}


		for orderbook_id in orderbook_ids {
			let orderbook = self.orderbooks.get_mut(&orderbook_id).unwrap();
			if !orderbook.best_price.is_none() {
				let best_price = orderbook.get_best_price();
				self.last_price_for_outcomes.insert(orderbook_id, best_price);
				orderbook.fill_best_orders(shares_to_fill);
			}
		}
		spend -= shares_to_fill * market_price;
		shares_filled += shares_to_fill;

		return (spend, shares_filled);
	}

	pub fn get_min_shares_fillable(&self, outcome: u64) -> u128 {
		let mut shares = None;
		let orderbook_ids = self.get_inverse_orderbook_ids(outcome);
		for orderbook_id in orderbook_ids {
			let orderbook = self.orderbooks.get(&orderbook_id).unwrap();

            if let Some((best_price, best_order_map)) = orderbook.orders_by_price.iter().next() {
                let mut left_to_fill = 0;
                let mut shares_to_fill = 0;
                for (order_id, _) in best_order_map.iter() {
					let order = orderbook.open_orders.get(&order_id).unwrap();
                    left_to_fill += order.spend - order.filled;
                    shares_to_fill += left_to_fill / best_price;
                }
                if shares.is_none() || shares_to_fill < shares.unwrap() {
                    shares = Some(shares_to_fill);
                }
            }
		}
		return shares.unwrap();
	}

	pub fn get_market_prices(&self) -> BTreeMap<u64, u128> {
		let mut market_prices: BTreeMap<u64, u128> = BTreeMap::new();
		for outcome in 0..self.outcomes {
			let market_price = self.get_market_price(outcome);
			market_prices.insert(outcome, market_price);
		}
		return market_prices;
	}

	pub fn get_market_price(&self, outcome: u64) -> u128 {
		let orderbook_ids = self.get_inverse_orderbook_ids(outcome);
		let mut market_price = 100;

 		for orderbook_id in orderbook_ids {
			let orderbook = self.orderbooks.get(&orderbook_id).unwrap();
			let best_price = orderbook.best_price;

			if !best_price.is_none() {
				market_price -= best_price.unwrap();
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
		assert!(env::block_timestamp() >= self.end_time, "market hasn't ended yet");
		assert_eq!(self.resoluted, false);
		assert_eq!(from, self.creator);
		assert!(winning_outcome == None || winning_outcome.unwrap() < self.outcomes);
		self.winning_outcome = winning_outcome;
		self.resoluted = true;
	}

	pub fn get_claimable(&self, from: String) -> u128 {
		assert_eq!(self.resoluted, true);
		assert!(env::block_timestamp() >= self.end_time, "market hasn't ended yet");
		let invalid = self.winning_outcome.is_none();
		let mut claimable = 0;

		if invalid {
			for (_, orderbook) in self.orderbooks.iter() {
				claimable += orderbook.get_spend_by(from.to_string());
			}
		} else {
			for (_, orderbook) in self.orderbooks.iter() {
				claimable += orderbook.get_open_order_value_for(from.to_string());
			}
			let winning_orderbook = self.orderbooks.get(&self.winning_outcome.unwrap()).unwrap();
			claimable += winning_orderbook.calc_claimable_amt(from);
		}
		return claimable;
	}

	pub fn get_claimable_fees(&self) -> u128 {
	    return self.fees_collected;
	}

    // Updates the best price for an order once initial best price is filled
	fn update_next_best_price(&mut self, inverse_orderbook_ids: &Vec<u64>, first_iteration: &bool, outcome_to_price_share_pointer: &mut HashMap<u64, (u128, u128)>, best_order_exists: &mut bool, market_price: &mut u128, lowest_liquidity: &u128) {
	    for orderbook_id in inverse_orderbook_ids {
            let orderbook = self.orderbooks.get(&orderbook_id).unwrap();
            if !first_iteration {
                if outcome_to_price_share_pointer.get_mut(orderbook_id).is_none() {continue}
                outcome_to_price_share_pointer.get_mut(orderbook_id).unwrap().1 -= lowest_liquidity;
                let price_liquidity = outcome_to_price_share_pointer.get(orderbook_id).unwrap();
                let liquidity = price_liquidity.1;

                if liquidity == 0 {
                    // get next best price
                    let next_best_price_prom = orderbook.orders_by_price.range(0..price_liquidity.0 - 1).next();

                    if next_best_price_prom.is_none() {
                        outcome_to_price_share_pointer.remove(orderbook_id);
                        continue;
                    }
                    *best_order_exists = true;
                    let next_best_price = *next_best_price_prom.unwrap().0;
                    let add_to_market_price =  price_liquidity.0 - next_best_price;
                    *market_price += add_to_market_price;
                    outcome_to_price_share_pointer.insert(*orderbook_id, (next_best_price, orderbook.get_liquidity_for_price(next_best_price)));
                }
            }
        }
	}

    // Updates the lowest liquidity available amongst best prices
	fn update_lowest_liquidity(&mut self, inverse_orderbook_ids: &Vec<u64>, first_iteration: &bool, lowest_liquidity: &mut u128, outcome_to_price_share_pointer: &mut HashMap<u64, (u128, u128)>, best_order_exists: &mut bool) {
	    *best_order_exists = false;
	    for orderbook_id in inverse_orderbook_ids {
            // Get lowest liquidity at new price
            let orderbook = self.orderbooks.get(&orderbook_id).unwrap();
            if *first_iteration {
                let price = orderbook.best_price;
                if price.is_none() {continue}
                *best_order_exists = true;
                let liquidity = orderbook.get_liquidity_for_price(price.unwrap());
                outcome_to_price_share_pointer.insert(*orderbook_id, (price.unwrap(), liquidity));
            }
            if outcome_to_price_share_pointer.get(orderbook_id).is_none() {continue}
            let liquidity = outcome_to_price_share_pointer.get(orderbook_id).unwrap().1;
            if *lowest_liquidity == 0 {*lowest_liquidity = liquidity}
            else if *lowest_liquidity > liquidity { *lowest_liquidity = liquidity}

        }

	}

	pub fn get_liquidity(&mut self, outcome: u64, spend: u128, price: u128) -> u128 {
		let inverse_orderbook_ids = self.get_inverse_orderbook_ids(outcome);
		// Mapped outcome to price and liquidity left
		let mut outcome_to_price_share_pointer: HashMap<u64,  (u128, u128)> = HashMap::new();
		let mut max_spend = 0;
		let mut max_shares = 0;
		let mut market_price = self.get_market_price(outcome);
		let mut best_order_exists = true;
		let mut lowest_liquidity = 0;
		let mut first_iteration = true;

		while max_spend < spend && market_price <= price && best_order_exists {
			self.update_next_best_price(&inverse_orderbook_ids,
			&first_iteration,
			&mut outcome_to_price_share_pointer,
			&mut best_order_exists,
			&mut market_price,
			&lowest_liquidity);

			lowest_liquidity = 0;
			if market_price <= price {
				self.update_lowest_liquidity(&inverse_orderbook_ids,
				&first_iteration,
				&mut lowest_liquidity,
                &mut outcome_to_price_share_pointer,
                &mut best_order_exists);
				max_spend += lowest_liquidity * market_price;
				max_shares += lowest_liquidity;
			}
			first_iteration = false;
		}

		return max_spend;
	}


	pub fn delete_orders_for(&mut self, from: String) {
		for orderbook_id in 0..self.outcomes {
			let orderbook = self.orderbooks.get_mut(&orderbook_id).unwrap();
			orderbook.delete_orders_for(from.to_string());
		}
	}
}

