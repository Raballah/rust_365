// Day 16 - Strings in Depth 
// the format! approach
/*fn main() {
    let small = String::from("Cow");
    let big = String::from("Shit");

    let bad_word = small + &big;
    println!("That is some {} talk.", bad_word);
}*/

/*fn main() {
    let first = String::from("Come");
    let second = String::from("on!");

    let joint = format!("{} {}", first, second);
    println!(" {} We are going now.", joint);
} */

// use of push_str(). mutating in place
/*fn main() {
    let code = String::from("James");
    let name = String::from("Echo");

    let code_name = format!("{} {}", code, name);
    println!("{} sounds like a funny code name!", code_name);
}*/

// Use of .push_str for concatentation. 
fn main() {
    let code = String::from("James");
    let mut name = String::from("Echo ");

    name.push_str(&code);
    println!(" {} is a funny name.", name);
}