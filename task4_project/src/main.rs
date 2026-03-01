// Given the  array containing ages of girls: [32, 72, 86, 54, 40, 10, 89, 41, 49, 18, 30, 21, 51]
// You are to determine which age is below 40 and diplay all those ages.
// You are expected to use the .enumerate() and .iter() functions in your Rust code.
// This should clearly show how .enumerate() and .iter() work in Arrays loops.
// Note: several errors have been made in this attempt. explain then in details as well.
// println!("The second girl, who is aged 40 or below, is aged {}", young_girls[1]);
//println!("In the list of women, who are aged 40 and above, the second woman is aged {}.", super_old.len([2]));

fn age_category(ages: &[i32]) -> Vec<i32> {
    let mut old_woman = Vec::new();

    for (i, &age) in ages.iter().enumerate() {
        if age > 40 {
            old_woman.push(age);
            println!("This woman at index {} is very old at age {}", i, age);
        }
    }
    old_woman
}

fn main() {
    let ages = [32, 72, 86, 54, 40, 10, 89, 41, 49, 18, 30, 21, 51];
    let super_old = age_category(&ages);

    println!("Given here is the list of women aged 40 and above: {:?} ", super_old);
    //if I wanted to count how many women are aged above 40 in the array; i would use this approach:
    println!("The generated list of super old woman has exactly {} woman who are super old.", super_old.len());

    //if I wanted to determine the age of the woman appearing in the list, I would proceed as follows:

    if super_old.len() >= 2 {
        println!("The second woman in the list of wanawake ambao wamechapa is aged {} years old.", super_old[1]);
    }
}
