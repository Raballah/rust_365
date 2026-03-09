// Option<> definition in Rust:
// Day 4 Mini Project: Student Score Analyzer.

/*

fn all_scores(every_score: &[i32]) {
    
    for score in every_score {
        println!("{}", score);
    }
    println!("Total number of score entries/elements: {}", every_score.len());
}

fn best_score(total_scores: &[i32]) -> Option<i32> {

    if total_scores.is_empty() {
        return None;
    }

    let mut best_performer = total_scores[0];

    for &score in total_scores {
        if score > best_performer {
            best_performer = score;
        }
    }
    Some(best_performer)
}

fn poor_score(score_record: &[i32]) -> Option<i32> {

    if score_record.is_empty() {
        return None;
    }

    let mut poor_performer = score_record[0];

    for &score in score_record {
        if score < poor_performer {
            poor_performer = score;
        }
    }
    Some(poor_performer)
}

fn main() {
    let student_scores = [91, 74, 87, 56, 68];

    println!("---All Scores---");
    all_scores(&student_scores);

    println!("\n---Highest Score---");
    match best_score(&student_scores) {
        Some(score) => println!("In this case, the highest score is {}%.", score), 
        None => println!("No scores available!"),
    }

    println!("\n---Lowest Score---");
    match poor_score(&student_scores) {
        Some(score) => println!("The worst score is {}%.", score),
        None => println!("No scores available!"),
    }

    println!("\n---First 3 Scores---");
    let first_3scores = &student_scores[..3];
    println!("Here is the slice of the first three scores in the list: {:?}", first_3scores);
}

*/

// Structs in Rust
// User-defined data types, 
// which allow users to bundle data of different types into a single functional unit.
// Defined using UpperCamelCase like PersonInfo
// Here is an example:

// Give me a struct (box of data/template for data), which contains vital information about a person.
// The creation of structs in Rust
// structs are user-defined data types, more like a data template, which enables the bundling 
// of different data types to enable a single unit of operation on data.


/*
struct CarDetails {
    make: String,
    model: String,
    year: u32,
    mileage: String,
    chassis: String,
    color: String,
}

fn main() {
    let my_car = CarDetails {
        make: String::from("Subaru"),
        model: String::from("Impreza"),
        year: 2008,
        mileage: String::from("96 KM"),
        chassis: String::from("002209848BC9"),
        color: String::from("blue"),
    };

    println!("My {} {} {} {} car of chasis number {} had a mileage of only {}."
    , my_car.color, my_car.year, my_car.make, my_car.model, my_car.chassis, my_car.mileage);
}

*/

// The use of structs A real-life use case of structs is in manaing a User Profile in a web application.
// Instead of passing around individual strings for a username and email, 
// you group the into a single custom type. 
// Further Structs Understanding, the Structs in User Profile in a web application.
// Instead of passing around individual strings for a username and email, you group all associated/
// related data into a single custom type. Here is an example:

// The Use Case: User Registration
//In this case, the struct acts as a blueprint to ensure that every user has the required ddata field.

// Defining the data structure 

/*
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Creating a particular instant of that struct
    
    let user1 = User {
        email: String::from("jamesunkownn@gmail.com"),
        username: String::from("rustacean123"),
        active: true,
        sign_in_count: 21,
    };

    // Specific Outcome. Accessing two user properties from the User Profile

    println!("The username of user1 is {}.", user1.username);
    println!("The email of user1 is {}.", user1.email);
}
*/

// Assume you are given a User Profile with name, age, and net worth.
// You want to search for people in the list to get their net worth. 

struct IndividualDetails {
    name: String,
    age: u32,
    net_worth: String,
}

// Remember to set lifetime parameter for the function, 'a
// Returns a struct, as defined in the data structure.

fn get_individual_by_name<'a>(millionaires: &'a[IndividualDetails], name: &'a str) -> Option<&'a IndividualDetails> {
    for person in millionaires {
        if person.name == name {
            return Some(person);
        }
    }
    None
}

fn main() {
    let rich_guys = vec![
        IndividualDetails {
            name: "Willy".to_string(),
            age: 67,
            net_worth: "200 USD".to_string()
        },
        IndividualDetails {
            name: "Joseph".to_string(),
            age: 58,
            net_worth: "USD 129".to_string()
        },
        IndividualDetails {
            name: "Robby".to_string(),
            age: 43,
            net_worth: "USD 562".to_string()
        },
    ];

    // Search for Robby and get the result
    match get_individual_by_name(&rich_guys, "Willy") {
        Some(guy) => println!("{} has a net worth of {}.", guy.name, guy.net_worth),
        None => println!("No rich guy found!"),
    }

    // Find out the net worth of Mercy
    match get_individual_by_name(&rich_guys, "Mercy") {
        Some(lady) => println!("{} is aged {} with a net worth of {}.", lady.name, lady.age, lady.net_worth),
        None => println!("No, this is not a millionaire!"),
    }
}