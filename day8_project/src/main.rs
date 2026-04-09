// Day 9, borrowing and references

fn print_value(text: &str) {
    println!("{}", text);
}
fn main() {
    let s = String::from("Hello");
    print_value(&s);
    print_value("Hello");

    println!("{}", s);
}