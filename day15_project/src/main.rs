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
    AddScore,
    ViewScores,
    RemoveLastScore,
    AnalyzeScores,
    Exit,
}

fn parse_menu_selection(input: i32) -> Option<MenuChoice> {
    match input {
        1 => Some(MenuChoice::AddScore),
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

#[derive(Debug, Clone)]
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

// Needed for use by sort_by_key/ Clone necessary to clone Vec<Mark>
#[derive(Debug, Clone)]
struct ScoreStats {
    count: usize,
    average: f64,
    maximum: i32,
    minimum: i32,
}

impl ScoreStats {
    fn analyzer(scores: &[Mark]) -> Self {
        let count = scores.len();
        let sum: i32 = scores.iter().map(|m| m.score).sum();
        let average = if count > 0 {
            sum as f64 / count as f64
        } else {
            0.00
        };
        let maximum = scores.iter().map(|m| m.score).max().unwrap_or(0);
        let minimum = scores.iter().map(|m| m.score).min().unwrap_or(0);

        Self { count, average, maximum, minimum }
    }

    fn score_stats_display(&self, scores: &[Mark]) {
        println!("Total scores counted: {}", self.count);
        println!("Average score: {:.1}", self.average);
        println!("Maximum score: {}", self.maximum);
        println!("Minimum score: {}", self.minimum);

        // Vec<&Mark> build temporarily, sorted and dropped on display
        let mut sorted: Vec<&Mark> = scores.iter().collect();
        sorted.sort_by_key(|m| m.score);

        println!("\nSorted scores (ascending) >>:");
        for (i, mark) in sorted.iter().enumerate() {
            println!(" {}: {}", i + 1, mark.score);  
            }
        // sorted dropped here.
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
                MenuChoice::AddScore => self.add_score(),
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
        if self.scores.is_empty() {
            println!("No scores found. Choose 1 to enter some scores first!");
        }

        // View the entire app / scores vector
        for (i, mark) in self.scores.iter().enumerate() {
            println!("{}: {}", i + 1, mark.score);
        }

        // Exit
        read_input("Press Enter key to return to main menu...");
    }

    fn remove_last_score(&mut self) {
        match self.scores.pop() {
            Some(last_mark) => {
                println!("Last score removed: {}", last_mark.score);

                println!("= Remaining scores = ");
                for (i, mark) in self.scores.iter().enumerate() {
                    println!("{}: {}", i + 1, mark.score);
                }
            },
            None => println!("No scores to remove. Enter some scores to continue!"),
        }
    }

    fn analyze_scores(&self) {
        if self.scores.is_empty() {
            println!("No scores yet. Add some scores first!");
        } else {
            let stats = ScoreStats::analyzer(&self.scores);
            stats.score_stats_display(&self.scores);
        }

        read_input("Press enter to return to main menu...");
    }
}

fn main() {
    let mut app = App::new();
    app.run();
    // or simply App::new().run();
}