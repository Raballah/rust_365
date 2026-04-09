// Day 8 - Ownership
// mini project - Onwership playground
// A small program to demonstrate move, clone, ownership transfer via function

fn football(game: String) {
    println!("Some hate {}, but it's still popular globally", game);
}

fn main() {
    let game = String::from("soccer");
    let favourite_game = game;
    let hated_game = favourite_game.clone();
    
    football(hated_game);
}