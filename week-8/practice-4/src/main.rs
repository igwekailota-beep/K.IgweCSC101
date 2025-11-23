// fn main() {
//     // Name vector
//     let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];
    
//     // Age vector
//     let age = vec![16, 17, 19, 22, 20, 21, 18, 23];
    
    
//     print!("\nAge allocation:\n");
    
//     // Loop to iterate elements in vector
//     for i in 0..age.len() {
        
//         // Iterating through i on the vector
//         println!("{} is {} years old", name[i], age[i]);

//         if age[i]  <= 18{
//             println!("You are still under 18\n");
//         }
//         else {
//             println!("You are an adult\n");
//         }
//     }
// }

fn main() {
    let numbers = vec![10, 20, 30];
// The loop gives us the VALUES: 10, 20, 30
for things in numbers {
    // No index needed! We have the item directly.
    println!("Value is {}", things);
}
}
