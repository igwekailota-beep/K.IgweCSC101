fn main() {
	let p:f64 = 510_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//Compound Interest
	let a = p*(1.0 - (r/100.0)).powf(n);
	println!("The Amount is {}",a);
	println!("Initial value: ₦{:.2}", p);
    println!("After {} years the value is: ₦{:.2}", n , a);


}