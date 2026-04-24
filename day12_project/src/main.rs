// Day 15 Vectors and Collections 


fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(20);
    v.push(18);
    v.push(4);
    v.push(15);

    println!("The vector has the following numbers: {:?}", v);

    if let Some(val) = v.pop() {
        println!("The new numbers in the vector are: {:?}", v);
    }
}