use std::io::{self, Write};

fn read_f64(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(v) => return v,
            Err(_) => println!("Invalid number, please enter a valid floating-point value (e.g. 1.23)"),
        }
    }
}

fn approx_zero(x: f64) -> bool {
    x.abs() < 1e-12
}

fn main() {
    println!("Solve ax^2 + bx + c = 0 (quadratic roots)");

    let a = read_f64("Enter a: ");
    let b = read_f64("Enter b: ");
    let c = read_f64("Enter c: ");

    // Special-case: not a quadratic if a == 0
    if approx_zero(a) {
        println!("a is (approximately) 0 — the equation is not quadratic.");
        if approx_zero(b) {
            if approx_zero(c) {
                println!("0 = 0 — infinitely many solutions.");
            } else {
                println!("{} = 0 is impossible — no solution.", c);
            }
        } else {
            let x = -c / b;
            println!("Linear equation: one root: x = {}", x);
        }
        return;
    }

    let discriminant = b * b - 4.0 * a * c;
    println!("Discriminant D = {}", discriminant);

    if discriminant > 0.0 {
        let sqrt_d = discriminant.sqrt();
        let x1 = (-b + sqrt_d) / (2.0 * a);
        let x2 = (-b - sqrt_d) / (2.0 * a);
        println!("Two distinct real roots:");
        println!("x1 = {}", x1);
        println!("x2 = {}", x2);
    } else if approx_zero(discriminant) {
        let x = -b / (2.0 * a);
        println!("One real (double) root:");
        println!("x = {}", x);
    } else {
        // Complex roots
        let sqrt_neg = (-discriminant).sqrt();
        let real = -b / (2.0 * a);
        let imag = sqrt_neg / (2.0 * a);
        println!("Two complex roots:");
        println!("x1 = {} + {}i", real, imag);
        println!("x2 = {} - {}i", real, imag);
    }
}