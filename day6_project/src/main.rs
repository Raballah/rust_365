// Day 7 - Mini Project 1
// CLI Student Management Tool
// A real Rust CLI application combinng functions, loops, match/control flow, user input, data handling
// Goal: Build an interactive CLI tool, which accepts multiple student scores

use std::io;

fn show_menu() {
    println!("\n=== Student Score Manager ===");

        println!("\n1. Add student score");
        println!("2. View all scores");
        println!("3. Analyze scores");
        println!("4. Exit");
}

enum MenuOption {
    AddScore,
    ViewScores,
    AnalyzeScores,
    Exit,
}

fn parse_menu_choice(input: i32) -> Option<MenuOption>{
    match input {
        1 => Some(MenuOption::AddScore),
        2 => Some(MenuOption::ViewScores),
        3 => Some(MenuOption::AnalyzeScores),
        4 => Some(MenuOption::Exit),
        _ => None, // All i32 inputs are invalid
    }
}

fn get_user_input() -> MenuOption {
    loop {
        let mut menu_choice = String::new();
        println!("\nEnter your choice: ");
        
        io::stdin()
            .read_line(&mut menu_choice)
            .expect("Failed to read menu choice");
        
        // parse to i32, convert to MenuOption
        if let Ok(num) = menu_choice.trim().parse::<i32>() {
            if let Some(option) = parse_menu_choice(num) {
                break option; // Valid MenuOption, exit loop
            }
        }
        // if parse failed or number number not 1-4
        println!("Invalid Entry. Choice must be from 1-4!");
        continue;
    }
}

fn is_valid(score: i32) -> bool {
    (0..=100).contains(&score)
}

fn add_score(scores: &mut Vec<i32>) {  // Modifies the scores mut vector, borrowed here.
    // Add student score
    loop {
        // score entry validation
        let mut input = String::new();

        println!("\nEnter Student Score or 'exit' to Exit: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
        
        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Score(s) Added. Returning to Menu...");
            break; // Back to (Menu/Outer) loop.
        }
        
        let score: i32 = match trimmed.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Entry. Score Must be From 0-100.");
                continue;
            }
         };

         if !is_valid(score) {
            println!("Invalid. Enter 0-100. No Negative Entries.");
            continue;
         } else {
            scores.push(score);
            println!("Score {} added. Scores added so far: {}", score, scores.len());
         }
    }
}

fn view_scores(scores: &[i32]) {  // Borrows scores &Vec<i32>, displays as i32
    //View All Scores
    loop {
        if scores.is_empty() {
            println!("No score found! Select 1 and add some scores first!");
            return;
        }
        
        println!("\n--- All Scores ---");

        for score in scores {
            println!("{}", score);
        }
        
        println!("\nScores Count: {}", scores.len());

        // average score calculation
        let sum: i32 = scores.iter().sum();

        let average_score: f64 = sum as f64 / scores.len() as f64;

        println!("Average Score: {:.0}", average_score);

        // highest score determinant
        if let Some(highest_score) = scores.iter().max() {
            println!("Highest Score: {}", highest_score);
        }

        // lowest score determinant
        if let Some(lowest_score) = scores.iter().min() {
            println!("Lowest Score: {}", lowest_score);    
        }

        // Input for 'Exit' to exit loop
        let mut input = String::new();
        
        println!("Type 'Exit' to exit: ");
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
                    
        let trimmed = input.trim();
        
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Exiting...Back to Main Menu!");
            break;
        }
        continue;
    }
}

fn is_pass(score: i32) -> bool {
    score >= 60
}

fn analyze_scores(scores: &[i32]) {
    // Analyze Scores
    loop {
        if scores.is_empty() {
            println!("No scores found!. Select 1 and enter some scores to analyze!");
            return;
        }
        
        println!("\n--- Score Analysis ---\n");
        
        for score in scores {
            let grade = calculate_grade(*score);
            let label = if is_pass(*score) {"Yes"} else {"No"};
            let message = feedback(*score);
            
            println!("Score: {} | Grade: {} | Pass?: {} | Comment: {}", score, grade, label, message);
        }

        println!("\n--Pass/Fail Overview--\n");
        let mut pass_count: i32 = 0;
        let mut fail_count: i32 = 0;

        for &score in scores {
            if is_pass(score) == true {
                pass_count += 1;
            } else {
                fail_count += 1;
            }
        }

        println!("Pass Count: {}", pass_count);
        println!("Fails Count: {}", fail_count);

        // Inpt for 'Exit' to exit loop
        let mut optional_input = String::new();
        
        println!("Type 'Exit' to exit!");
        
        io::stdin()
            .read_line(&mut optional_input)
            .expect("Failed to read optional input!");
        
            let trimmed2 = optional_input.trim();
            
            if trimmed2.eq_ignore_ascii_case("exit") {
                println!("Session Exited Successfully!");
                break;
            }
            continue;
    }
}

fn calculate_grade(score: i32) -> char {
    match score {
        w if w > 100 || w < 0 => 'I', // Invalid Entry/Score
        80..=100 => 'A',
        70..=79 => 'B',
        60..=69 => 'C',
        50..=59 => 'D',
        0..=49 => 'F',
        _ => 'I', // Invalid Entry/Score
    }
}

fn feedback(score: i32) -> &'static str {
    match score {
        w if w > 100 || w < 0 => "Invalid Score. Score Must be From 0-100.",
        80..=100 => "Excellent Work!",
        70..=79 => "Good Work. Keep Improving!",
        60..=69 => "Good Trial, but Needs Improvement!",
        50..=59 => "Fair Trial. Work Harder Next Time!",
        0..=49 => "Below Average. See Me!",
        _ => "Invalid Score",
    }
}

fn main() {
    // 1. Student Score Collector / Vector
    let mut scores: Vec<i32> = Vec::new();
    
    loop {
        // 2. Menu System (Core Feature)
        show_menu();

        // 3. Menu Choice Handler - reprompts, iteration ends at valid choice
        let choice: MenuOption = get_user_input();

        // 4. Choice-based Actions
        match choice {
            MenuOption::AddScore => add_score(&mut scores),
            MenuOption::ViewScores => view_scores(&scores),
            MenuOption::AnalyzeScores => analyze_scores(&scores),
            MenuOption::Exit => {
                println!("Session Exited Successfully! Goodbye!");
                break; // Outer loop exited, program exited.
            }
        }
    }
}