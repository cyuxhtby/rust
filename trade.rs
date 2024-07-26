use std::io::{self, Write};

// Trade struct 
#[derive(Debug, Clone)]
pub struct Trade {
    pub id: u32,
    pub order_type: OrderType,
    pub sell_token: String,
    pub buy_token: String,
    pub sell_amount: u64,
    pub buy_amount: u64,
}

// Order type enum
#[derive(Debug, Clone)]
pub enum OrderType {
    Buy,
    Sell,
}

// Trade impl
impl Trade {
    pub fn new(id: u32, order_type: OrderType, sell_token: String, buy_token: String, sell_amount: u64, buy_amount: u64 ) -> Self {
        Trade { id, order_type, sell_token, buy_token, sell_amount, buy_amount }
    }

    pub fn process(&self) {
        match self.order_type {
            OrderType::Buy => {
                println!("Temp: Processing buy order: {:?}", self);
            }
            OrderType::Sell => {
                println!("Temp: Processing sell order: {:?}", self);
            }
        }  
    }
}

fn main() {
    let mut trades = Vec::new();
    let mut id = 1;

    loop {
        let mut input = String::new();

        println!("Enter trade type (buy/sell) or type 'done' to finish:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        if input == "done" {
            break;
        }

        let order_type = match input.as_str() {
            "buy" => OrderType::Buy,
            "sell" => OrderType::Sell,
            _ => {
                println!("Invalid order type");
                continue;
            }
        };

        let mut sell_token = String::new();
        println!("Enter sell token:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut sell_token).unwrap();
        let sell_token = sell_token.trim().to_string();
        println!("Sell token: {}", sell_token);

        let mut buy_token = String::new();
        println!("Enter buy token:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buy_token).unwrap();
        let buy_token = buy_token.trim().to_string();
        println!("Buy token: {}", buy_token);


        let mut sell_amount = String::new();
        println!("Enter sell amount in USD:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut sell_amount).unwrap();
        let sell_amount: u64 = sell_amount.trim().parse().unwrap_or(0);
        println!("Sell amount: {}", sell_amount);


        let mut buy_amount = String::new();
        println!("Enter buy amount in USD:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buy_amount).unwrap();
        let buy_amount: u64 = buy_amount.trim().parse().unwrap_or(0);
        println!("Buy amount: {}", buy_amount);


        trades.push(Trade::new(id, order_type, sell_token, buy_token, sell_amount, buy_amount));
        id += 1;
    }

    for trade in trades {
        trade.process();
    }
}