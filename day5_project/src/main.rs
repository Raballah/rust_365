// Day 5: Loops 
// for loop. Using for with ranges.
// rust iterators using continue feature.

fn main() {
    let shoe_sizes = [44, 46, 32, 37, 45, 40, 41, 39, 38, 36, 35, 47];

    // Task: filter all shoes size above 38 into a new array, then continue if not 46

    let new_sizes: Vec<i32> = shoe_sizes
        .iter() // Borrows shows sizes as &i32
        .copied() // Converts borrowed &i32 to i32
        .filter(|&size| size > 38) // Filters by size
        .collect(); // returns the new_sizes growable vector as Vec<i32>

    for &size in &new_sizes {   // Borrows &Vec<i32> with elements &i32
        if size == 46 {       // Manually converts the borrowed &i32 to i32
            continue;
        }

        println!("Available size: {}", size);
    }

    println!("List of all available shoe sizes: {:?}", new_sizes);
}