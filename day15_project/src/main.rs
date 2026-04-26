// on Vectors and Collections
// Accessing elements in vectors, indexing can panic. use .get()

/*
fn main() {
    let numbers = vec![11, 34, 74, 12, 78, 37, 57];

    match numbers.get(5) {
        Some(value) => println!("Value at index 5: {}", value),
        None => println!("No value at index 5!"),
    }

    match numbers.get(20) {
        Some(val) => println!("Value of x: {}", val),
        None => println!("Such index not found!"),
    }

    if let Some(num) = numbers.get(4) {
        println!("Value at index 4: {}", num);
    } else {
        println!("The vector has non value at index 4!");
    }
}
*/
// Iteratign over vectors

/*
fn main() {
    let mut shoe_sizes: Vec<i32> = vec![34, 45, 32, 12, 32, 22, 29];

    for (index, size) in shoe_sizes.iter_mut().enumerate() {
        *size += 2;
        println!("Index {}: Shoe size: {}", index, size);
    }
}
*/

// Ownership rules with vectors

fn main() {
    let mut greeting = vec![String::from("Hello")];

    let hi = &vec![0]; // mutable borrow
    greeting.push(String::from(" world!")); 

    println!("Hey, {:?}!", hi);
    println!("Wow! {:?}", greeting);
}