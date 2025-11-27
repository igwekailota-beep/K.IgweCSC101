use std::io::Write;

fn main() {
    // 1. Define the data using an ARRAY of Tuples
    // Notice: We removed the 'vec!' macro. Just square brackets [] are used.
    let students = [
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    println!("PAU SMIS - Student Data ");
    
    // 2. Create the file
    let mut file = std::fs::File::create("pau_smis.txt").expect("create failed");

    // 3. Write the Header Row
    // We use formatted spacing {:<20} to create neat columns
    file.write_all("Student Name\tMatric. Number\tDepartment\tLevel\n".as_bytes())
        .expect("write failed");


    // 4. Loop through the ARRAY
    for student in students {
        // Destructure the tuple for readability
        let name = student.0;
        let matric = student.1;
        let dept = student.2;
        let level = student.3;

        // Create the formatted string line
        let line = format!("{:<10}{:<10}{:10}{:10}\n", name, matric, dept, level);

        // Display to terminal (optional, just to see it running)
        print!("Saving: {}", line);

        // Write to file
        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("\nData successfully saved to pau_smis.txt");
}