// Day 12 mini projec. refactoring CLI Scores Manager with struct Student 

use std::io;
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

struct Student {
    name: String,
    score: i32,
}

impl Student {
    fn new(name: String, score: i32) -> Self {
        Self { name, score}
    }

    fn is_valid(score: i32) -> bool {
        (0..=100).contains(&score)
    }

    fn grade(&self) -> char {
        match self.score {
            w if w > 100 || w < 0 => 'I', // Invalid Entry/Score
            80..=100 => 'A',
            70..=79 => 'B',
            60..=69 => 'C',
            50..=59 => 'D',
            0..=49 => 'F',
            _ => 'I', // Invalid Entry/Score}
        }
    }
    
    fn is_pass(&self) -> bool {
        self.score >= 60
    }

    fn feedback(&self) -> &'static str {
        match self.score {
            80..=100 => "Excellent Work!",
            70..=79 => "Good Work. Keep Improving!",
            60..=69 => "Good Trial, but Needs Improvement!",
            50..=59 => "Fair Trial. Work Harder Next Time!",
            0..=49 => "Below Average. See Me!",
            _ => "Invalid Score",            
        }
    }
}

struct Statistics {
    count: usize,
    average: f64,
    highest: i32,
    lowest: i32,
}

fn compute_statistics(students: &[Student]) -> Statistics {
    let count = students.len();
    let sum: i32 = students.iter().map(|s| s.score).sum();
    let average = sum as f64 / count as f64;
    let highest = students.iter().map(|s| s.score).max().unwrap_or(0); // Safe caller guarantees non-empty
    let lowest = students.iter().map(|s| s.score).min().unwrap_or(0);

    Statistics { 
        count, 
        average, 
        highest,
        lowest
    }
}

impl Statistics {
    fn display(&self) {
        println!("\nScores Count: {}", self.count);
        println!("Average Score: {:.0}", self.average);
        println!("Highest Score: {}", self.highest);
        println!("Lowest Score: {}", self.lowest);
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

fn add_score(students: &mut Vec<Student>) {  // Modifies the scores mut vector, borrowed here.
        
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

         if !Student::is_valid(score) {
            println!("Invalid. Enter 0-100. No Negative Entries.");
            continue;
         } else {
            students.push(Student::new(name, score));
            // Getting reference to last student added.
            if let Some(last_student) = students.last() {
                println!("Student {} added. Number of students added so far: {}", last_name.name, students.len());
            }
         }
    }
}

fn view_scores(students: &[Student]) {  // Borrows scores &Vec<i32>, displays as i32
    //View All Scores
    loop {
        if students.is_empty() {
            println!("No score found! Select 1 and add some scores first!");
            return;
        }
        
        println!("\n--- All Scores ---");

        for student in students {
            println!("{}: {}", student.name, student.score);
        }
        
        // compute once, compute via method
        let stats = compute_statistics(students);
        stats.display();
     
        let trimmed = read_input("\nType 'exit' to return to Menu: ");
        
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Exiting...Back to Main Menu!");
            break;
        }
        continue;
    }
}

fn analyze_scores(students: &[Student]) {
    // Analyze Scores
    loop {
        if students.is_empty() {
            println!("No scores found!. Select 1 and enter some scores to analyze!");
            return;
        }
        
        println!("\n--- Score Analysis ---\n");
        for student in students {
            println!(
                "Student name: {} | Score: {} | Grade: {} | Pass: {} | Comment: {}",
                student.name,
                student.score,
                student.grade(),
                if student.is_pass() { "Yes" } else { "No" },
                student.feedback()
            );
        }

        println!("\n--Pass/Fail Overview--\n");

        let pass_count = students.iter().filter(|s| s.is_pass()).count();
        let fail_count = students.iter().filter(|s| !s.is_pass()).count();

        println!("Pass Count: {}", pass_count);
        println!("Fails Count: {}", fail_count);

        // Inpt for 'Exit' to exit loop
        let mut optional_input = String::new();
        
        println!("Type 'Exit' to exit!");
        
        io::stdin()
            .read_line(&mut optional_input)
            .expect("Failed to read optional input!");
        
            let trimmed2 = optional_input.trim();
            
            if trimmed2.eq_ignore_ascii_case("exit") {
                println!("Session Exited Successfully!");
                break;
            }
            continue;
    }
}

fn main() {
    // 1. Student Score Collector / Vector
    let mut students: Vec<Student> = Vec::new();
    
    loop {
        // 2. Menu System (Core Feature)
        show_menu();

        // 3. Menu Choice Handler - reprompts, iteration ends at valid choice
        let choice: MenuOption = get_user_input();

        // 4. Choice-based Actions
        match choice {
            MenuOption::AddScore => add_score(&mut students),
            MenuOption::ViewScores => view_scores(&students),
            MenuOption::AnalyzeScores => analyze_scores(&students),
            MenuOption::Exit => {
                println!("Session Exited Successfully! Goodbye!");
                break; // Outer loop exited, program exited.
            }
        }
    }
}