// Day 15 Mini-Project: Building a CLI Score Tracker

use std::io;

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

        println!("\n1. Add Score");
        println!("2. View Scores");
        println!("3. Remove Last Score");
        println!("4. Show Average");
        println!("5. Exit");
}

enum MenuChoice {
    AddScores,
    ViewScores,
    RemoveLast,
    ShowAverage,
    Exit,
}

fn parse_menu_selection(input: i32) -> Option<MenuChoice> {
    match input {
        1 => Some(MenuChoice::AddScores),
        2 => Some(MenuChoice::ViewScores),
        3 => Some(MenuChoice::RemoveLast),
        4 => Some(MenuChoice::ShowAverage),
        5 => Some(MenuChoice::Exit),
        _ => None, // Meaning all i32 inputs become invalid here.
    }
}

fn get_user_input() -> MenuChoice {
    loop {
        let menu_selection = read_input("Enter 1 to 5 to choose action: ");

        if let Ok(num) = menu_selection.parse::<i32>() {
            if let Some(choice) = parse_menu_selection(num) {
                break choice;
            }
            // If parsed failed or selection not a choice 
            println!("Invalid Entry. Enter a number from 1 to 5!");
            continue;
        }
    }
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
// menu_display();
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
        // display main menu
        menu_display();
        // add student
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
    let mut app = App::new();
    app.run();
}