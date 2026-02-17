use serde::{Serialize, Deserialize};
use serde_json::{json};
use wasm_bindgen::prelude::*;


#[derive(Clone, Serialize, Deserialize)]
struct Bills {
    hundreds: i32,
    fifties: i32,
    twenties: i32,
    tens: i32,
    fives: i32,
    ones: i32
}


#[derive(Clone, Serialize, Deserialize)]
struct Coins {
    quarters: i32,
    dimes: i32,
    nickels: i32,
    pennies: i32
}

fn get_bills(transaction: &Transaction) -> Bills {
    let mut current_count = transaction.given - transaction.cost;
    let mut bills: Bills = Bills { hundreds: 0, fifties: 0, twenties: 0, tens: 0, fives: 0, ones: 0 };

    while current_count.floor() > 0_f64 {
        if current_count >= 100_f64 {
            bills.hundreds = current_count as i32 / 100;
            current_count -= (100 * bills.hundreds) as f64;
        }
        if current_count >= 50_f64 {
            bills.fifties = current_count as i32 / 50;
            current_count -= (50 * bills.fifties) as f64;
        }
        if current_count >= 20_f64 {
            bills.twenties = current_count as i32 / 20;
            current_count -= (20 * bills.twenties) as f64;
        }
        if current_count >= 10_f64 {
            bills.tens = current_count as i32 / 10;
            current_count -= (10 * bills.tens) as f64;
        }
        if current_count >= 5_f64 {
            bills.fives = current_count as i32 / 5;
            current_count -= (5 * bills.fives) as f64;
        }
        if current_count >= 1_f64 {
            bills.ones = current_count as i32 / 1;
            current_count -= bills.ones as f64;
        }
    }

    bills
}

fn get_coins(transaction: &Transaction) -> Coins {
    let money_paid = transaction.given - transaction.cost;
    let cents_int_as_f64 = (money_paid - (money_paid.floor() as f64)) * 100_f64;

    let mut current_count = cents_int_as_f64.round() as i32;
    let mut coins: Coins = Coins { quarters: 0, dimes: 0, nickels: 0, pennies: 0 };

    while current_count > 0 {
        if current_count >= 25 {
            coins.quarters = current_count as i32 / 25;
            current_count -= 25 * coins.quarters;
        }
        if current_count >= 10 {
            coins.dimes = current_count as i32 / 10;
            current_count -= 10 * coins.dimes;
        }
        if current_count >= 5 {
            coins.nickels = current_count as i32 / 5;
            current_count -= 5 * coins.nickels;
        }
        if current_count >= 1 {
            coins.pennies = current_count as i32 / 1;
            current_count -= coins.pennies;
        }
    }

    coins
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    cost: f64,
    given: f64
}

#[wasm_bindgen]
impl Transaction { 
    #[wasm_bindgen(constructor)]
    pub fn new(cost: f64, given: f64) -> Transaction {
        Transaction { cost, given }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Change {
    bills: Bills,
    coins: Coins
}

#[wasm_bindgen]
pub fn calculate_change(data: &Transaction) -> String {
    let transaction = Transaction {
        cost: data.cost,
        given: data.given
    };

    let bills = get_bills(&transaction);
    let coins = get_coins(&transaction);

    let change = Change {
        bills,
        coins
    };

    json!(change).to_string()
}

