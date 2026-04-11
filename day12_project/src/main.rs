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

impl Student {
    fn is_pass(&self) -> bool {
        self.score >= 50
    }

    fn grade(&self) -> char {
        match self.score {
            w if w > 100 || w < 0 => 'I', // Invalid score
            80..=100 => 'A',
            70..=79 => 'B',
            60..=69 => 'C',
            50..=59 => 'D',
            _ => 'F',
        }
    }
}

fn score_display(student: &Student) {
    println!("Student name: {}", student.name);
    println!("Pass level: {}", student.is_pass());
    println!("Grade: {}", student.grade());
}

fn main() {
    let student = Student {
        name: String::from("Alice"),
        score: 78,
    };

    score_display(&student);
}