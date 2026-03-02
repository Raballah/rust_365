// Rust slices, do not own data, borrrow data/ represent part of an array/String
// given women's ages as [82, 18, 29, 39, 49, 59, 10, 38, 71, 63, 90, 42, 17, 50, 79, 47, 27];
// your work is to return a list of women who are aged 50 or below


fn age_categorized(female_age: &[i32]) -> Vec<i32> {
    let mut junior_women = Vec::new();

    for &age in female_age {
        if age <= 50 {
            junior_women.push(age);
        }
    }
    junior_women
}


fn main() {
    let female_age = [82, 18, 29, 39, 49, 59, 10, 38, 71, 63, 90, 42, 17, 50, 79, 47, 27];
    let young_women = age_categorized(&female_age);

    println!("The list of young women in the list aged 50 or below is this: {:?}", young_women);
}