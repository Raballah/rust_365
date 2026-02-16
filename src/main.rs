// Mutability and Type Casting.
// A mini task to simulate student study group attendance. 
// A state-changing system. 

fn main() {
    let total_attendance = 12;
    let mut attendance_today = 0; //Attendance begins at zero (0), increases progressively.

    // Iterate the attendance of students as they enter the study sessions.
    attendance_today += 1;
    attendance_today += 1;
    attendance_today += 1;
    attendance_today += 1;
    attendance_today += 1;

    // Cast the percentage calculation to f64 for decimal precision.
    let percentage = (attendance_today as f64 / total_attendance as f64) * 100.0; 
    
    println!("Expected total attendance: {}", total_attendance);
    println!("Today's total attendance: {}", attendance_today);
    println!("Percentage attendance today: {:.1}%", percentage);

}