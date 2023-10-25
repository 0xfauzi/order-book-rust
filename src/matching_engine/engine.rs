use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::matching_engine::orderbook::{Order, OrderBook};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TradingPair {
    // BTC ==> USD
    // base ==> quote
    base: String,
    quote: String,
}

impl Display for TradingPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.base, self.quote)
    }
}
impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {
            base,
            quote
        }
    }
}

#[derive(Debug)]
pub struct MatchingEngine {
    orderbook: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbook: HashMap::new()
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbook.insert(pair, OrderBook::new());
    }

    pub fn place_limit_order(&mut self, trading_pair: TradingPair, price: f64, order: Order) -> Result<(), String> {
        match self.orderbook.get_mut(&trading_pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                Ok(())
            }
            None => Err(format!("{} does not exist", trading_pair.to_string())),
        }
    }
}

