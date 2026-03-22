// Day 6: Functions and Return Values
// Day 6 Mini Project: "Student Score Analyzer CLI"
// Building a simple CLI program, which takes a student's score.
// uses functions to determine the grade.

// 1. Grade Calculator
fn calculate_grade(score: i32) -> char {
    match score {
        w if w > 100 || w < 0 => 'I', // Invalid. Recheck Score Entry!
        80..=100 => 'A',
        70..=70 => 'B',
        60..=69 => 'C',
        50..=59 => 'D',
        0..=49 => 'F',
        _ => 'I', // Covers all possible entry scenarios
    }
}

// 2. Pass/Fail Checker
fn is_pass(score: i32) -> Option<bool> {
    if !(0..=100).contains(&score) {
        return None;
    }
    Some(score >= 50)
}

// 3. Feedback Generator

fn feedback(score: i32) -> String {
    match score {
        w if w > 100 || w < 0 => String::from("Invalid score: Recheck score entry!"),
        80..=100 => String::from("Excellent work!"),
        70..=79 => String::from("Almost there. Keep working harder!"),
        60..=69 => String::from("Good job. Keep improving!"),
        50..=59 => String::from("Fair trial. Keep improving!"),
        0..=49 => String::from("Needs improvement!"),
        _ => String::from("Invalid score: Recheck score entry!"),
    }
}

// 4. Return Score Analysis as a Tuple

fn analyze(score: i32) -> (&'static str, char, String) {
    let pass_status = is_pass(score);
    let pass_label = match pass_status {
        Some(true) => "Pass",
        Some(false) => "Fail",
        None => "Invalid! Please Recheck Score Entry!",
    };

    (pass_label, calculate_grade(score), feedback(score))
}

// 5. Main
use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter Student Score: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read student score!");
    
    let student_score: i32 = input
        .trim()
        .parse()
        .expect("Please Enter a Valid Student Score!");

    let pass_checker = is_pass(student_score);

    let pass_determinant = match pass_checker {
        Some(true) => "Pass",
        Some(false) => "Fail",
        None => "Invalid! Please Recheck Score Entry!",
    };

    let grade = calculate_grade(student_score);
    let message = feedback(student_score);
    
    // Option 1 Display: Inline Display
    println!("\n--- Students' Grading and Pass Categorization---");
    println!("Pass?: {}, Grade: {}, Feedback: {}", pass_determinant, grade, message);

    // Option 2 Display: Score analysis as a tuple
    let score_tuple = (pass_determinant, grade, message);

    println!("\n--- Score Analysis Result as a Tuple---");
    println!("Score Analysis: {:?}", score_tuple);

    // Option 3: Use of analyze() fn, but this brings me a Some(value) in display

    let score_analysis = analyze(student_score);
    println!("\n--- Score Analysis Using Analyze()---");
    println!("Score Analysis: {:?}", score_analysis);
}