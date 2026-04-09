// Day 8 - Ownership
// mini project - Onwership playground
// A small program to demonstrate move, clone, ownership transfer via function

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

    let hated_game = favourite_game.clone();
    
    football(hated_game);

    let game_giver = give_back(favourite_game);

    println!("{} is now available!", game_giver);
}