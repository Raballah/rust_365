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

fn menu_display() {
    println!("\n=== Score Tracker ===");

        println!("\n1. Add score");
        println!("2. View all scores");
}

struct Mark {
    score: i32,
}

impl Mark {
    fn new(score: i32) -> Option<Self> {
        if (0..=100).contains(&score) {
            Some( Self { score })
        } else {
            None
        }
    }
}

// Student score entry app
// let mut scores: Vec<Mark> = Vec::new();
// loop {
// show
//}
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
        // add student
        self.add_score();

        // show main menu
        menu_display();

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
                Err(_) => {
                    println!("Invalid entry. Score must be a number, from 0 to 100");
                    continue;
                }
            };

            match Mark::new(score) {
                Some(mark) => {
                    self.scores.push(mark);
                    println!("Score added!");
                },
                None => println!("Invalid. Enter non-negative numbers, from 0 to 100!"),
            }
            // Last score added and total scores added so far.
            match self.scores.last() {
                Some(last_mark) => println!(
                    "Last score added: {} Total scores: {}", 
                    last_mark.score, 
                    self.scores.len()
                ),
                None => println!("No score added so far!"),
            }
        }
    }
}

fn main() {
    App::new().run();
}