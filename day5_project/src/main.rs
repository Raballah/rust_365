// Day 5: Loops 
// Loops that return values, more Rust-like
// Count my money, break to return value, and then subtract to get balance.

fn main() {
    let mut money_counter = 0;

    let my_balance = loop {
        money_counter += 1;

        if money_counter == 1200 {
            break money_counter - 122;
        }
    };

    println!("My balance from the money availabe is {}", my_balance);
}