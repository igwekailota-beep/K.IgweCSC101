// fn main() {
//     //Using Vec::new() method
//     let v: Vec<f64> = Vec::new();
    


//     // printing the size of the vectors
//     println!("\n The length of the Vec::new is {}",v.len() );

//     // using macro
//     let v = vec!["Grace","Effiong", "Basil","Kareem"];

//     println!("\nThe length of vec macro is: {}",v.len() );
// }


// fn main() {
//     let x:i32 = 5;

//     {let x:i32 = 12;
//     assert_eq!(x,12 );  }

//        assert_eq!(x,5 );
//    println!("{}",x );
// let  (mut x, mut y)= (1,2);
// x+=2;
// assert_eq!(x,3 );
// assert_eq!(y,2 );

// println!("success");

// Destructuring assignments
// let (x,y);
// (x,..) = (3,4);
// (..,y) = (1,2);
// assert_eq!((x,y),(3,2)) ;


// println!("Success");


// let v1:u16 = 255_u16 +8_u16;
// let v2:i16 = i16::checked_add(255,8).unwrap();
// println!("{}, {}",v1,v2 );

//Looping using 'for'
// let mut sum :i32 = 0;
// for i in -3..=2{
//     sum +=i;
// }
// assert!(sum == -3);
// println!("Success");

// for c in 'a'..'z'{
//     println!("{}",c );
// }

// let x = 1u32 + 2u32;
// println!("{}",x );

// let x:u32 = 5u32;

// let y:u32={
//     let x_squared = x*x;
//     let x_cube = x_squared * x;
//     x_cube + x_squared + x
// };
// println!("Success is {}", y);

// let z:u32 = {
//     2*x
// };


//Caller Functions
// let s:i32  =sum(1,2);
// assert_eq!(s,3);
// println!("success");
// }
// fn sum (x:i32,y:i32) ->i32{
//     x+y
// }


// print();
// }

// //Replace i32 with another type
// fn print(){
//     println!("Success");
// }

// never_return();
// println!("Failed!"); 
// }
// fn never_return() -> ! {
//     // If you want to create a diverging function then use panic! so it never returns, the program crashes
//     panic!()

// let mut stone_cold: String = String::from("hell,");
// println!("Stone_Cold Says: {}", stone_cold);
// stone_cold.push_str("Yeah");
// println!("Stone_Cold says: {}",stone_cold );

// let the_slice:&str = &stone_cold;
// println!("The slice is {}",the_slice );
// }

// human_id("Emeka", 18, 173.4);
// sum(10,86);
// }
// fn human_id(name:&str,age:u32,height:f32){
//     println!("My name is {}, I am {} years old, and my height is {} cm.",name, age,height );
// }
// fn sum(a:i32,b:i32){
//     a+b;
//     println!("Sum = {}",a+b );
// }

fn main() {
    let weight:f64=36.0;
    let height:f64= 18.0;
    let bmi = calculate_bmi(weight,height);
    println!("BMI is = {}",bmi );
}
fn calculate_bmi(weight_kg:f64, height_m:f64)->f64{
  weight_kg/height_m*height_m
}