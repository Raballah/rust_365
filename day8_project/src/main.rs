// Day 9, borrowing and references
// Day 9 Mini Project - Borrowing Fix 

// Display first, 4th, and last word in this sentence

/*
fn main() {
    let sentence = String::from("You will not believe what I just found out!");

    let mut words = sentence.split_whitespace(); // a lazy iterator

    // get first word

    let first = words.next().unwrap_or("none");
    println!("First word: {}", first);

    // get last word.. a new use of the lazy iterator split_whitespace
    let last = sentence.split_whitespace().last().unwrap_or("none");
    println!("Last word: {}", last);

    // display 4th word, still reuse the lazy iterator, but focus on .nth(3)

    let fourth = sentence.split_whitespace().nth(3).unwrap_or("none");
    println!("Fourth word: {}", fourth);
} 
*/
struct FruitBox {
    customer_name: String,
    fruit_type: String,
    children: u32,
}

impl FruitBox {
    // calculating shipping cost based on fruit type
    fn calculate_shipping(&self) -> f64 {
        if self.fruit_type == "Mango" {
            10.50 // mangos are heavy
        } else {
            5.00 // others are lighter/cheaper to ship.
        }
    }
}

fn main() {
    let mut major_package: Vec<FruitBox> = vec![
        FruitBox { customer_name: String::from("John"), fruit_type: String::from("Mango"), children: 4 },
        FruitBox { customer_name: String::from("Mary"), fruit_type: String::from("Oranges"), children: 3 },
        FruitBox { customer_name: String::from("Moses"), fruit_type: String::from("Apples"), children: 1 },
        FruitBox { customer_name: String::from("Kay"), fruit_type: String::from("Mango"), children: 4 },
        FruitBox { customer_name: String::from("Paul"), fruit_type: String::from("Oranges"), children: 6 },
    ];

    // Display item in one of the structs
    println!("Mary's box contains {}", major_package[1].fruit_type);

    if let Some(marys_box)  = major_package.iter_mut().find(|b| b.customer_name == "Mary") {
        marys_box.children += 1;        
        println!("Updated: {} now has {} children.", marys_box.customer_name, marys_box.children);
    } else {
        println!("No box found for Mary!");
    } 

    let children_fed: u32 = major_package.iter().map(|s| s.children).sum();
    println!("Total children enjoying our fruits: {}", children_fed);



    let total_cost: f64 = major_package
        .iter()
        .map(|box_item| box_item.calculate_shipping())
        .sum();
    println!("The total cost of shippping is ${:.1}", total_cost);
}