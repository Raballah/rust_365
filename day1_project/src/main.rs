// Day 16 - Strings in Depth 
// the format! approach
/*fn main() {
    let small = String::from("Cow");
    let big = String::from("Shit");

    let bad_word = small + &big;
    println!("That is some {} talk.", bad_word);
}*/

fn main() {
    let small = String::from("Cow");
    let big = String::from("Shit");

    let bad_word = format!("{} {}", small, big);
    println!("That is some {} talk.", bad_word);
    println!("Small: {}, Big: {}", small, big);
}