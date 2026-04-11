// Day 9, borrowing and references
// Day 9 Mini Project - Borrowing Fix 

// Display first, 4th, and last word in this sentence

fn main() {
    let sentence = String::from("You will not believe what I just found out!");

    let mut words = sentence.split_whitespace(); // a lazy iterator

    // get first word

    let first = words.next().unwrap_or("none");
    println!("First word: {}", first);

    // get last word.. a new use of the lazy iterator split_whitespace
    let last = sentence.split_whitespace().last().unwrap_or("none");
    println!("Last word: {}", last);

    // display 4th word, still reuse the lazy iterator, but focus on .nth(3)

    let fourth = sentence.split_whitespace().nth(3).unwrap_or("none");
    println!("Fourth word: {}", fourth);
} 
