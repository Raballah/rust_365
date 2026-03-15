// Day 5: Loops 
// for loop. Using for with ranges.
// rust iterators using continue feature.

fn main() {
    let numbers = vec![1,2,3];

    for n in numbers.iter() { // creates an interator, which yields &i32
        println!("{}", n);  // println! macro implements Display trait, automatically handles conversion of &i32 to i32 
    }

    for n in numbers {  // Implements numbers.into_iter() interation, consumption iteration. n is i32 here, within Vec<i32)
        println!("{}", n);  // prints, but moves ownership of numbers vector to the loop iterator. numbers vector unavaiable beyond this for loop scope.
    }
}