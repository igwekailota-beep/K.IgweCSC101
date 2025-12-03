struct Rectangle {
    width: u32,
    height: u32,
}

//Logic to calculate the area of a rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        //use the operatot to fetch teh value of a field via the keyboard
        self.width * self.height
    }
}

fn main() {
    //instantiate the structure
    let small = Rectangle {
        width: 10,
        height: 20,
    };

    //print the rectangle's area
    println!(
        "\nWidth is {}\n Height is {}\n Area of rectangle is {} ",
        small.width,
        small.height,
        small.area()
    );
}
