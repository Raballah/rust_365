// Day 14: Struct + Enum Driven CLI System 

use std::io;
use std::fmt;
// Extracted input reading into a single function
fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    
    input.trim().to_string()
}

fn show_menu() {
    println!("\n=== Student Score Manager ===");

        println!("\n1. Add student score");
        println!("2. View all scores");
        println!("3. Analyze scores");
        println!("4. Exit");
}

enum PassStatus {
    Pass,
    Fail,
}

impl fmt::Display for PassStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match self {
            PassStatus::Pass => "Pass",
            PassStatus::Fail => "Fail",
        };
        write!(f, "{}", p)
    }
}

struct Student {
    name: String,
    score: i32,
}

impl Student {
    fn new(name: String, score: i32) -> Option<Self> {
        if (0..=100).contains(&score) {
            Some(Self { name, score })
        } else {
            None
        }
    }

    fn grade(&self) -> Grade {
        match self.score {
            80..=100 => Grade::A,
            70..=79 => Grade::B,
            60..=69 => Grade::C,
            50..=59 => Grade::D,
            _ => Grade::F,
        }
    }
    
    fn result(&self) -> PassStatus {
        if self.score >= 60 {
            PassStatus::Pass
        } else {
            PassStatus::Fail
        }
    }

    fn feedback(&self) -> &'static str {
        match self.score {
            80..=100 => "Excellent Work!",
            70..=79 => "Good Work. Keep Improving!",
            60..=69 => "Good Trial, but Needs Improvement!",
            50..=59 => "Fair Trial. Work Harder Next Time!",
            0..=49 => "Below Average. See Me!",
            _ => "Below average. See me!",            
        }
    }

    fn performance(&self) -> Performance {
        match self.score {
            80..=100 => Performance::Excellent,
            70..=79 => Performance::Good,
            60..=69 => Performance::Average,
            50..=59 => Performance::Weak,
            _ => Performance::Poor,
        }
    }

    fn display(&self) {
        println!(
            "Student name: {} | Score: {} | Grade: {} | Pass: {} | Comment: {} | Performance level: {}",
            self.name,
            self.score,
            self.grade(),
            self.result(),
            self.feedback(),
            self.performance()
        );
    }
}

enum Grade {
    A,
    B,
    C,
    D,
    F,
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
            Grade::F => "F",
        };
        write!(f, "{}", s)
    }
}

struct Statistics {
    count: usize,
    average: f64,
    highest: i32,
    lowest: i32,
}

impl Statistics {
    fn display(&self) {
        println!("\nScores Count: {}", self.count);
        println!("Average Score: {:.0}", self.average);
        println!("Highest Score: {}", self.highest);
        println!("Lowest Score: {}", self.lowest);
    }

    fn from_students(students: &[Student]) -> Self {
        let count = students.len();
        let sum: i32 = students.iter().map(|s| s.score).sum();
        let average = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
        let highest = students.iter().map(|s| s.score).max().unwrap_or(0);
        let lowest = students.iter().map(|s| s.score).min().unwrap_or(0);

        Self { count, average, highest, lowest }
    }
}

enum Performance {
    Excellent,
    Good,
    Average,
    Weak,
    Poor,
}

impl fmt::Display for Performance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let performance = match self {
            Performance::Excellent => "Excellent",
            Performance::Good => "Good",
            Performance::Average => "Average",
            Performance::Weak => "Weak",
            Performance::Poor => "Poor",
        };
        write!(f, "{}", performance)
    }
}

enum MenuOption {
    AddScore,
    ViewScores,
    AnalyzeScores,
    Exit,
}

fn parse_menu_choice(input: i32) -> Option<MenuOption>{
    match input {
        1 => Some(MenuOption::AddScore),
        2 => Some(MenuOption::ViewScores),
        3 => Some(MenuOption::AnalyzeScores),
        4 => Some(MenuOption::Exit),
        _ => None, // All i32 inputs are invalid
    }
}

// 1. Student Score Collector / App Layer
struct App {
    students: Vec<Student>,
}

impl App {
    fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    fn run(&mut self) {
        loop {
            // 2. Menu System (Core Feature)
            show_menu();

            // 3. Menu Choice Handler - reprompts, iteration ends at valid choice
            let choice: MenuOption = get_user_input();

            // 4. Choice-based Actions
            match choice {
                MenuOption::AddScore => self.add_student(),
                MenuOption::ViewScores => self.view_students(),
                MenuOption::AnalyzeScores => self.analyze_students(),
                MenuOption::Exit => {
                    println!("Session Exited Successfully! Goodbye!");
                    break; // Outer loop exited, program exited.
                }
            }
        }
    }

    fn add_student(&mut self) {  // Modifies the scores mut vector, borrowed here.
        
    loop {
        // Name declared within the main loop
        let name = read_input("Enter student NAME or 'exit' to Exit: ");
        
        if name.eq_ignore_ascii_case("exit") {
            break; // Exits and heads over to main menu
        }
        
        // Add student score
        // score entry validation
        let trimmed_score = read_input("\nEnter student SCORE or 'exit' to Exit: ");

        if trimmed_score.eq_ignore_ascii_case("exit") {
            println!("Score(s) Added. Returning to Menu...");
            break; // Back to (Menu/Outer) loop.
        }
        
        let score: i32 = match trimmed_score.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Entry. Score Must be From 0-100.");
                continue;
            }
         };

         match Student::new(name, score) {
            Some(student) => {
                self.students.push(student);
                println!("Student added"); 
            },
            None => println!("Invalid. Enter 0-100. No Negative Entries."),
         }

         // Getting reference to last student added.
         if let Some(last_student) = self.students.last() {
            println!(
                "Last added student: {}. Number of students added so far: {}",
                last_student.name,
                self.students.len()
            );
         }
        }
    }

    fn view_students(&self) { // Borrows scores &Vec<i32>, displays as i32
    //View All Scores
        loop {
            if self.students.is_empty() {
                println!("No score found! Select 1 and add some scores first!");
                return;
            }
            
            println!("\n--- All Scores ---");
            for student in &self.students {
                println!("{}: {}", student.name, student.score);
            }
            
            // compute once, compute via method
            let stats = Statistics::from_students(&self.students);
            stats.display();
        
            let trimmed = read_input("\nType 'exit' to return to Menu: ");
            
            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Successfully exited to main menu...!");
                break;
            }
        }
    }

    fn analyze_students(&self) {
        // Analyze Scores
        loop {
            if self.students.is_empty() {
                println!("No scores found!. Select 1 and enter some scores to analyze!");
                return;
            }
            
            println!("\n--- Score Analysis ---\n");
            for student in &self.students {
                student.display();

                println!("\n--- Score Implication for the Student ---\n");
                match student.grade() {
                    Grade::A => println!("Top student"),
                    Grade::B => println!("Upcoming top performer"),
                    Grade::C => println!("Slow learner. Scaffolding recommended"),
                    Grade::D => println!("Remedial reuired"),
                    Grade::F => println!("Needs help"),                   
                }
            }

            println!("\n--Pass/Fail Overview--\n");
            let pass_count = self.students.iter()
                .filter(|s| {
                    let r = s.result();
                    matches!(r, PassStatus::Pass)
                    })
                    .count();
            let fail_count = self.students.len() - pass_count;

            println!("Pass Count: {}", pass_count);
            println!("Fails Count: {}", fail_count);

            // Inpt for 'Exit' to exit loop
            let trimmed = read_input("\nType 'Exit' to exit!");

            if trimmed.eq_ignore_ascii_case("exit") {
                println!("Session Exited Successfully!");
                break;            
            }
        }
    }
}

fn get_user_input() -> MenuOption {
    loop {
        let menu_choice = read_input("\nEnter your choice: ");

        if let Ok(num) = menu_choice.parse::<i32>() {
            if let Some(option) = parse_menu_choice(num) {
                break option; // Valid MenuOption, exit loop
            }
        }
        // if parse failed or number number not 1-4
        println!("Invalid Entry. Choice must be from 1-4!");
        continue;
    }
}

fn main() {
    App::new().run();
}