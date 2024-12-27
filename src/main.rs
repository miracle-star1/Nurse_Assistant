// Import necessary libraries
use std::thread;
use std::time::Duration;
use chrono::{Local, Timelike};


fn main() {
    println!("Nurse Assistant");

    // Input: Patient name and medication time
    let mut patient_name = String::new();
    let mut medication_hour = String::new();
    let mut medication_minute = String::new();

    println!("Enter the patient's name:");
    std::io::stdin().read_line(&mut patient_name).expect("Failed to read input");
    let patient_name = patient_name.trim();

    println!("Enter the hour (24-hour format) for the medication:");
    std::io::stdin().read_line(&mut medication_hour).expect("Failed to read input");
    let medication_hour: u32 = medication_hour.trim().parse().expect("Please enter a valid number");

    println!("Enter the minute for the medication:");
    std::io::stdin().read_line(&mut medication_minute).expect("Failed to read input");
    let medication_minute: u32 = medication_minute.trim().parse().expect("Please enter a valid number");

    println!("Reminder set for {} at {:02}:{:02}.", patient_name, medication_hour, medication_minute);

    loop {
        let now = Local::now();
        if now.hour() == medication_hour && now.minute() == medication_minute {
            println!("Time to give medication to {}!", patient_name);
            break;
        }
        // Sleep for 30 seconds to prevent constant CPU usage
        thread::sleep(Duration::from_secs(30));
    }
}