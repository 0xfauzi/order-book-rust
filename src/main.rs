use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use crate::BidOrAsk::{Ask, Bid};

#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
struct OrderBook {
    bids_per_price: HashMap<Price, Limit>, // for O(1) retrieval of limits
    asks_per_price: HashMap<Price, Limit>, //for O(1) retrieval of limits
    bids: BinaryHeap<Reverse<Limit>>, //Min heap, lowest price first
    asks: BinaryHeap<Limit> //Max heap, highest price first
}


impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            bids_per_price: HashMap::new(),
            asks_per_price: HashMap::new(),
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new()
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            Bid => {
                let price_object = Price::new(price);
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
            }
            Ask => {

            }
        }
    }
}

#[derive(Debug, Hash, Copy, Clone)]
struct Price {
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
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integer = price as u64;
        let fractional = ((price % 1.0) * scalar as f64).round() as u64;
        Price {
            scalar,
            integer,
            fractional
        }
    }

    fn reconstruct(&mut self) -> f64 {
        self.integer as f64 + (self.fractional as f64 / self.scalar as f64)
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    // Limits created at a specific price level
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new()
        }
    }

    fn add_order(&mut self, order: Order) {
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
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order {
            bid_or_ask,
            size
        }
    }
}

fn main() {

    let buy_order_1 = Order::new(Bid, 20.5);
    let buy_order_2 = Order::new(Bid, 20.5);
    let sell_order_1 = Order::new(Ask, 24.1);
    let sell_order_2 = Order::new(Ask, 300.12);
    let mut orderbook = OrderBook::new();
    orderbook.add_order(20.5, buy_order_1);
    orderbook.add_order(20.5, buy_order_2);
    orderbook.add_order(24.1, sell_order_1);
    orderbook.add_order(24.12, sell_order_2);
    println!("{:?}", orderbook);
}
