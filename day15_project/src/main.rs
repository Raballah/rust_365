// on Vectors and Collections
// Accessing elements in vectors, indexing can panic. use .get()

fn main() {
    let numbers = vec![1, 3, 7];

    match numbers.get(1) {
        Some(val) => println!("Value of x: {}", val),
        None => println!("Such index not found!"),
    }

    if let Some(val) = numbers.get(4) {
        println!("Value at index 4: {}", val);
    } else {
        println!("The vector has non value at index 4!");
    }
}