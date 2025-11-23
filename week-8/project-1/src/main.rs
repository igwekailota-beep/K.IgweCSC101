use std::io;

fn main() {
    println!("Public Service APS Level Checker ");
    

    // 1. Create a Vector of Tuples to act as the database
    // Format: (Job Title, Public Servant Level)
    let mut staff_db: Vec<(&str, &str)> = Vec::new();

    //[span_0](start_span)// 2. Populate the database with ALL data from the table[span_0](end_span)
    
    // APS 1-2 Data
    staff_db.push(("Intern", "APS 1-2"));
    staff_db.push(("Paralegal", "APS 1-2"));
    staff_db.push(("Placement", "APS 1-2"));

    // APS 3-5 Data
    staff_db.push(("Administrator", "APS 3-5"));
    staff_db.push(("Research Assistant", "APS 3-5"));
    staff_db.push(("Junior Associate", "APS 3-5"));
    staff_db.push(("Classroom Teacher", "APS 3-5"));

    // APS 5-8 Data
    staff_db.push(("Senior Administrator", "APS 5-8"));
    staff_db.push(("PhD Candidate", "APS 5-8"));
    staff_db.push(("Associate", "APS 5-8"));
    staff_db.push(("Snr Teacher", "APS 5-8"));

    // EL1 8-10 Data
    staff_db.push(("Office Manager", "EL1 8-10"));
    staff_db.push(("Post-Doc Researcher", "EL1 8-10"));
    staff_db.push(("Senior Associate 1-2", "EL1 8-10"));
    staff_db.push(("Leading Teacher", "EL1 8-10"));

    // EL2 10-13 Data
    staff_db.push(("Director", "EL2 10-13"));
    staff_db.push(("Senior Lecturer", "EL2 10-13"));
    staff_db.push(("Senior Associate 3-4", "EL2 10-13"));
    staff_db.push(("Deputy Principal", "EL2 10-13"));

    // SES Data
    staff_db.push(("CEO", "SES"));
    staff_db.push(("Dean", "SES"));
    staff_db.push(("Partner", "SES"));
    staff_db.push(("Principal", "SES"));


    // 3. Ask User for Input
    println!("\nEnter a Job Title (e.g., 'CEO', 'Paralegal', 'Dean'):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let user_title = input.trim();

    // 4. Search the Vector for a match
    let mut found = false;

    // We iterate through the tuples. 
    // 'row.0' is the Title, 'row.1' is the Level.
    for row in staff_db.iter() {
        if row.0.eq_ignore_ascii_case(user_title) {
            println!("\n Validation Successful ");
            println!("Staff Title: {}", row.0);
            println!("Staff Level: {}", row.1);
            found = true;
            break; // Stop the loop once found
        }
    }

    if !found {
        println!("\nError: The job title '{}' is not in the database.", user_title);
    }
}
