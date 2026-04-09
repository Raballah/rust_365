// Day 8 - Ownership
// mini project - Onwership playground
// A small program to demonstrate move, clone, ownership transfer via function
// Before day 9, address some issues I have commented out on this code.
fn football(game: String) {
    println!("Some hate {}, but it's still popular globally", game);
}

// Return ownership
fn give_back(game: String) -> String {
    game
}

fn main() {
    let game = String::from("soccer");
    let favourite_game = game; 

    // println!("I like {}.", game); error because value borrowed after move.

    let hated_game = favourite_game.clone(); // explicit copy of 'soccer' value from favouriate_game variable

    println!("Favourite game: {}", favourite_game); // valid print, so println! macros just implements Display, does not take ownership of a variable value, right?

    football(hated_game.clone());

    // println!("Are you sure {} is a hated game?", hated_game); // fails, as hated_game ownership moved to function football fn
    // solution with clone(), deep copy
    println!("Are you sure {} is a hated game?", hated_game);

    // Returning ownership

    let game_giver = give_back(favourite_game);

    println!("{} is now available!", game_giver);

    /*let s = String::from("Rust");
    let len = calculate_length(s); // fails, no formula specified. out of scope

    println!("{}", s); */
}