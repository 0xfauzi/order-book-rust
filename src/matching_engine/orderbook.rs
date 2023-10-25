use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
pub struct OrderBook {
    bids_per_price: HashMap<Price, Limit>, // for O(1) retrieval of limits
    asks_per_price: HashMap<Price, Limit>, //for O(1) retrieval of limits
    bids: BinaryHeap<Reverse<Limit>>, //Min heap, lowest price first
    asks: BinaryHeap<Limit> //Max heap, highest price first
}


impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bids_per_price: HashMap::new(),
            asks_per_price: HashMap::new(),
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new()
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price_object = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::Bid => {
                match self.bids_per_price.get_mut(&price_object) {
                    None => {
                        let mut limit = Limit::new(price_object);
                        limit.add_order(order);
                        self.bids_per_price.insert(price_object, limit);
                    }
                    Some(limit) => {
                        limit.add_order(order);
                    }
                }
            },
            BidOrAsk::Ask => {
                match self.asks_per_price.get_mut(&price_object) {
                    None => {
                        let mut limit = Limit::new(price_object);
                        limit.add_order(order);
                        self.asks_per_price.insert(price_object, limit);
                    }
                    Some(limit) => {
                        limit.add_order(order);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Hash, Copy, Clone)]
pub struct Price {
    integer: u64,
    fractional: u64,
    scalar: u64
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        // Two Prices are equal if both their integer and fractional parts are equal.
        self.integer == other.integer && self.fractional == other.fractional
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // We first compare the integer parts.
        // If they are equal, we compare the fractional parts.
        if self.integer < other.integer {
            Some(Ordering::Less)
        } else if self.integer > other.integer {
            Some(Ordering::Greater)
        } else if self.fractional < other.fractional {
            Some(Ordering::Less)
        } else if self.fractional > other.fractional {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for Price {}

impl Ord for Price {
    // Default ordering should be Equal
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Price {
    // fixed-point arithmetic to avoid precision issues with float
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integer = price as u64;
        let fractional = ((price % 1.0) * scalar as f64).round() as u64;
        Price {
            scalar,
            integer,
            fractional
        }
    }

    pub fn reconstruct(&mut self) -> f64 {
        self.integer as f64 + (self.fractional as f64 / self.scalar as f64)
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    // Limits created at a specific price level
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new()
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

impl PartialEq for Limit {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Limit {}

impl PartialOrd for Limit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl Ord for Limit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}


#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order {
            bid_or_ask,
            size
        }
    }
}
