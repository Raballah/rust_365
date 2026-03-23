use std::io;

fn blood_scanner(age: i32) -> char {
    match age {
        w if w > 80 || w < 0 => 'I', // Invalid age. Please enter age lower than 36.
        60..=80 => 'A',
        31..=59 => 'B',
        0..=30 => 'C',
        _ => 'I',
    }
}

fn is_valid(age: i32) -> bool {
    (0..=80).contains(&age)
}

fn main() {
    loop {
        let mut input = String::new();

        println!("Enter Age to Determine Blood Group or 'exit' to Exit: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        let case_trim = input.trim();
        let trimmed = case_trim.to_lowercase();

        if trimmed == "exit" {
            println!("Session Exited Successfully!");
            break; // Breaks iteration loops and exits
        }

        let age: i32 = match trimmed.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Entry. Enter Age From 0-80!");
                continue;
            }
        };

        if !is_valid(age) {
            println!("Invalid Entry. Enter Age From 0-80!");
            continue;
        }

        let blood_group = blood_scanner(age);

        println!("Based on the age, the patient's blood group is valid and of type {}.", blood_group);
    }
}