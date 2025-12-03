use std::io::Write;

fn main() {
    let larger = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Guilder",
        "Heineken",
        "Star",
    ];
    let stout = vec!["Legend", "Turbo King", "Williams", "", "", ""];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "", ""];
    println!(" Welcome to Nigeria Brewery Plc. ");

    let mut file = std::fs::File::create("Nigeria_Brewery.txt").expect("create failed");
    let header = format!("{:<20} {:<20} {:<20}\n", "Larger", "Stout", "Non-alcoholic");
    file.write_all(header.as_bytes()).expect("write failed");

    for i in 0..larger.len() {
        let larger_item = larger[i];

        let stout_item = stout[i];

        let non_alcoholic_item = non_alcoholic[i];

        // let stout_item = if i < stout.len(){stout[i]}
        // else{""};

        // let non_alcoholic_item = if i < non_alcoholic.len(){non_alcoholic[i]}
        // else{""};

        let line = format!(
            "{:<20} {:<20} {:<20} \n",
            larger_item, stout_item, non_alcoholic_item
        );

        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("Data saved successfully to Nigeria_brewey.txt");
}
