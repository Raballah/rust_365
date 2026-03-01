// Now back to Ararys and the use of the push() function for vectors. data type, Vec<i32>, 
// created as Vec::new(); push() is a widely used function in Rust. 
// push() used for adding elements to dynamically-sized collections like Vec<i32> and String::new
// used as vector_name.push(element); push() modifies the data structures of collections; 
// hence, the variable containing the collection must be declared as mutable using mut. for example:
// push() takes ownership of the value/item/element being added to the collection. Example usage of push()
// push() takes ownership of the value / elemnent/value beinga dded to the collection, the Vector or String
// to remove and return the last element added to a collection using push(), use pop()
// real-life use-case of the push() function with Vector and String collections.

//Processing user input.
// A common use case is collecting user input in a loop to build a list, 
// such as adding command line arguments to a list of tasks;

fn main() {
    // 1. Initialize an empty, mutable vector
    let mut task_list: Vec<String> = Vec::new();
    
    // 2. Simulate user input
    let input1 = String::from("Buy milk");
    let input2 = String::from("Graze cows");
    let input3 = String::from("Meet the Dean");
    let input4 = String::from("Go home to my family!");

    // 3. Push data into the vector
    task_list.push(input1);
    task_list.push(input2);
    task_list.push(input3);
    task_list.push(input4);

    // 4. Output the result
    println!("These are the tasks I will taking part in today: {:?}", task_list);
}