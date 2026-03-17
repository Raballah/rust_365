// Day 5: Loops 
// for loops, comprising iteration, borrowing, mutation, ownership presevation

fn main() {
    let mut numbers = vec![1, 2, 3];

    let first = &numbers[0];
    println!("{}", first);
    
    for n in &mut numbers {
        *n += 1;
    }
}