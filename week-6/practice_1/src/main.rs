fn main() {
     println!("\n--- Practice 1: String Literal ---");
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    
    // Note: The original example had a typo: println!("University: {}, \nAddress: {}",uni,addr);
    // which is missing a variable for the first placeholder. I'll correct it to:
    println!("Name: {}\nUniversity: {}\nAddress: {}", name, uni, addr);
    
    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}\nSchool: {}",department,school);
}

   
