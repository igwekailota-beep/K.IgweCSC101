use std::io;

fn main() {
    println!("Employee Incentive Calculator");

    // ---- Input experience ----
    let mut experience = String::new();
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // ---- Input age ----
    let mut age_input = String::new();
    println!("Enter the employee's age: ");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid age entered!");
            return;
        }
    };

    // ---- Determine incentive ----
    let incentive: i32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // For experienced employees between 28 and 30 (not explicitly stated)
            incentive = 1_000_000; // you can decide any suitable value
        }
    } else {
        incentive = 100_000;
    }

    // ---- Output result ----
    println!(
        "The annual incentive for this employee is ₦{}",
        incentive
    );
}