struct Laptop {
    hp: u32,
    ibm: u32,
    toshiba: u32,
    dell: u32,
}

//Implement methods for the struct
impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        (self.hp * quantity) + (self.ibm * quantity) + (self.toshiba) + (self.dell * quantity)
    }
}

fn main() {
    let laptops = Laptop {
        hp: 650000,
        ibm: 755000,
        toshiba: 550000,
        dell: 850000,
    };

    let cost = laptops.total_cost(3);

    println!(
        "Total cost for buying 3 laptops of each brands is ${}",
        cost
    );
}
