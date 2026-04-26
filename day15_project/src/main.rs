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

/*
fn main() {
    let mut greeting = vec![String::from("Hello")];

    let hi = &vec![0]; // mutable borrow
    greeting.push(String::from(" world!")); 

    println!("Hey, {:?}!", hi);
    println!("Wow! {:?}", greeting);
}
*/

// In case of different data types, use enums not vectors. 

/*
use std::fmt;

#[derive(Debug)]
enum Data {
    Int(i32),
    Text(String),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Int(n) => write!(f, "{}", n),
            Data::Text(t) => write!(f, "{}", t), 
        }
    }
}


fn main() {
    let values = vec![
        Data::Int(46),
        Data::Text(String::from(" is the best age.")),
        ];
    
    let sentence: String = values.iter()
        .map(|item| item.to_string())
        .collect();

    println!("I think {}", sentence);
}
*/

// Common operations with vectors

fn main() {
    let mut order_numbers: Vec<i32> = vec![891, 894, 589, 738, 120, 521, 903];

    let total_orders = order_numbers.len();
    println!("Total orders: {}", total_orders);

    if order_numbers.is_empty() {
        println!("No orders found!");
    } else {
        println!("Some orders are available!");
    }

    if order_numbers.contains(&603) {
        println!("The order 589 is avaiable!")
    } else {
        order_numbers.push(603);
        println!("Order number 603 added. Current order numbers: {:?}", order_numbers);
    }

}