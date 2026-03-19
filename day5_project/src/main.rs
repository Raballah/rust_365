// Day 6: Functions and Return Values
// Day 6 Mini Project: "Student Score Analyzer CLI"

// Two control flows are if and match
// if accepts boolean expressions, no ranges like 1..=78
// match goes with ranges, for example 3..=67;

// No need to use loops (for, while, loop break, continue- eliminates of what has been ===) with control 
// No need t include control flows with loops, but control flows are used inside loops, where necessary

// Looping through numbers and categorizing into specific ranges, needs control flow if match inside loops

fn main() {
    let freight_size = 45;

    match freight_size {
        w if w < 17 => println!("Not significant enough!"),
        17..=31 => println!("Seriously in demand!"),
        32..42 => println!("Somewhat in demand!"),
        nightmare @ 42..=48 => println!("Dwindling demand! {} falls here!", nightmare),
        49..=51 => println!("Almost no demand!"),
        _ => println!("Ignorable!"),
        }
}