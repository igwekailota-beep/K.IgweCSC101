// fn main() {
//     let b: (i32, bool, f64) = (30, true, 4.9);
//     print(b);
// }

// fn print(x: (i32, bool, f64)) {
//     println!("Inside print method");
//     // Assigns a tuple to distinct variables
//     let (age, is_male, cgpa) = x;
//     println!("Age is {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
// }

fn main() {
    let mut _x:i32 = 5;
    let _r = &mut _x;
    *_r +=1;
    *_r -=3;

    println!("the result is {}",_r );
}