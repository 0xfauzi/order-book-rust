mod matching_engine;


use matching_engine::orderbook::Order;
use crate::matching_engine::engine::{MatchingEngine, TradingPair};
use crate::matching_engine::orderbook::BidOrAsk::Bid;

fn main() {

    let buy_order_1 = Order::new(Bid, 20.5);
    // let buy_order_2 = Order::new(Bid, 20.5);
    // let sell_order_1 = Order::new(Ask, 24.1);
    // let sell_order_2 = Order::new(Ask, 300.12);
    // let mut orderbook = OrderBook::new();
    // orderbook.add_order(20.5, buy_order_1);
    // orderbook.add_order(20.5, buy_order_2);
    // orderbook.add_order(24.1, sell_order_1);
    // orderbook.add_order(24.12, sell_order_2);
    // println!("{:?}", orderbook);

    let mut matching_engine = MatchingEngine::new();
    let trading_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    // matching_engine.add_new_market(trading_pair);
    matching_engine.place_limit_order(trading_pair, 22.1, buy_order_1).unwrap();
    println!("{:?}", matching_engine);
}
