// Day 16 - Strings in Depth 
// the format! approach
fn main() {
    // Appending text
    let mut profession_type = String::from("Plumber");
    profession_type.push('!');
    println!("He is currently working as a {}", profession_type);

    profession_type.push_str(" However, are you fine with it?");
    println!("Good job being a {}", profession_type);
}