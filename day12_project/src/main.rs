struct Student {
        name: String,
        score: i32,
        passed: bool,
    }

fn score_status(learner: &Student) {
    println!("Student's name: {}", learner.name);
    println!("Student's score: {}", learner.score);
    println!("Student's pass status: {}", learner.passed);
}

fn main() {
    let student = Student {
        name: String::from("Alice"), // name Owns the 'Alice' value
        score: 85,
        passed: true,
    };

    score_status(&student);
}