fn main() {
//     let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];
//     let mut input1 = String::new();
    
//     println!("\nEnter an index value btw (0-8)");
//     std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    
//     // Index is the non negative value which is smaller than the size of the vector
//     let index: usize = input1.trim().parse().expect("Invalid input");
    
//     // Getting value at given index value
//     let ch: Option<&char> = v.get(index);
//     value(ch);
// }

// // Method to print the get value
// fn value(ch:Option<&char>) {
//     println!("Element of vector: {:?}", ch);


    let v = vec![10,20,30,40,50,60,70,80];
    let mut input1 = String::new();

    println!("Choose an index from 0-9");
    std::io::stdin().read_line(&mut input1).expect("Failed to find input");
    let result:usize = input1.trim().parse().expect("Invalid input");
    let value = v.get(result);//index 100 doesn't exist

    match value{
        Some (number) =>println!("Found the number {}",number),
        None =>println!("The box was empty! Nothing found"),
    }
}