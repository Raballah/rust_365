/* // Day 9, borrowing and references
// Day 9 Mini Project - Borrowing Fix 

fn football(game: &str) {
    println!("Some hate {}, but it's still popular globally", game);
}

fn to_owned_string(game: &str) -> String {
    game.to_string()  // ensures the &str is converted to the expected String outcome.
}

fn add_suffix(text: &mut String) {
    text.push_str(" is fun.");
}

fn prepend_text(game: &mut String) {
    game.insert_str(0, "We love ");
}

fn main() {
    let mut game = String::from("soccer");
    let favourite_game = &game; // borrows 'soccer' no ownership of value taken here.

    println!("Favourite game: {}", favourite_game); // favourite_game borrows game as str, &soccer, println! macro implements Display trait to soccer 
    
    let hated_game = &game;  // borrowed as str, we have &soccer here

    football(hated_game); // fn deferences &soccer to soccer. &soccer still available because of borrow

    println!("Are you sure {} is a hated game?", hated_game); //works just fine, no ownership taken due to &str borrow

    // Returning ownership 

    let game_giver = to_owned_string(&game); // here, favourite_game is still &str, that is &soccer, the value. no ownership taken by function.

    println!("{} is now available.", game_giver); // println! automatically deferences the borrow to display soccer. no ownership taken by println! macro

    // addiing a string to game:
    add_suffix(&mut game);

    println!("Personal view on soccer: {}", game);

    // I'm assuming that beyond this scope/ point, favourite_game and hated_game are unaivalable / 
    // cannot be used further because game has been modified. Is this correct? or since we have &mut game, they still valid?

    // now, say I wanted add some text before the word 'soccer' after game has been modified in this scope. 
    // would this work? I know of the rule of not having more than 1 &mut String concurrently, 
    // but how would I go about this? or is there a way to build on an already modified string?
    
    prepend_text(&mut game);
    println!("{}", game);
} */

// Day 10, Mutable References Rules
// Only one mutable refereces &mut or several immutable references &, 
// but not both at the same time.
// Day 10 Mini Project, Fixing Borrow Errors 
/*
fn main() {
    let mut text = String::from("Rust");

    let r1 = &text;
    let r2 = &text;
    // multiple immutable borrws & allowed at the same time.

    println!("{} and {}", r1, r2); // r1 and r2 gets out of scope
    
    let r3 = &mut text;

    r3.push_str(" is awesome!");

    println!("{}", r3); // works, r3 gets out of scope, frees 'text'
} 
*/

fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn main() {
    let nums = [10, 20, 30, 40, 50];
    let result = sum(&nums[1..3]);

    println!("{}", result);
}