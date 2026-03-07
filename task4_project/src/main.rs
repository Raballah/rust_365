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