// Day 5: Loops 
// for loops, comprising iteration, borrowing, mutation, ownership presevation

fn main() {
    let mut numbers = vec![1, 2, 3];

    for n in &numbers {  // Borrows the vector, produces an iteractor n: &132
        println!("{}", n); // Prints 1, 2, 3. println! macro implements Display
    } // borrow ends, vector released as vec![1, 2, 3]

    for n in &mut numbers {  // mutable borrow of the vector, we have iteractor n: &mut i32
        *n *=2;  // * deferences the pointer, giving the iteractor pointer &mut i32, access to the i32 elements.
    } // borrow ends here, but yields/modifies initial vector to vec![2, 4, 6]

    for n in numbers { // moves ownership of the vector to the for iteractor loop
        println!("{}", n); // prints 2, 4, and 6, the i32 elements as borrowed.
    } // borrowing ends here. ownership moves to the for loop iterator. vector not available for further use beyond this beyond.
}