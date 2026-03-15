// Day 5: Loops 
// While loop
// Building a multi-line text buffer 

use std::io;

fn main() {
    let mut document = String::new();
    let mut line = String::new();

    println!("Type 'done' to exit!:");

    loop {
        line.clear();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        if line.trim() == "done" {
            break;
        }

        document.push_str(&line);
    }

    println!("Document progress: \n{}", document);
}