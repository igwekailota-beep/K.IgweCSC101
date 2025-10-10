fn main() {
    // Define the sales data: (Item, Quantity, Amount)
    let sales = [
        ("Toshiba", 2, 450_000.0),
        ("Mac", 1, 1_500_000.0),
        ("HP", 3, 750_000.0),
        ("Dell", 3, 2_850_000.0),
        ("Acer", 1, 250_000.0),
    ];

    let mut total_sales = 0.0;
    let mut total_quantity = 0;

    // Calculate total sales and total quantity
    for &(_item, qty, amount) in sales.iter() {
        let item_total = (qty as f64) * amount;
        total_sales += item_total;
        total_quantity += qty;
    }

     // Compute average per item sold
    let average_sales = total_sales / (total_quantity as f64);

    // Display the results
    println!("Chief Donatus and Sons Ltd Sales Summary");
    println!("Total Sales Amount: ₦{:.2}", total_sales);
    println!("Average Sales per Item: ₦{:.2}", average_sales);
}



