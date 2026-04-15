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

    fn feedback(&self) -> &'static str {
        match self.score {
            w if w > 100 || w < 0 => "Invalid Score. Score Must be From 0-100.",
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

fn compute_statistics(students: &Vec<Student>) -> Statistics {
    let count = students.len();
    let sum: i32 = students.iter().map(|s| s.score).sum();
    let average = sum as f64 / count as f64;
    let highest = students.iter().map(|s| s.score).max().unwrap(); // Safe caller guarantees non-empty
    let lowest = students.iter().map(|s| s.score).min().unwrap();

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
        loop {
            // Begin with student name
            let name = read_input("Enter student name or 'exit' to Exit: ");

            if name.eq_ignore_ascii_case("exit") {
                println!("Exited to outer loop successfully...");
                break; // Back to Main/Outer loop
            } else {
                break; // name collected, proceed to score entry
            }           
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
            println!("Student {} added. Number of students added so far: {}", students::name, students.len());
         }
    }
}

fn view_scores(scores: &[i32]) {  // Borrows scores &Vec<i32>, displays as i32
    //View All Scores
    loop {
        if students.is_empty() {
            println!("No score found! Select 1 and add some scores first!");
            return;
        }
        
        println!("\n--- All Scores ---");

        let scores_list = students.score;
        println!("{}", scores_list);
        
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
        for student in students {
            println!(
                "Student name: {} | Score: {} | Grade: {} | Pass: {} | Comment: {}",
                student.name,
                student.score,
                student.grade(),
                if student.is_pass() { "Yes" } else { "No" }
            );
        }

        println!("\n--Pass/Fail Overview--\n");
        let mut pass_count: i32 = 0;
        let mut fail_count: i32 = 0;

        if students.is_pass(&score) {
            pass_count += 1;
        } else {
            fail_count += 1;
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

struct FruitBox {
    customer_name: String,
    fruit_type: String,
    children: u32,
}

impl FruitBox {
    // calculating shipping cost based on fruit type
    fn calculate_shipping(&self) -> f64 {
        if self.fruit_type == "Mango" {
            10.50 // mangos are heavy
        } else {
            5.00 // others are lighter/cheaper to ship.
        }
    }
}

fn main() {
    let mut major_package: Vec<FruitBox> = vec![
        FruitBox { customer_name: String::from("John"), fruit_type: String::from("Mango"), children: 4 },
        FruitBox { customer_name: String::from("Mary"), fruit_type: String::from("Oranges"), children: 3 },
        FruitBox { customer_name: String::from("Moses"), fruit_type: String::from("Apples"), children: 1 },
        FruitBox { customer_name: String::from("Kay"), fruit_type: String::from("Mango"), children: 4 },
        FruitBox { customer_name: String::from("Paul"), fruit_type: String::from("Oranges"), children: 6 },
    ];

    // Display item in one of the structs
    println!("Mary's box contains {}", major_package[1].fruit_type);

    if let Some(marys_box)  = major_package.iter_mut().find(|b| b.customer_name == "Mary") {
        marys_box.children += 1;        
        println!("Updated: {} now has {} children.", marys_box.customer_name, marys_box.children);
    } else {
        println!("No box found for Mary!");
    } 

    let children_fed: u32 = major_package.iter().map(|s| s.children).sum();
    println!("Total children enjoying our fruits: {}", children_fed);



    let total_cost: f64 = major_package
        .iter()
        .map(|box_item| box_item.calculate_shipping())
        .sum();
    println!("The total cost of shippping is ${:.1}", total_cost);
}