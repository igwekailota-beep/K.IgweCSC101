

use std::io;

fn main() {
    println!(" EY Global Limited: Experience Scout ");

    // 1. Create a Vector of Tuples to store candidates
    // Format: (Name, Years_of_Experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates are you interviewing?");
    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Failed to read");
    let count: u32 = count_input.trim().parse().expect("Please enter a valid number");

    // 2. Loop to get input for each candidate
    for i in 0..count {
        println!("\n--- Candidate {} ---", i + 1);
        
        // Get Name
        println!("Enter Name:");
        let mut name_input = String::new();
        io::stdin().read_line(&mut name_input).expect("Failed to read");
        let name = name_input.trim().to_string();

        // Get Experience
        println!("Enter Years of Experience:");
        let mut exp_input = String::new();
        io::stdin().read_line(&mut exp_input).expect("Failed to read");
        let experience: u32 = exp_input.trim().parse().expect("Please enter a number");

        // Add to vector
        candidates.push((name, experience));
    }

    // 3. Logic to find the highest experience
    if candidates.len() > 0 {
        // Assume the first person is the best to start
        let mut best_candidate = &candidates[0];

        // Iterate through everyone to see if anyone beats the current best
        for person in candidates.iter() {
            // person.1 is the experience (the second item in the tuple)
            if person.1 > best_candidate.1 {
                best_candidate = person;
            }
        }

        println!("       HIRING DECISION       ");
       
        println!("The candidate with the most experience is:");
        println!("Name: {}", best_candidate.0);
        println!("Experience: {} years", best_candidate.1);
       
    } else {
        println!("No candidates were entered.");
    }
}