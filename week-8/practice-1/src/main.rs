fn main() {
    //Using Vec::new() method
    let v: Vec<f64> = Vec::new();
    std::io:stdin().read_line()


    // printing the size of the vectors
    println!("\n The lenght of the Vec::new is {}",v.len() );

    // using macro
    let v = vec!["Grace","Effiong", "Basil","Kareem"];

    println!("\nThe length of vec macro is: { }",v.len() );
}