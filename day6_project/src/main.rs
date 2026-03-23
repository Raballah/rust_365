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
        let mut input = String::new();

        println!("Enter Student Score or 'exit' to Exit: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let small_case = input.trim();
        let trimmed = small_case.to_lowercase();

        if trimmed == "exit" {
            println!("Session Exited Successfully!");
            break;
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

        let message = feedback(score);

        println!("\n--- Score and Feedback System ---");

        println!("Score: {} | Feedback: {}", score, message);
    }
}