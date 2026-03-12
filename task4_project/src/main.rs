// Day 4: Lessons Regarding Control Flow (if, else, else if, match)
// If expressions
// if mus be boolean expressions, not integers. 

// Using match, insted of if, as an exprsesion to return values. 
// That is, use match as an expresssion to represent a value, instead of the let itself, 
// to return a value. 

// Propensity to crime based across ages. 

fn main() {
    let age = 75; 

    let crime_propensity = match age {
        18..=25 => "Very likely!",
        26..=30 => "Likely!",
        31..=35 => "Neutral!",
        36..=40 => "Unlikely!",
        41..=45 => "Very unlikely!",
        _ => "Not likely!",
    };

    println!("Likelihood of James, 75, committing a crime: {}", crime_propensity);
}