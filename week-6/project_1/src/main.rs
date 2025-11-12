fn main() {
  use std::io;

fn main() {
    // Display the menu
    println!("--- MENU ---");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");
    

    let mut food_choice = String::new();
    let mut quantity_str = String::new();

    // Input type of food
    println!("\nEnter the letter for your food choice (P, F, A, E, W):");
    io::stdin()
        .read_line(&mut food_choice)
        .expect("Invalid input");
    
    // Get the first character and make it uppercase
    let food_char = food_choice.trim().to_uppercase().chars().next()
        .expect("Please enter a valid character");

    // Input quantity
    println!("Enter the quantity:");
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Invalid input");

    let quantity: f64 = quantity_str.trim().parse()
        .expect("Please enter a valid number for quantity");

    // Get price based on choice
    let price_per_item = match food_char {
        'P' => 3200.0,
        'F' => 3000.0,
        'A' => 2500.0,
        'E' => 2000.0,
        'W' => 2500.0,
        _ => {
            println!("Invalid food choice!");
            return; // Exit the program if choice is invalid
        }
    };

    // Calculate total charges
    let mut total_charges = price_per_item * quantity;
    let mut discount = 0.0;

    println!("\n--- ORDER SUMMARY ---");
    println!("Item Price: N{:.2}", price_per_item);
    println!("Quantity: {}", quantity);
    println!("Initial Total: N{:.2}", total_charges);

    // Apply discount if total is greater than N10,000
    if total_charges > 10_000.0 {
        discount = total_charges * 0.05; // 5% discount
        total_charges -= discount;
        
        println!("Discount (5%): -N{:.2}", discount);
    }

    // Display final total charges
    println!("---------------------");
    println!("Final Total Charges: N{:.2}", total_charges);
}
}

