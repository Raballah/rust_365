// A Rust program to determine the min and max numbers out of a given array list.
// Ages of girls in a community study meeting: [23, 31, 18, 21, 22, 12, 28, 29, 27, 32, 17, 16, 20, 26]

fn age_categorized(girl_age: &[i32]) -> (i32, i32) {
    let mut min_age = girl_age[0];
    let mut max_age = girl_age[0];

    for &age in girl_age {
        if age < min_age {
            min_age = age;
        }
        if age > max_age {
            max_age = age;
        }
    }

    (min_age, max_age)
}

fn main() {
    let girl_age = [23, 31, 18, 21, 22, 12, 28, 29, 27, 32, 17, 16, 20, 26];
    let (min_age, max_age) = age_categorized(&girl_age);

    println!("The youngest girl is aged {}, while the oldest girl has an age of {}.", min_age, max_age);
}