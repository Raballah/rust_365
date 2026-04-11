/* struct Student {
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
    let student1 = Student {
        name: String::from("Alice"), // name Owns the 'Alice' value
        score: 85,
        passed: true,
    };

    score_status(&student1);

    let student2 = Student {
        name: String::from("Mary"),
        ..student1
    };

    score_status(&student2);
} 
*/
// Struct method syntax impl

struct Student {
    name: String,
    score: i32,
}

// Associated function, say for creating a new student's data

impl Student {
    fn new(name: &str, score: i32) -> Student {
        Student {
            name: name.to_string(),
            score,
        }
    }
}

fn score_display(learner: &Student) {
    println!("Name: {}", learner.name);
    println!("Score: {}", learner.score);
}
fn main() {
    let student2 = Student::new("James", 85);

    score_display(&student2);
}