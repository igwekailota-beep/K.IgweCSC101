use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut input1= String::new();
    println!("Enter the new information");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let user_input:String = input1.trim().parse().expect("Invalid input");

    let mut file = OpenOptions::new().create(true).append(true).open("data.txt").expect("can't open the file");
    file.write_all("\nhello Class\n".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document\n".as_bytes()).expect("write failed");
    file.write_all(user_input.as_bytes()).expect("write failed");
    println!("file append success");
}
 