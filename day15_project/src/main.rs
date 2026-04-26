// Day 15 Mini-Project: Building a CLI Score Tracker

use std::io;
use std::fmt;

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    
    input.trim().to_string()
}

struct Mark {
    score: i32,
}

impl Mark {
    fn new(score: i32) -> Self {
        Self { score }
    }
}

// Student score entry app

struct App {
    scores: Vec<Mark>,
}

impl App {
    fn new() -> Self {
        Self {
            scores: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.add_score();
    }

    fn add_score(&mut self) {
        loop {
            let trimmed = read_input("Enter score or type 'exit' to Exit: ");

            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Exited session successfully...");
                break; // Back to outer scope of the loop
            }

            let score: i32 = match trimmed.parse::<i32>() {
                Ok(num) => num,
                Err(_) =>{
                    println!("Invalid entry. Enter numbers from 0 to 100");
                    continue;
                }
            }; 
        }
    }
}

fn main() {
    App::new().run();
}