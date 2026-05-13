// Day 16 - Strings in Depth 
// the format! approach
fn main() {
    let fake_news: &str = "he is real";
    println!("Yes, {}.", fake_news);

    let confirmation: String = fake_news.to_string();
    println!("Are you sure {}?", confirmation);

    let another_fake: &str = &confirmation;
    println!("He thinks that {}.", another_fake);
}