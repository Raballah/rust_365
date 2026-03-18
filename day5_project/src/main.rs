// Day 5: Loops 
// Looop Labels. Rust allows loops to be labelled, calld labelled loops. 
// Case Example: Searching Through 2D Dataset
// Say I want to search for the number 17 in an excel sheet dataset, within a range.

fn main() {
    let rope_length = [
        [78, 128, 24, 18],
        [89, 133, 28, 90],
        [109, 56, 48, 46],
        [110, 72, 83, 71],
        [107, 94, 32, 21],
    ];

    let target = 28;

    'outer: for row in 0..5 {
        for col in 0..4 {
            if rope_length[row] [col] == 28 {
                println!("Found {} at ({}, {}).", target, col, row);
                break 'outer;
            }
        }
    }

    println!("Search completed successfully!");
}