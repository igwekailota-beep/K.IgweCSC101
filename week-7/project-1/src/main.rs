use std::io;
use std::f64::consts::PI; // Importing PI for the cylinder calculation

fn main() {
    println!("\nWelcome to the MTH 101 Calculator!");
    println!("\nPlease select the calculation you wish to perform:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    // Get the user's menu choice
    let choice = get_input("Enter your choice (1-5): ");

    // Perform action based on choice
    if choice == 1.0 {
        calculate_trapezium();
    } else if choice == 2.0 {
        calculate_rhombus();
    } else if choice == 3.0 {
        calculate_parallelogram();
    } else if choice == 4.0 {
        calculate_cube();
    } else if choice == 5.0 {
        calculate_cylinder();
    } else {
        println!("Invalid selection. Please restart the program and choose 1-5.");
    }
}

// --- Helper Function ---
// A function to prompt for input and return a floating-point number
fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    
    let mut input_string = String::new();
    
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
        
    // Convert the string to a number (f64), handling potential errors
    let number: f64 = input_string.trim().parse().expect("Please enter a valid number");
    return number;
}

// --- Calculation Functions ---

// Formula: height/2 * (base1 + base2)
fn calculate_trapezium() {
    println!("\nArea of Trapezium ");
    let height = get_input("Enter height:");
    let base1 = get_input("Enter base 1:");
    let base2 = get_input("Enter base 2:");
    
    let area = (height / 2.0) * (base1 + base2);
    println!("The Area of the Trapezium is: {:.2}", area);
}

// Formula: 1/2 * diagonal1 * diagonal2
fn calculate_rhombus() {
    println!("\n Area of Rhombus ");
    let d1 = get_input("Enter diagonal 1:");
    let d2 = get_input("Enter diagonal 2:");
    
    let area = 0.5 * d1 * d2;
    println!("The Area of the Rhombus is: {:.2}", area);
}

// Formula: base * altitude
fn calculate_parallelogram() {
    println!("\n Area of Parallelogram ");
    let base = get_input("Enter base:");
    let height = get_input("Enter height:");
    
    let area = base * height;
    println!("The Area of the Parallelogram is: {:.2}", area);
}

// Formula: 6 * (side)^2
fn calculate_cube() {
    println!("\n Area of Cube ");
    let side = get_input("Enter length of the side:");
    
    // side.powi(2) means side to the power of 2 (squared)
    let area = 6.0 * side.powf(2.0); 
    println!("The Area of the Cube is: {:.2}", area);
}

// Formula: PI * radius^2 * height
fn calculate_cylinder() {
    println!("\n--- Volume of Cylinder ---");
    let radius = get_input("Enter radius:");
    let height = get_input("Enter height:");
    
    let volume = PI * radius.powf(2.0) * height;
    println!("The Volume of the Cylinder is: {:.2}", volume);
}