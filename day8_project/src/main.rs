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
/*
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
*/

// Focus on enums

/*
enum Grade {
    A,
    B,
    C,
    D,
    F
}

fn describe_grade(grade: Grade) -> &'static str {
    match grade {
        Grade::A => "Excellent work",
        Grade::B => "Good",
        Grade::C => "Average",
        Grade::D => "Below average",
        Grade::F => "Fail",
    }
}
fn main() {
    let g = Grade::A;
    match g {
        Grade::A => println!("Excellent attempt");
        Grade::B => println!("Good work")
        _ => println!("Keep working harder!")
    }
}
*/
 
// A Filter iterator adapter and a matches! macro overivew
// matches! returns a bool, so good for providing the predicate to use with filter

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    NonBinary(String), // holds a label
}

impl Gender {
    fn is_binary(&self) -> bool {
        matches!(self, Gender::Male | Gender::Female)
    }

    fn label(&self) -> &str {
        match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::NonBinary(inner_text) => inner_text,
        }
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

impl Person {
    fn new(name: &str, age: u8, gender: Gender) -> Self {
        Person { name: name.to_string(), age, gender }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn gender_label(&self) -> &str {
        self.gender.label()
    }
}

fn main() {
    let people = vec!{
        Person::new("Alice",  30, Gender::Female),
        Person::new("Bob",    25, Gender::Male),
        Person::new("Carol",  17, Gender::Female),
        Person::new("Dave",   40, Gender::Male),
        Person::new("Erin",   22, Gender::NonBinary("Genderqueer".into())),
        Person::new("Frank",  19, Gender::Male),
    };

    // Counting number of males and females using .filter iterator type and match! macro
    let male_count = people
        .iter()
        .filter(|p| matches!(p.gender, Gender::Male))
        .count();
    
    let female_count = people
        .iter()
        .filter(|p| matches!(p.gender, Gender::Female))
        .count();
    
    println!("Males: {}", male_count);
    println!("Females: {}", female_count);

    // Adult females only 
    let adult_females: Vec<&Person> = people
        .iter()
        .filter(|p| matches!(p.gender, Gender::Female) && p.is_adult())
        .collect(); // collect? why not .push() here?
    
    for person in &adult_females {
        println!("{} is an adult female.", person.name);
    }

    // pattern matching for enums that carry data
    let non_binary: Vec<&Person> = people
        .iter()
        .filter(|p| matches!(p.gender, Gender::NonBinary(_)))
        .collect();
    
    for person in &non_binary {
        println!("{} identifies as {}", person.name, person.gender_label());
    }
}