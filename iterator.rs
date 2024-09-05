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
    block: u64,
}

#[derive(Debug, PartialEq, Clone)]
struct Intent {
    user_id: u64,
    acceptable_trades: Vec<Trade>,
}

// Defines the data stucture to be traversed and the pointers to be used
struct IntentIter {
    intents: Vec<Intent>,
    intent_index: usize, // Unsigned integer w pointer size naitive to cpu
    trade_index: usize,
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

impl Iterator for IntentIter {
    type Item = Trade;

    fn next(&mut self) -> Option<Self::Item> {
        while self.intent_index < self.intents.len() {
            let intent = &self.intents[self.intent_index];
            if self.trade_index < intent.acceptable_trades.len() {
                let trade = intent.acceptable_trades[self.trade_index].clone();
                self.trade_index += 1;
                return Some(trade);
            }
            self.intent_index += 1;
            self.trade_index = 0;
        }
        None
    }
}


fn trades_above_price(intents: Vec<Intent>, min_price: f64) -> Vec<Trade> {
    IntentIter::new(intents)
        .filter(|t| t.price > min_price)
        .collect()
}

fn main() {
    let intents = vec![
        Intent {
            user_id: 0,
            acceptable_trades: vec![
                Trade {id: 49, price: 100.01, block: 20681603},
                Trade {id: 49, price: 98.02, block: 20681604},
            ]
        },
        Intent {
            user_id: 1,
            acceptable_trades: vec![
                Trade {id: 50, price: 100.05, block: 20681603},
                Trade {id: 50, price: 99.8, block: 20681604},
            ]
        }
    ];

    let intent_iter = IntentIter::new(intents.clone());

    for trade in intent_iter {
        println!("Trade {:?}", trade);
    }
    
    println!("\nTrades over 100");
    let trades_over_100 = trades_above_price(intents.clone(), 100.00);
    for trade in trades_over_100 {
        println!("Trade {:?}", trade);
    }
}