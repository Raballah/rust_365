// Day 6: Functions and Return Values
// Day 6 Mini Project: "Student Score Analyzer CLI"
// Building a simple CLI program, which takes a student's score.
// uses functions to determine the grade.

// 1. Grade Calculator

fn calculate_grade(score: i32) -> char {
    match score {
        w if w > 100 || w < 0 => 'I', // 'I' invalid score entered. reccheck entry.
        80..=100 => 'A',
        70..=79 => 'B',
        60..=69 => 'C',
        50..=59 => 'D',
        0..=49 => 'F',
        _ => 'I',
    }
}

// 2. Pass/Fail Checker

fn is_pass(score: i32) -> Option<bool> {  // is_pass is Verbose bool return.
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

// 4. Return tuple

fn analyze(score: i32) -> (char, Option<bool>, String) {
    (calculate_grade(score), is_pass(score), feedback(score))
}

fn main() {
    let score = 109;
    let passed = is_pass(score);

    let pass_determinant = match passed {
        Some(true) => "Pass",
        Some(false) => "Fail",
        None => "Invalid - Please recheck score entry!",
    };

    let grade = calculate_grade(score);
    let message = feedback(score);

    println!("\n--- Students' Grading and Pass Categorization---");

    println!("Grade: {} | Pass?: {} | Feedback: {}", grade, pass_determinant, message);

    let score_analysis = analyze(score);

    println!("\n--- Score Analysis Result as a Tuple---");

    println!("Score analysis: {:?}", score_analysis);
}