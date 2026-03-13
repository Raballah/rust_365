// Day 4: Lessons Regarding Control Flow (if, else, else if, match)
// Student Grade Evaluator Mini Task

fn main() {

    // if else
    let score = 78;

    if score >= 90 && score <=100 {
        println!("Grade: A");
    } else if score >=80 && score <=89 {
        println!("Grade: B");
    } else if score >=70 && score <=79 {
        println!("Grade: C");
    } else if score >=60 && score <=69 {
        println!("Grade: D");
    } else if score < 60 {
        println!("Grade: F");
    } else {
        println!("Grade: Unknown");
    }

    let pass_status = if score >= 60 {
        "Passed"
    } else {
        "Failed"
    };

    println!("\n --Based on the score, this studet has {}.--", pass_status);


    // match 

    let mark = 78; 

    let grade = match mark {
        w if w > 100 => "Grade: Unknown",
        90..=100 => "Grade: A",
        80..=89 => "Grade: B",
        70..=79 => "Grade: C",
        60..=69 => "Grade: D",
        w if w < 60 => "Grade: F",
        _ => "Grade: Uncategorized",
    };

    let pass_determination = if mark >= 60 {
        "Passed"
    } else {
        "Failed"
    };

    println!("\n The grade of this student is {}.", grade);
    println!("\n --The mark indicates that this student has {}.--", pass_determination);
}