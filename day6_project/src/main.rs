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
    loop {
        // 1. Menu System (Core Feature)
        println!("\n=== Student Score Manager ===");

        println!("\n1. Add student score");
        println!("2. View all scores");
        println!("3. Analyze scores");
        println!("4. Exit");

        // 2. Menu Choice Handler - reprompts, iteration ends at valid choice
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

        // 3. Choice-based Actions
        match choice {
            1 => {
                // Handling user iput and feedback
                loop {
                    // Score Entry Validation
                    let mut input = String::new();

                    println!("\nEnter Student Score or 'exit' to Exit: ");

                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read input");
                    
                    let trimmed = input.trim().to_lowercase();

                    if trimmed == "exit" {
                        println!("Returning to Menu...");
                        break; // Back to outer (Menu) loop
                    }

                    let score: i32 = match trimmed.parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid Entry. Score Must be From 0-100.");
                            continue;
                        }
                    };

                    if !is_valid(score) {
                        println!("Invalid Entry. Score Must be From 0-100.");
                        continue;
                    }

                    // Check if feedback is required on demand
                    loop {
                        let mut optional_input = String::new();

                        println!("Feedback required? (Type 'No' or 'Yes'): ");

                        io::stdin()
                            .read_line(&mut optional_input)
                            .expect("Failed to read optional input!");
                        
                        let trimmed2 = optional_input.trim().to_lowercase();

                        // 'No'/ 'Yes' are okay entries. otherwise, back to optional_input prompt
                        match trimmed2.as_str() {
                            "no" => {
                                println!("\n--- Student Score System ---");
                                println!("Student Score: {}", score);
                                break; // Iteration loop ends and begins afresh
                            },
                            "yes" => {
                                let message = feedback(score);
                                
                                println!("\n--- Score and Feedback System ---");
                                
                                println!("Score: {} | Feedback: {}", score, message);
                                break;                                
                            },
                            _ => {
                                println!("Invalid Entry. Please Enter 'No' or 'Yes'!");
                                continue;
                            },
                        }
                    }
                }
            },
            2 => {

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