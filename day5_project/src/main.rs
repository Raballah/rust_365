// Day 5: Loops 
// loop keyword. loops forevoer until you stop it, is it using break or continue?
// loop is infinite loop. runs forever, until you stop it.
// loop keyword is a loop that counts forever, untily stopped. infinite loop. 
// stop using break

fn main() {
    let mut hundreds = 100;

    loop {
        hundreds += 100;

        println!("This is part of the hundreds: {}", hundreds);
        
        if hundreds == 700 {
            break;
        }
    }
}