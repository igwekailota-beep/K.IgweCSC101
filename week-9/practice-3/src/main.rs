fn main() {
    std::fs::remove_file("Files/information.txt").expect("could not remove the file");
    println!("file is removed");
}