// Day 5: Loops 
// While loop
// Simple command line tool using while loop.

use std::io;

fn main() {
    let mut user_data = String::new();

    while user_data.trim() != "exit" {
        user_data.clear(); // clears the previous input
        println!("Type something to proceed (or 'exit' to quit):");

        io::stdin()
            .read_line(&mut user_data)
            .expect("Failed to read line!");

        println!("You typed: {}", user_data);
    }

    println!("Exited successfully!");
}