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
        println!("2. Edit Score");
        println!("3. View Scores");
        println!("4. Remove Score");
        println!("5. Analyze Scores");
        println!("6. Exit");
}

enum MenuChoice {
    AddScore,
    EditScore,
    ViewScores,
    RemoveScore,
    AnalyzeScores,
    Exit,
}

fn parse_menu_selection(input: i32) -> Option<MenuChoice> {
    match input {
        1 => Some(MenuChoice::AddScore),
        2 => Some(MenuChoice::EditScore),
        3 => Some(MenuChoice::ViewScores),
        4 => Some(MenuChoice::RemoveScore),
        5 => Some(MenuChoice::AnalyzeScores),
        6 => Some(MenuChoice::Exit),
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
    fn display(&self, scores: &[Mark]) {
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
                MenuChoice::EditScore => self.edit_score(),
                MenuChoice::ViewScores => self.view_scores(),
                MenuChoice::RemoveScore => self.remove_by_index(),
                MenuChoice::AnalyzeScores => self.analyze_scores(),
                MenuChoice::Exit => {
                    println!("Successfully exited...");
                    break;
                }
            }
        }
    }

    // Service layer
    fn analyzer(&self) -> Option<ScoreStats> {
        if !self.scores.is_empty() {
            let count = self.scores.len();
            let sum: i32 = self.scores.iter().map(|m| m.score).sum();
            let average = if count > 0 {
                sum as f64 / count as f64
            } else {
                0.00
            };
            
            let maximum = self.scores.iter().map(|m| m.score).max().unwrap_or(0);
            let minimum = self.scores.iter().map(|m| m.score).min().unwrap_or(0);

            Some(ScoreStats { count, average, maximum, minimum })
        } else {
            None 
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
                    println!("Score {} added successfully!", score);
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

    fn edit_score(&mut self) {
        loop {
            if self.scores.is_empty() {
                println!("No scores yet. Add some scores first!");
                break;
            }

            // Currently available scores to edit
            for (i, mark) in self.scores.iter().enumerate() {
                println!(" {}: {}", i + 1, mark.score);
            }

            // Index-based choice of score to edit
            let trimmed = read_input("Enter index to edit or 'exit' to Exit: ");

            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Successfully exited to main menu...");
                break;
            }

            let typed_index: usize = match trimmed.parse::<usize>() {
                Ok(num) if num >= 1 => num,
                _ => {
                    println!("Invalid. Index must be a number, from 1 onwards!");
                    continue;
                }
            };
            
            // .get(i) to check if index within bounds, immutable borrow
            let programming_index = typed_index - 1;

            let score_value = match self.scores.get(programming_index) {
                Some(mark) => mark.score, // copies score: i32 directly
                None => {
                    println!("No score at this index. Enter an available index!");
                    continue;
                }
            };
            
            // Proceed to edit by entering a new score at the target index
            self.scores.get_mut(programming_index);
            println!("Current score at position {}: {}", typed_index, score_value);

            let trimmed2 = read_input("Enter new score or 'exit' to Exit: ");

            if trimmed2.eq_ignore_ascii_case("exit") {
                println!("Successfully exited to main menu...");
                break;
            }
            println!("To be populated with logic");
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

    fn remove_by_index(&mut self) {
        loop {
            if self.scores.is_empty() {
                println!("No scores found. Add some scores first!");
                break;
            }

            println!("-- Remove Score by Index --");

            // Every score and its index currently 
            println!("\n- Available index-score currently -");
            for (i, mark) in self.scores.iter().enumerate() {
                println!(" {}: {}", i + 1, mark.score);
            }

            let trimmed = read_input("Enter index to remove score or 'exit' to Exit: ");

            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Exited successfully...");
                break; // Back to outer scope of the loop
            }

            let display_index: usize = match trimmed.parse::<usize>() {
                Ok(num) if num >= 1 => num,
                _ => { // Not Err(_) because 0 can be parsed as Ok(0), hence _ catches all 3 options
                    println!("Invalid. Enter a number from 1 onwards!");
                    continue;
                }
            };

            let actual_index = display_index - 1; // Index converted to 0-based.

            // Immutable borrow with .get() use to get score first
            let score_value = match self.scores.get(actual_index) {
                Some(mark) => mark.score, // copies i32, ends borrow
                None => {
                    println!("No score at index {}. Choose an available index!", display_index);
                    continue;
                }
            };  // immutable borrow with .get() ends here.

            // .remove(!) T with mutable borrow now to remove the score
            self.scores.remove(actual_index);
            println!("Score {} at index {} removed!", score_value, display_index);

            println!("= Remaining scores = ");
            if self.scores.is_empty() {
                println!(" No scores remaining.");
            } else {
                for (i, mark) in self.scores.iter().enumerate() {
                    println!(" {}: {}", i + 1, mark.score);
                }
            }
        }
    }

    fn analyze_scores(&self) {
        match self.analyzer() {
            Some(stats) => stats.display(&self.scores),
            None => println!("No scores yet. Add scores first!")
        }
        
        read_input("Press enter to return to main menu...");
    }
}

fn main() {
    let mut app = App::new();
    app.run();
    // or simply App::new().run();
}