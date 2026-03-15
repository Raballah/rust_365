// Day 5: Loops 
// for loop. Using for with ranges.
// the continue key word with for loop
// continue skips the current iteraction

fn main() {
    let ages = [34, 28, 25, 33, 35, 41, 38, 21, 27];

    let mut possible_age: Vec<i32> = Vec::new();

    for age in ages {
        if age < 35 {
            println!("{}", age);
            possible_age.push(age);
        }
    }

    for age in possible_age {
        if age == 34 {
            continue;
        }
        println!("Eligible age: {}", age);
    }
}