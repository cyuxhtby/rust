// use std::collections::VecDeque;
// Std. lib `Iterator` definition
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
//
// Any type that implements Iterator must define the associated type 'Item' and the `next` method
// The derive macro implements these traits for Trade:
// Debug: Allows printing with {:?}
// PartialEq: Allows comparison with ==
// Clone: Allows creating a deep copy of Trade
#[derive(Debug, PartialEq, Clone)]
struct Trade {
    id: u64,
    price: f64,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone)]
struct Intent {
    user_id: u64,
    acceptable_trades: Vec<Trade>,
}

struct IntentIter {
    intents: Vec<Intent>,
    intent_index: usize, // Unsigned integer native to the cpu pointer size
    trade_index: usize,
}

impl Iterator for IntentIter {
    type Item = Trade;
    fn next(&mut self) -> Option<Self::Item> {}
}

impl IntentIter {
    fn new(intents: Vec<Intent>) -> Self {
        IntentIter {
            intents,
            intent_index: 0,
            trade_index: 0,
        }
    }
}

fn trades_above_price(intents: Vec<Intent>, min_price: f64) -> Vec<Trade> {
    IntentIter::new(intents)
        .filter(|t| t.price > min_price)
        .collect()
}

fn main() {}