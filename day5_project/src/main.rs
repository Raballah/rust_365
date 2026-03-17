// Day 5: Loops 
// for loops, comprising iteration, borrowing, mutation, ownership presevation

fn main() {
    let mut numbers = vec![1, 2, 3];

    for n in &mut numbers {  // mutable borrow of the vector (read and write), yields an iterator n with the pointer: &mut i3
        *n += 1;  // *n deferences the pointer, allows access to the referenced i32 elements of the collection/vector
    } // borrow ends here. vector modified to vec![2, 3, 4] (no printing here.)

    for n in &numbers { // borrow vector, an iteractor n with the pointer: &i32
        println!("{}", n); // println! macro implements Display. 2, 3, and 4 printed.  
    } // borrow ends here. vector remains vec![2, 3, 4], available for use beyond this for loop iterator scope.

    println!("{:?}", numbers); // prints [2, 3, 4]
}