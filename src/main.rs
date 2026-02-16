// A mini-practical task regarding mutability and type casting. 
// Analytical logic, result known so, a report-based format. 

fn percentage_calculation(today_number: i32, total: i32) ->f64 {
    if total == 0 {
        0.00 // accounting for dvision by zero (0).
    } else {
        // Cast to f64 for decimal precision.
        (today_number as f64 / total as f64) * 100.0
    }
}

fn main() {
    let today_number = 17;
    let total = 26;

    let percentage_attendance = percentage_calculation(today_number, total);

    println!("The number of students expected to attend the study session: {}", total);
    println!("Today's number of attendance: {}", today_number);
    println!("The percentage attendance today: {:.1}%", percentage_attendance);
}