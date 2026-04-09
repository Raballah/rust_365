// Day 9, borrowing and references

fn change_name(naming: &mut String) {
    naming.push_str(" Smart");
}
fn main() {
    let mut name = String::from("James");
    change_name(&mut name);

    println!("Full Names: {}", name);
}