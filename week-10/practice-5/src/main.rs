fn main() {
    let x = vec![100, 200, 300];
    borrrow_vector(&x);

    println!("Printing the value from the main() x[0]= {}", x[0]);
}

fn borrrow_vector(z: &Vec<i32>) {
    println!("********************************");
    println!("Inside the print_vector function {:?}\n", z);
    println!("********************************",);
}
