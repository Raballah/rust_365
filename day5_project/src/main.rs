// Day 6: Functions and Return Values
// Day 6 Mini Project: "Student Score Analyzer CLI"

fn main() {
    let number_given = 15;

    match number_given {
        w if w < 1 => println!("Ignore these!"),
        1 => println!("Lowest Number!"),
        2..=13 => println!("Almost a Big Number!"),
        worst_nightmare @ 14..17 => println!("This number is my {}", worst_nightmare),
        17..=20 => println!("Largest of them all."),
        _ => println!("Not necessary!"),
        }
}