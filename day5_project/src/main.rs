// Day 5: Loops 
// for loop, mutable and immutable borrows.

fn main() {
    let mut numbers = vec![5, 10, 15];

    for n in numbers.iter() {
        println!("{}", n);
    }

    for n in numbers.iter_mut() {
        *n += 5;
    }
    println!("{:?}", numbers);
}