// Day 5: Loops 
// Looop Labels. Rust allows loops to be labelled, calld labelled loops. 
// Day 5 Mini Project: Student Attendance Simulation
// The project entails simulating students arrriving at a school 

/*
Student attendance simulation. Simulating students entering a class. Program has a total of 
10 students. Uses a loop to simulate attendance counting. each loop increases the number of students present. Stop when the classroom is full.
Print attendance progress.  Constraints: program must use loop, break, for, while, continue
you many split them into sections in the same program. add this rule. if student number 5 arrives, skip printing them using continue.
so, expected output excludes 5. 
Will this suggested code print. offer corrections based on this approach. 
*/

fn main() {
    let total_students = 10;
    let mut present = 0;

    for student in 1..=total_students {
        if student == 5 {
            println!("Student 5 not counted.");
            continue;
        }

        present += 1;
        println!("Student {} Entered. Currently Present: {}", student, present);
    }

    println!("Final Attendance: {}/{}", present, total_students);
}