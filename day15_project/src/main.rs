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
        println!("4. Analyze Scores");
        println!("5. Exit");
}

enum MenuChoice {
    AddScores,
    ViewScores,
    RemoveLastScore,
    AnalyzeScores,
    Exit,
}

fn parse_menu_selection(input: i32) -> Option<MenuChoice> {
    match input {
        1 => Some(MenuChoice::AddScores),
        2 => Some(MenuChoice::ViewScores),
        3 => Some(MenuChoice::RemoveLastScore),
        4 => Some(MenuChoice::AnalyzeScores),
        5 => Some(MenuChoice::Exit),
        _ => None, // Meaning all i32 inputs become invalid here.
    }
}

fn get_user_input() -> MenuChoice {
    loop {
        let menu_selection = read_input("Greetings! What do you want to do next? Choose from 1 to 5: ");

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

#[derive(Debug)]
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

#[derive(Debug)]
struct ScoreStats {
    count: usize,
    average: f64,
    maximum: i32,
    minimum: i32,
}

impl ScoreStats {
    fn analyzer(scores: &[Mark]) -> Self {
        let count = scores.len();

        /*if count == 0 {
            println!("No scores to analyze. Enter some scores first!");
            break;
        }*/

        let sum: i32 = scores.iter().map(|m| m.score).sum();
        let average = sum as f64 / count as f64;
        let maximum = scores.iter().map(|m| m.score).max().unwrap_or(0);
        let minimum = scores.iter().map(|m| m.score).min().unwrap_or(0);

        Self { count, average, maximum, minimum }
    }

    fn score_stats_display(&self) {
        println!("Total scores counted: {}", self.count);
        println!("Average score: {:.1}", self.average);
        println!("Maximum score: {}", self.maximum);
        println!("Minimum score: {}", self.minimum);
    }
}

// Student score entry app
// let mut scores: Vec<Mark> = Vec::new();

#[derive(Debug)]
struct App {
    scores: Vec<Mark>,
}

impl App {
    fn new() -> Self {
        Self {
            scores: Vec::new(),
        }
    }

    // program instance initializer / session activator
    fn run(&mut self) {
        loop {
            //  Display of Main Menu
            menu_display();

            // User choice handler.
            let choice: MenuChoice = get_user_input();

            // choice-based actions
            match choice {
                MenuChoice::AddScores => self.add_score(),
                MenuChoice::ViewScores => self.view_scores(),
                MenuChoice::RemoveLastScore => self.remove_last_score(),
                MenuChoice::AnalyzeScores => self.analyze_scores(),
                MenuChoice::Exit => {
                    println!("Successfully exited...");
                    break;
                }
            }
        }
    }

    // Major inner actions for the program
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

            // Retrieve last score added and total scores added so far
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

    fn view_scores(&self) {
        loop {
            if self.scores.is_empty() {
                println!("No scores found. Choose 1 to enter some scores first!");
                return;
            }

            // view the entire scores vector
            println!("Current scores: {:?}", self.scores);

            // view individual scores
            for mark in &self.scores {
                println!("{}", mark.score);

            }

            // exit to main main menu
            let trimmed = read_input("Type 'exit' to exit to main menu: ");

            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Back to main menu...");
                break;
            }
        }
    }

    fn remove_last_score(&mut self) {
        loop {
            match self.scores.pop() {
                Some(last_mark) => {
                    println!("Last score removed: {}", last_mark.score);
                    println!("Remaining marks: {:?}", self.scores);
                },
                None => println!("No scores to remove. Enter some scores to continue!"),
            }

            // exit to main main menu
            let trimmed = read_input("Type 'exit' to exit to main menu: ");

            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Back to main menu...");
                break;
            }
        }
    }

    fn analyze_scores(&self) {
        //let count = self.scores.len();

        if self.scores.is_empty() {
            println!("No scores yet. Add some scores first!");
        } else {
            let stats = ScoreStats::analyzer(&self.scores);
            stats.score_stats_display();
        }

        read_input("Press enter to return to main menu...");
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}