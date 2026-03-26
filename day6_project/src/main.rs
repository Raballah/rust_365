// Day 7 - Mini Project 1
// CLI Student Management Tool
// A real Rust CLI application combinng functions, loops, match/control flow, user input, data handling
// Goal: Build an interactive CLI tool, which accepts multiple student scores

use std::io;

fn is_valid(score: i32) -> bool {
    (0..=100).contains(&score)
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
        println!("\n=== Student Score Manager ===");

        println!("\n1. Add student score");
        println!("2. View all scores");
        println!("3. Analyze scores");
        println!("4. Exit");

        // 3. Menu Choice Handler - reprompts, iteration ends at valid choice
        let choice: i32 = loop {
            let mut menu_choice = String::new();
            println!("\nEnter your choice: ");
            
            io::stdin()
                .read_line(&mut menu_choice)
                .expect("Failed to read menu choice");
            
            match menu_choice.trim().parse() {
                Ok(num) => break num, // num becomes choice, loop ends.
                Err(_) => {
                    println!("Invalid Entry. Choice must be from 1-4!");
                    continue;
                }
            }
        };

        // 4. Choice-based Actions
        match choice {
            1 => {
                // Add Student Score
                loop {
                    // score entry validation
                    let mut input = String::new();

                    println!("\nEnter Student Score or 'exit' to Exit: ");

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    
                    let trimmed = input.trim().to_lowercase();

                    if trimmed == "exit" {
                        println!("Score Added. Returning to Menu...");
                        break; // Back to (Menu/Outer) loop
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
            },
            2 => {
                // View All Scores
                loop {
                    if scores.is_empty() {
                        println!("No score found!");
                        break;
                    }

                    for score in &scores {
                        println!("{:?}", score);
                    }

                    println!("Scores Collected so far: {:?}", scores);

                    let mut input = String::new();

                    println!("Type 'Exit' to exit: ");

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input!");
                    
                    let trimmed = input.trim().to_lowercase();

                    if trimmed == "exit" {
                        println!("Exiting...Back to Main Menu!");
                        break;
                    }
                    continue;
                }
            },
            3 => {
                // Analyze Scores
                    loop {
                        for &score in &scores {
                            let message = feedback(score);

                            println!("Score: {} | Feedback: {}", score, message);
                        }

                        let mut optional_input = String::new();

                        println!("Type 'Exit' to exit!");

                        io::stdin()
                            .read_line(&mut optional_input)
                            .expect("Failed to read optional input!");
                        
                        let trimmed2 = optional_input.trim().to_lowercase();

                        if trimmed2 == "exit" {
                            println!("Session Exited Successfully!");
                            break;
                        }
                        continue;
                    }
            },
            4 => {
                println!("Session Exited Successfully!");
                break; // Outer loop exited, program exited.
            },
            _ => {
                println!("Invalid Entry. Choice Must be 1, 2, 3, or 4!");
                continue;
            },
        }
    }
}