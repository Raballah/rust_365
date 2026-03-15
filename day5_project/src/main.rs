// Day 5: Loops 
// While loop
// Simple command-line tool. Reading from a buffer.

use std::io;

fn main() {
    let mut user_input = String::new();

    while user_input.trim() != "exit" {
        user_input.clear(); // Clear existing, previous entry
        println!("Type something to proceed or 'exit' to quit!");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read any input from the line!");
        
        println!("Your Entry: {}", user_input);
    }

    println!("Exited successfully!");
}