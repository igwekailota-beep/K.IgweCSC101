fn add_one(e: &mut i32) {
    let e = *e += 1;
}

fn main() {
    let mut i: i32 = 3;
    add_one(&mut i);
    println!("{}", i);
}
