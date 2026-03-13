// Day 4: Lessons Regarding Control Flow (if, else, else if, match)
// Student Grade Evaluator Mini Task

fn return_items(marks: &[i32]) {
    
    for value in marks {
        println!("This is a valid value: {}", value);
    }
}
fn main() {
    let scores = vec![89, 38, 58, 27];
    
    // call the formular:
    return_items(&scores);

    println!("Every item in the collection: {:?}", scores);
}