// Day 12 mini projec. refactoring CLI Scores Manager with struct Student 

/*
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
}

struct Statistics {
    count: usize,
    average: f64,
    highest: i32,
    lowest: i32,
}

fn compute_statistics(scores: &[i32]) -> Statistics {
    let count = scores.len();
    let sum: i32 = scores.iter().sum();
    let average = sum as f64 / count as f64;
    let highest = *scores.iter().max().unwrap(); // Safe caller guarantees non-empty
    let lowest = *scores.iter().min().unwrap();

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

fn is_valid(score: i32) -> bool {
    (0..=100).contains(&score)
}

fn add_score(students: &mut Vec<Student>) {  // Modifies the scores mut vector, borrowed here.
        
    loop {
        // Begin with student name
        let name = read_input("Enter student name or 'exit' to Exit: ");
        
        if name.eq_ignore_ascii_case("exit") {
            println!("Exited to main main successfully...");
            break; // Back to (Menu/Outer) loop.
        } else {
            continue;
        }
        
        // Add student score
        // score entry validation
        let trimmed_score = read_input("\nEnter Student Score or 'exit' to Exit: ");

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

         if !is_valid(score) {
            println!("Invalid. Enter 0-100. No Negative Entries.");
            continue;
         } else {
            students.push(Student {name, score});
            println!("Student {} added. Students added so far: {}", student.name, students.len());
         }
    }
}

fn view_scores(scores: &[i32]) {  // Borrows scores &Vec<i32>, displays as i32
    //View All Scores
    loop {
        if scores.is_empty() {
            println!("No score found! Select 1 and add some scores first!");
            return;
        }
        
        println!("\n--- All Scores ---");

        for score in scores {
            println!("{}", score);
        }
        
        // compute once, compute via method
        let stats = compute_statistics(scores);
        stats.display();
     
        let trimmed = read_input("\nType 'exit' to return to Menu: ");
        
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Exiting...Back to Main Menu!");
            break;
        }
        continue;
    }
}

fn analyze_scores(students: &Vec<Student>) {
    // Analyze Scores
    loop {
        if students.is_empty() {
            println!("No scores found!. Select 1 and enter some scores to analyze!");
            return;
        }
        
        println!("\n--- Score Analysis ---\n");
        
        for score in students {
            let grade = grade(&students);
            let label = if is_pass(&students) {"Yes"} else {"No"};
            let message = feedback(*score);
            
            println!("Score: {} | Grade: {} | Pass?: {} | Comment: {}", score, grade, label, message);
        }

        println!("\n--Pass/Fail Overview--\n");
        let mut pass_count: i32 = 0;
        let mut fail_count: i32 = 0;

        for &score in students {
            if is_pass(score) == true {
                pass_count += 1;
            } else {
                fail_count += 1;
            }
        }

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

fn feedback(score: i32) -> &'static str {
    match score {
        w if w > 100 || w < 0 => "Invalid Score. Score Must be From 0-100.",
        80..=100 => "Excellent Work!",
        70..=79 => "Good Work. Keep Improving!",
        60..=69 => "Good Trial, but Needs Improvement!",
        50..=59 => "Fair Trial. Work Harder Next Time!",
        0..=49 => "Below Average. See Me!",
        _ => "Invalid Score",
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
*/

// Bank use case of struct with impl method syntax. 


struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Add a new account (constructor-like associated function)
    fn new(owner: String, initial_balance: f64) -> Self {
        if initial_balance < 0.0 {
            panic!("Initial Balance cannot be negative");
        }
        BankAccount {
            owner,
            balance: initial_balance,
        }
    }

    // Instance: read balance of account 
    fn get_balance(&self) -> f64 {
        self.balance
    }

    // Method to deposit funds into the account, you mutate state
    fn deposit(&mut self, amount: f64) {
        if amount > 0.00 {
            self.balance += amount;
        }
    }

    // Instance of withdrawal, mutate state.
    fn withdrawal(&mut self, amount: f64) -> bool {
        if amount > 0.00 && amount <= self.balance {
            self.balance -= amount;
            true
        } else {
            false
        }     
    }
}

fn main() {
    // Create an account
    let mut account = BankAccount.new("James".to_string(), 1500.0);

    account.deposit(550.0);
    // display current balance
    println!("Balance: {}", account.get_balance());

    // Proceed to withdraw of 800.0
    let withdraw_success = account.withdrawal(800.0);
    println!("Withdrawal possibility: {} | Balance after withdrwal: {}", withdraw_success, account.get_balance());
}