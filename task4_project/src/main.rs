// Day 4 Mini Project: Student Score Analyzer

// fn for displaying all scores
fn scores_total(score_board: &[i32]) {

    for score in score_board.iter() {
        println!("Part of the scores: {}", score);
    }
    println!("Total number of score entries/elements: {}", score_board.len());
}

// fn for displaying the highest score
fn highest_mark(favourite_score: &[i32]) -> Option<i32> {

    if favourite_score.is_empty() {
        return None;
    }

    let mut best_performer = favourite_score[0];
    for &score in favourite_score.iter() {
        if score > best_performer {
            best_performer = score;
        }
    }
    Some(best_performer)
}
// fn to return the lowest score 

fn worst_student(poor_performer: &[i32]) -> Option<i32> {
    if poor_performer.is_empty() {
        return None;
    }

    let mut worst_performer = poor_performer[0];

    for &score in poor_performer.iter() {
        if score < worst_performer {
            worst_performer = score;
        }
    }
    Some(worst_performer)
}

fn main() {
    let student_scores = [89, 38, 50, 60, 47];

    // All Scores
    println!("\n---All Scores---");
    scores_total(&student_scores);

    // Highest Scores From the List
    println!("\n---Best Score out of the List---");
    match highest_mark(&student_scores) {
        Some(score) => println!("The best score of all scores is: {}", score),
        None => println!("No scores provided!"),
    }

    // Lowest scores from the list 
    println!("\n---Worst Score out of the List---");
    match worst_student(&student_scores) {
        Some(mark) => println!("The worst performing score is: {}", mark),
        None => println!("No scores to compare!"),
    }  

    // The List of the First Three Scores
    println!("\n---First Three Scores---");
    let first_three = &student_scores[..3];

    println!("These are the first three scores: {:?}", first_three);
}