// Day 15 Vectors and Collections 
/*
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(20);
    v.push(18);
    v.push(4);
    v.push(15);

    println!("The vector has the following numbers: {:?}", v);

    if let Some(val) = v.pop() {
        println!("The new numbers in the vector are: {:?}", v);
    }
} */

/*
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const SAVE_FILE: &str = "ages.json"; // a saved-file existing for the program, with data

#[derive(Debug, Serialize, Deserialize)]
struct Age {
    name: String,
    age: u8,
}

// function to load from file on startup
fn load_data() -> Result<Vec<Age>, Box<dyn std::error::Error>> {
    // Check if the file exists, where checking happens... its in the file path,
    // no wonder you use std::path::Path; so, check if it exists from PATH,
    // file you are checking on your code is identified by the constant SAVE_FILE, 
    // but saved as "ages.json" on computer, JSON FILE systems are string, including output and naming conventions.
    // File existing: 
    // We anticipate that the file does not initially exist, so we return an error message in string.
    if !Path::new(SAVE_FILE).exists() {
        return Err("File does not exist".into());
    }

    // File exists at this point. Now, it's saved as a JSON file "ages.json", 
    // its content is in &str, so, read the content to string for Rust to Deserialize
    // label the content as you read to string using the file system, fs
    // So, the file itself exists, you are not moving it, just reading it.
    // the crate specified here is used: use std::fs; helps in reading the file
    
    let content = fs::read_to_string(SAVE_FILE)?;

    // File now read to string, so, it is in string literal &str type, 
    // the data type of SAVE_FILE. now what? extract data from it, 
    // is this where the Deserialize part of serde is used? Yes
    // Deserialize content back to Vec<Age>, so, assign a label of type Vec<Age>
    // Deseralize JSON string back into vec<Age>
    let data: Vec<Age> = serde_json::from_str(SAVE_FILE)?;
    Ok(data)
}

fn save_data(data: &[Age]) -> Result<(), Box<dyn std::error::Error>> {
    // Convert Vec<Age> to a pretty-printed JSON string
    let json = serde_json::to_string_pretty(data)?;

    // Write the string to the file (creates or overwrites)
    fs::write(SAVE_FILE, json)?;
    Ok(())
}

fn main() {
    // load data on startup
    // use unwrap_or_else to provide a default empty Vec if file doesn't exist
    let mut users = load_data().unwrap_or_else(|_| {
        println!("No saved file found, starting with empty list.");
        Vec::new()      
    });

    println!("Loaded users: {:?}", users);

    // Add some new data
    users.push(Age {
        name: String::from("Alice"),
        age: 30,
    });

    // Save data before exiting 
    if let Err(e) = save_data(&users) {
        eprintln!("Failed to load data: {}", e);
    } else {
        println!("Data successfully persisted to {}", SAVE_FILE);
    }
}
*/

/*
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const SAVED_FILE: &str = "person.json";

// create the struct
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    occupation: String,
}

// Load file on startup
fn load_from_file() -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    // Check if a file exists to avoid any errors. use Path crate
    if !Path::new(SAVED_FILE).exists() {
        return Err("File does not exist".into());
    }

    // At this point, the file exists, but in json string, read to string as well.
    // Read using file system fs, as imported>> use std::fs;
    let content = fs::read_to_string(SAVED_FILE)?;

    // Deserialize the read content string, as erpresently in SAVE_FILE, to a Vec<Person>
    let data: Vec<Person> = serde_json::from_str(&content)?;
    Ok(data)
}

// save file 
fn save_file(data: &[Person]) -> Result<(), Box<dyn std::error::Error>> {
    // Convert Vec<Person> to JSON string in pretty print
    let json = serde_json::to_string_pretty(&data)?;

    // Write the string to the file, creates new or overwrites
    fs::write(SAVED_FILE, json)?;
    Ok(())
}

fn main() {
    // load data on startup 
    let mut people = load_from_file().unwrap_or_else(|_| {
        println!("No saved file found, starting with empty list.");
        Vec::new()
    });

    println!("Loaded users: {:?}", people);
    
    people.push(Person {
            name: "John".to_string(),
            age: 45,
            occupation: "Janitor".to_string(),
        });
    
    people.push(Person {
            name: String::from("Mercy"),
            age: 35,
            occupation: String::from("Welder"),
        });
    
    people.push(Person {
            name: String::from("Zed"),
            age: 29,
            occupation: String::from("Teacher"),
        });
    
    // Save data before exiting
    if let Err(e) = save_file(&people) {
        eprintln!("Failed to save data: {}", e);
    } else {
        println!("Data successfully persisted to {}", SAVED_FILE);
    }
}*/