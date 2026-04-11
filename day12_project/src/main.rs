struct Student {
        name: String,
        score: i32,
        passed: bool,
    }

fn main() {
    let mut student = Student {
        name: String::from("Alice"),
        score: 85,
        passed: true,
    };

    student.name = String::from("Mercy");
    student.passed = false;

    println!("{}", student.name);
    println!("{}", student.score);
    println!("{}", student.passed);
}