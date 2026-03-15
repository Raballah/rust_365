// Day 5: Loops 
// While loop
// A Basic Game Loop

fn main() {
    let mut lifetime = 100;

    loop {
        println!("Game on... Health remaining: {}", lifetime);

        lifetime -= 20;

        if lifetime <= 0 {
            println!("Game Over..0% health");
            break;
        }
    }
}