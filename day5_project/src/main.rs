// Day 5: Loops 
// Looop Labels. Rust allows loops to be labelled, calld labelled loops. 

fn main() {
    let mut _count = 0;

    'outer: loop {
        println!("Outer loop!");

        loop {
            println!("Inner loop");
            
            loop {
                println!("Will this work, and why?");
                break 'outer;
            }
        }
    }

    println!("Done looping for now!");
}