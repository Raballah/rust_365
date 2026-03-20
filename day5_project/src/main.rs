// Day 6: Functions and Return Values
// Day 6 Mini Project: "Student Score Analyzer CLI"
// Building a simple CLI program, which takes a student's score.
// uses functions to determine the grade.

fn score_grader(score: i32) -> char {
    match score {
        w if w > 100 => 'E',
        80..=100 => 'A',
        70..=79 => 'B',
        60..=69 => 'C',
        50..=59 => 'D',
        40..=49 => 'E',
        _ => 'F',
    }
}

fn pass_determinant(mark: i32) -> &'static str {
    if mark >= 50 {
        "Pass"
    } else {
        "Fail"
    }
}

fn main() {
    let student_score = 43;

    let grade = score_grader(student_score);
    let pass_level = pass_determinant(student_score);

    println!("\n--- Students' Grading and Pass Categorization---");

    print!("Grade: {}", grade);
    println!(" | Pass Level: {}", pass_level);
}
