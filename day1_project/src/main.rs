// Day 16 - Strings in Depth 
fn main() {
    // Strings => owned, growable UTF-8 strings stored on the heap
    let mut response: String = String::from("Are you sure ");
    response.push_str("?");
    println!("{} Yes, I am sure!", response); // Unlike vectors Vec, String implements Display trait {} for UTF-8 text
}