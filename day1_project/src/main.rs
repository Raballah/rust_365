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
/*fn main() {
    let code = String::from("James");
    let mut name = String::from("Echo ");

    name.push_str(&code);
    println!(" {} is a funny name.", name);
}*/

// String Length
/*fn main() {
    let money = String::from("é");
    println!("String length: {}", money.len());
}*/

// Not possible to index Strings
// Strings are UTF-8, Unicode Transformation Format - 8 byte 
// Characteres might take up multiple bytes, ambiguous to Index
// 
/*fn main() {
    let name = String::from("Order");
    let number = name[0];

    println!("{}", number);
}*/

// Accessing String characters safely, as not possible to index
// Access the 7th character.
// Characters vary in their bytes, so recommended to access using index like s[0]
// Bytes have 8 bits u8, or 1 byte, while characters are always assigned to 
// 4 bytes, u32
// Bytes are i) always 1 byte (8 bits) u8, ii) raw numeric data (0-255) iii) .bytes() returns individual bytes iv) Several bytes can form one character
// Characters ii) always 4 bytes (32 bits) ii) A unicode scalar value iii) .chars() returns each Unicode scalar value v) One char is a single code point.

// Accessing characters in a String 

/*fn main() {
    let full_name = String::from("Jamёs Right");

    // Access the 5th character
    if let Some(third) = full_name.bytes().nth(3) {
        println!("{}", third);
    }
}*/
//.bytes() brings up all the bytes in the String, usually represented as numeric data 0-255
//.chars() makes it possible to bring the individual Unicode scalar values in a String
// No wonder we have: 

/*fn main() {
    let name = String::from("Jóhn");

    if let Some(third) = name.bytes().nth(3) {
        println!("The third character is {}", third);
    } else {
        println!("No character found!");
    }
}*/

// Bytes vs Characters
/*fn main() {
    let status = String::from("Wóke");
    // print all bytes in this String
    for byte in status.bytes() {
        println!("{}", byte);
    }
}*/

/*fn main() {
    let vital = String::from("Wealth");

    // return all characters in a String
    for char in vital.chars() {
        println!("{}", char);
    }
}*/

// Iterating Over Strings

/*fn main() {
    let power = String::from("Health");

    // Iterate over the String to bring up all characters in the String
    for char in power.chars() {
        println!("{}", char);
    }
}*/

// Slicing Strings
// Slicing Strings using a range slicing
/*fn main() {
    let food = String::from("Ugali");
    
    let slice = &food[0..2];

    println!("The slice: {}", slice);
} */

// Use of .chars().take(2).collect();

/*
fn main() {
    let food1 = String::from("Ugali");
    let food2 = String::from("🥗Ugali");
    let food3 = String::from("こんにちは");

    // .take(2) to always give the first 2 characters
    let safe_slice1: String = food1.chars().take(2).collect();
    let safe_slice2: String = food2.chars().take(2).collect();
    let safe_slice3: String = food3.chars().take(2).collect();

    println!("First: {}", safe_slice1);
    println!("Second: {}", safe_slice2);
    println!("Third: {}", safe_slice3);
} */

// Slicing - use of the unicode-segmentation crate to slice String characters
/*
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let yummy = String::from("👍🏽Ugali🥗");

    let agreed: String = yummy.graphemes(true).take(1).collect();

    println!("Good meal: {}", agreed);
}*/

// Replacing text in Rust Strings
/*fn main() {
    let command = "You need to go!"; // string literal, command a pointer to data that is baked directly onto the compiled binary
    let new_command = command.replace("go!", "run!"); // takes ownership of the &str, now new_command is a String.

    println!("{} Why?", new_command);
}*/

/*fn main() {
    let main = "He is here.";
    let next_main = main.to_uppercase();

    println!("{}", next_main);
}*/

/*fn main() {
    let lie = "He is a good player.";
    let smaller = lie.to_lowercase();

    println!("{}", smaller);
} */

// Splitting Strings
/*fn main() {
    let csv = "Alice, 90, Bob, 50";

    for part in csv.split(", ") {
        println!("{}", part);
    }
}*/

// .split() uses char, space or &str to split, what is supplied is the backbone of the split
// spliting using char character
/*fn main() {
    let gone = "He, 48, and, Bob, Move, 89";

    for word in gone.split(',') {
        println!("{}", word);
    }
}*/

// Splitting Strings using .split() and closure
// closure - splitting on any character matching a condition
// The use of closure for splitting Strings 

/*fn main() {
    let gone = "He, 48, and, Bob, Move, 89";

    for word in gone.split(|c: char| c == ',' || c == ' ') {
        println!("{}", word);
    }
}*/

// Splitting Strings
/*
fn main() {
    let task = "He makes sense, but, rather, 169, on street, Nairobi";

    for word in task.split(|c: char| c == ' '|| c == ',') {
        println!("{}", word);
    }
}
*/

// Trimming whitespace 
/*
fn main() {
    let coincidence = "He is also here! ";

    let no_space = coincidence.trim();

    println!("{}", no_space);
}
*/

// Checking content with Strings
// The use of .contains(""), ends_with(""), and starts_with("")
// Use of .contains() -> returns a bool
/*
fn main() {
    let baked_binary = "Someone is creepy right now.";

    if baked_binary.contains("creepy") {
        println!("Delimiter found");
    } else {
        println!("No such word exist");
    }
}
*/

// Use of .ends_with() to find some content
/*
fn main() {
    let gone = "The days when there was honour.";

    if gone.ends_with("honor") {
        println!("This is definitely a honour!");
    } else {
        println!("No word found!");
    }
}
*/

// Use of starts_with()
/*
fn main() {
    let scared = "Seeing something you cannot explain.";

    if scared.to_lowercase().starts_with("seeing") {
        println!("This is true!");
    } else {
        println!("Not true.");
    }
}
*/
// Use of .find(), accepts a char, an &str, or a closure

/*
fn main() {
    let mark = "You know, sometimes, at all, you know 2015, and January.";

    match mark.find(|c: char| c == ','|| c == ' ') {
        Some(item) => println!("First match at index: {}", item),
        None => println!("No char available!"),
    }
}
*/

// Parsing Strings Into Numbers
// Checking content
/*fn main() {
    let word = "I am not your Agemate!";

    if word.to_lowercase().contains("not") {
        println!("Yes, I have found it.");
    } else {
        println!("Are you sure it's in there?");
    }
}*/
// use of ends_with
/*fn main() {
    let paragraph = "I think he's taken enough. This is now Enough!";
    let lower = paragraph.to_lowercase();

    // counting occurences of 'enough'
    let vector = lower.matches("enough").collect::<Vec<&str>>();
    println!("The matches collected as vector: {:?}", vector);
    
    // find position of first occurence // .find() returns Options<usize>
    match lower.rfind("enough") {
        Some(index) => println!("'enough' foun at byte index: {}", index),
        None => println!("Nothing found"), 
    }
    // Check if it contains enough.
    if lower.contains("enough") {
        println!("'enough' exists!");
    }

    // the use of .match_indices(), returns iterator of (byte_index, &str)
    // Use of match_indices() to provide exact index and value found, .find() and .matches() combination
    for (index, matched) in lower.match_indices("enough") {
        println!("{} found at byte index {}.", matched, index);
    }
}*/

// Use of .starts_with()

fn main() {
    let food_aspect = "Super crunchy with a lot of super crunchy parts.";

    if food_aspect.starts_with("super") {
        println!("Yeah, that's right!");
    } else {
        println!("Wrong about that!");
    }
}