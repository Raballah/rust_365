// Day 4: Lessons Regarding Control Flow (if, else, else if, match
// The match, but operates the same way as if statements. match as expresssion, or as a variable.
// The match feature in Rust

fn main() {
    let weight = 34;

    let weight_category = match weight {
        w if w > 100 => "obese",
        90..=100 => "very overweight",
        80..=89 => "fat",
        70..=79 => "normal",
        60..=69 => "slim",
        50..=59 => "thin",
        40..=49 => "emaciated",
        _ => "frail",
    };

    println!("Based on his weight readings now, he is definitely {}.", weight_category);
}