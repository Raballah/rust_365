// on Vectors and Collections
// Accessing elements in vectors, indexing can panic. use .get()

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