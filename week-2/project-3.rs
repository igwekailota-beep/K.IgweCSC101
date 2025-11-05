// fn main() {
// 	let p:f64 = 510_000.00;
// 	let r:f64 = 5.0;
// 	let n:f64 = 3.0;

// 	//Compound Interest
// 	let a = p*(1.0 - (r/100.0)).powf(n);
// 	println!("The Amount is {}",a);
// 	println!("Initial value: ₦{:.2}", p);
//     println!("After {} years the value is: ₦{:.2}", n , a);


// }

// use std::io;//for input and output

// fn main() {
// 	loop {
// 		println!("Compound Interest Savings Calculator");

    
	

	
//   let p =loop{
//     	let mut input1 = String::new();
// 	println!("\nEnter the principal amount (P): ");
//     io::stdin().read_line(&mut input1).expect("Failed to read input"); //typecasting
//     let p: f64 = input1.trim().parse().expect("Failed to read input");
//     match input1.trim().parse::<f64>(){
//     	Ok(num) => break num,
//     	Err(_) => println!("Invalid input! Please enter a valid number for P.")
//     }
//   };
    
//    let r =loop{
//     	let mut input2 = String::new();
//     println!("\nEnter the annual interest rate(R) in %: ");
//     io::stdin().read_line(&mut input2).expect("Failed to read input"); //typecasting
//     let r: f64 = input2.trim().parse().expect("Failed to read input");
//      match input2.trim().parse::<f64>(){
//     	Ok(num) => break num,
//     	Err(_) => println!("Invalid input! Please enter a valid number for R.")
//     }
//    };
   
//    let t = loop{
//    	let mut input3 = String::new();
//     println!("\nEnter the time period in years: ");
//     io::stdin().read_line(&mut input3).expect("Failed to read input"); //typecasting
//     let t: f64 = input3.trim().parse().expect("Failed to read input");
//      match input3.trim().parse::<f64>(){
//     	Ok(num) => break num,
//     	Err(_) => println!("Invalid input! Please enter a valid number for T.")
//     }
//    };
    
//     //Display the results neatly
//     println!("Principal (P): {:.2}",p  );
//     println!("Rate (R): {:.2}%",r );
//     println!("Time (T): {:.2}years", t );





//     //Amount
//     let a = p*(1.0 - (r/100.0)).powf(t);
//  	println!("Total Amount: {}",a);

//     //Compound Interest
//     let ci = a - p;
//     println!("Compound Interest (CI): {:.2}", ci ); 

//     //Ask user if they want to calculate again for another customer
//     println!("Do you want to calculate for another customer ? (y/n)");
//     let mut choice = String::new();
//     io::stdin().read_line(&mut choice).expect("Failed to read input");

//     if choice.trim().eq_ignore_ascii_case("n")

//     {println!("Thank you for using Compound Interest Calculator!");
//     break;}





// 	}

// }

use std::io; // for input and output

fn main() {
    loop {
        println!("=== Compound Interest Savings Calculator ===");

        // 1️⃣ Input handling for P, R, and T with error checking
        let p = loop {
            let mut p_input = String::new();
            println!("Enter the principal amount (P): ");
            io::stdin().read_line(&mut p_input).expect("Failed to read input");
            match p_input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(_) => println!(" Invalid input! Please enter a valid number for P."),
            }
        };

        let r = loop {
            let mut r_input = String::new();
            println!("Enter the annual int+erest rate (R) in %: ");
            io::stdin().read_line(&mut r_input).expect("Failed to read input");
            match r_input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(_) => println!(" Invalid input! Please enter a valid number for R."),
            }
        };

        let t = loop {
            let mut t_input = String::new();
            println!("Enter the time period (T) in years: ");
            io::stdin().read_line(&mut t_input).expect("Failed to read input");
            match t_input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(_) => println!(" Invalid input! Please enter a valid number for T."),
            }
        };

        // Compute total amount using the formula A = P * (1 + R/100)^T
        let a = p * (1.0 + r / 100.0).powf(t);
        

        // Compute compound interest (CI = A - P)
        let ci = a - p;

        // Display results neatly
        println!("\n===== RESULTS =====");
        println!("Principal (P): {:.2}", p);
        println!("Rate (R): {:.2}%", r); 
        println!("Time (T): {:.2} years", t);
        println!("Total Amount (A): {:.2}", a);
        println!("Compound Interest (CI): {:.2}\n", ci);

        // Ask user if they want to calculate again
        println!("Do you want to calculate for another customer? (y/n): ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().to_lowercase();
        // match choice.trim().parse::<char>(){
        // 	Ok(char) => break (_char),
        //         Err(_) => println!(" Please input 'y' or 'n'"),
        // }

        // if choice.trim().eq_ignore_ascii_case("n") {
        //     println!(" Thank you for using the Compound Interest Calculator!");
        //     break;
        // }

        if choice == "y" {
        	println!("\n Enter information of next customer");
        	continue;
        }
        else if choice == "n"{
        	println!("Stoping loop...");
        	break;
        }
        else{
        	println!("Please Enter either y to Continue or n to Stop");
        	break;
        }
    }
}
