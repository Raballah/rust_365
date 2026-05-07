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

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const SAVE_FILE: &str = "ages.json"; // a saved-file existing for the program, with data

#[derive(Debug, Serialize, Deserialize)]
struct Age {
    name: String,
    age: u8,
}

fn main() {
    let mut users: Vec<Age> = vec![
        { name: "Mercy".to_string(), age:78, }
        { name: "Onduru".to_string(), age: 38, }
        { name: "Pauline".to_string(), age: 28, }
    ];
}
// function to load from file on startup
fn load_data() -> Result<Vec<Age>, Box<dyn std::error:Error>> {
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
    // its content is in &str, so, read the content to String for Rust to code.
    // label the content as you read to string using the file system, fs
    // So, the file itself exists, you are not moving it, just reading it.
    // the create specified here: use std::fs; helps in reading the file
    
    let content = fs::read_to_string(SAVE_FILE)?;

    // File now read to string, so, it is in String literal &str type, 
    // the data type of SAVE_FILE. now what? extract data from it, 
    // is this where the Deserialize part of serde is used? Yes
    // Deserialize content back to Vec<Age>, so, assign a label of type Vec<Age>
    let data: Vec<Age> = serde_json::from_str(SAVE_FILE)?;

}












fn load_data() -> Result<Vec<Age>, Box<dyn std::error::Error>> {
    // check first if file exists
    if !Path::new(SAVE_FILE).exists() {
        return Err("File does not exist", into());
    }

    // where it exists, read file contents to string
    let content = fs::read_to_string(SAVE_FILE)?;

    // Deserialize JSON back to Vec<Age>
    let data: Vec<Age> = serde_json::from_str(&content)?;
    Ok(data)
}

fn main() {

}