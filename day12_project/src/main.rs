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

// Create a vector of struct type and save file on completion, 
// load file on startup.

/*
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

const SAVED_FILE: &str = "students.json";

#[derive(Debug, Serialize, Deserialize)]
struct Student {
    parent: String,
    name: String,
    score: u32,
}

// load file JSON in string on startup
fn load_on_startup() -> Result<Vec<Student>, Box<dyn std::error::Error>> {
    if !Path::new(SAVED_FILE).exists() {
        return Err("File does not exist".into());
    }
    
    // read file to string
    let content = fs::read_to_string(SAVED_FILE)?;
    // convert the string to Vec<Student>
    let data: Vec<Student> = serde_json::from_str(&content)?;
    Ok(data)
}

// fn to save data in Vec<Student> on closing. turn it into Json
fn save_on_exit(data: &[Student]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&data)?;

    // Write the serialized data to the SAVED_FILE. WRITES OR OVERWRITES
    fs::write(SAVED_FILE, json)?;
    Ok(())
}

fn main() {
    // load data on startup
    let mut students: Vec<Student> = load_on_startup().unwrap_or_else(|_| {
        println!("No saved file found, starting with a fresh list.");
        Vec::new()
    });

    println!("Loaded users: {:?}", students);

    // Adding some users:
    students.push(Student {
        parent: String::from("John"), 
        name: String::from("Joseph"),
        score: 45,
    });

    students.push(Student {
        parent: String::from("Jowi"),
        name: String::from("Mary"),
        score: 78,
    });

    // saving data before exiting
    if let Err(e) = save_on_exit(&students) {
        eprintln!("Failed to save data: {}", e);
    } else {
        println!("Data successfully persisted: {}", SAVED_FILE);
    }
}
*/

// Third attempt at it. Let's see how it goes again. 
// First begin by importing the necessary crates

/*
use serde::{Serialize, Deserialize}; // for converting to JSON file and from JSON file
use std::fs; // for reading and writing on saved file
use std::path::Path; // for locating file on saved location. 

const FILE_STORE: &str = "workers.json";

// create struct for workers, with necessary derivations
#[derive(Debug, Serialize, Deserialize)]
struct Worker {
    name: String,
    job_id: u32,
    service_years: u8,
}

// function to load from file
fn load_from_file() -> Result<Vec<Worker>, Box<dyn std::error::Error>> {
    // Check for file first and return error if file does not exist.
    if !Path::new(FILE_STORE).exists() {
        return Err("File does not exist".into());
    }
    // files exists in this scope
    // read the json file, you read and write on file using the std::fs crate
    // this is like capturing the file for the program 
    // (more like copying a file temporarily from an existing system file)
    let content = fs::read_to_string(FILE_STORE)?;
    // After reading it, the Deserialize the json file to Vec<Worker>
    // this means you convert it from the read string json version to Vec<Worker>
    let data: Vec<Worker> = serde_json::from_str(&content)?;
    Ok(data)
}

// Create function to save file, now in Vec<Worker>.
// Turns a Vec<Worker> file, using Serialize derive, to a json file
fn save_to_json(data: &[Worker]) -> Result<(), Box<dyn std::error::Error>> {
    // you want to turn the data of Vec<Worker> to a json format here,
    // in pretty print. then write it to the SAVE_FILE using the file system std::fs
    let json = serde_json::to_string_pretty(&data)?; // turns Vec<Worker> to a json file 
    // now write it using the fs system
    fs::write(FILE_STORE, json)?;
    Ok(())
}

fn main() {
    // load existing data on startup
    let mut workers: Vec<Worker> = load_from_file().unwrap_or_else(|_| {
        println!("No file found saved, starting afresh");
        Vec::new()
    });

    println!("Loaded users: {:?}", workers);

    // Adding users:
    workers.push( Worker {
        name: "James".to_string(),
        job_id: 35790,
        service_years: 18,
    });

    workers.push( Worker {
        name: "Opar".to_string(),
        job_id: 57903,
        service_years: 10,
    });

    // saving before exiting, handling saving errors automatically. 
    // can have errors saving or not

    if let Err(e) = save_to_json(&workers) {
        println!("Failed to save file with error: {}", e);
    } else {
        println!("Data successfully persisted to {}", FILE_STORE);
    }
}
*/

// 4th attempt to better understand the mechanics
// First import the necessary crates
use serde::{Serialize, Deserialize}; // for data serilization to json and deserialization of json to data
use std::fs; // a file reading and writing system between saved file and Rust
use std::path::Path; // for locating the path of saved file, to check if saved or not

// Create a constant, SAVE_FILE, for handling the saving of file 
const SAVE_FILE: &str = "teachers.json";

// Create the Teacher struct, with necessary derivations 
#[derive(Debug, Serialize, Deserialize)]
struct Teacher {
    name: String,
    age: u8,
    graduation_year: u16,
    salary: u32,
}

// function to load data on startup
fn load_data() -> Result<Vec<Teacher>, Box<dyn std::error::Error>> {
    // Check if file exists, if not, retun an eror mesage in string
    if !Path::new(SAVE_FILE).exists() {
        return Err("File does not exist".into());
    }
    // File exists at this point. Obviously in json format.
    // read to string using the fs create, then return as content
    let content = fs::read_to_string(SAVE_FILE)?; // now, content is in string format, 
    // read from json format to string format
    // now, convert the string format to Vec<Teacher>, and return it. 
    // string to Vec<Teacher> requires deserialization suing serde_json Deserialize crate
    // bind the deserialized String to Vec<Teacher> to say, data and return it.
    let data: Vec<Teacher> = serde_json::from_str(&content)?;
    Ok(data)
}

// Function to save on exit to json format.
// Requires Serialziation, that is, changing data/ Vec<Teacher> to json file.
// So, you use serde_json serialization crate.
fn save_data(data: &[Teacher]) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize data to json format / mostly string, and give it a pretty print.
    let json = serde_json::to_string_pretty(data)?;
    // Take the serialized data and write it to the json holder, SAVE_FILE, using std::fs
    fs::write(SAVE_FILE, json)?;
    Ok(())
} 

fn main() {
    // load data on startup
    let mut teachers: Vec<Teacher> = load_data().unwrap_or_else(|_| {
        println!("No data found, starting afresh!");
        Vec::new()
    });

    // Some data to the teachers vector
    teachers.push(Teacher {
        name: String::from("Mary"),
        age: 45,
        graduation_year: 2022,
        salary: 20_000,
    });

    teachers.push(Teacher {
        name: String::from("Mark"),
        age: 24,
        graduation_year: 2016,
        salary: 36_000,
    });

    teachers.push(Teacher {
        name: String::from("Mary"),
        age: 45,
        graduation_year: 2022,
        salary: 20_000,
    });

    println!("Existing teachers: {:?}", teachers);
    
    // Save on exit
    match save_data(&teachers) {
        Ok(()) => println!("Data persistence successful"),
        Err(e) => eprintln!("Failed to save data. Error {}", e),
    }
}

