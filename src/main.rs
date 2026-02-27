//let's now focus on swapping variables through destructuring 
// Rust makes it possible to swap two variables elegantly using tuple destructuring
fn main() {
    let person = ("Alice", 39, false);
    let (name, age, is_active) = person;

    println!("{} used to be {} years old when you were 9 years old. is this right?: {}", name, age, is_active);
}