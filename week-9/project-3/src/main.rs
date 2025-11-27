use std::io::Write;

fn main() {
    let sn = vec![1,2,3,4,5];
    let commissioner = vec!["Aignogun Alamba Daudu","Murtala Afeez Bendu","Okororocha Calistus Ogbonna ", "Adawale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel", "Petroleum"];
    let geo_zone = vec!["South West","North East","South South","South West","Soth East"];

    let mut file = std::fs::File::create("EFCC.txt").expect("create failed");
    let header = format!("{:5} {:<30} {:<25} {:<20}\n", "S/N","NAME OF COMMISSIONER","MINISTRY","GEOPOLITICAL ZONE");
    file.write_all(header.as_bytes()).expect("write failed");

    for i in 0..sn.len(){
        let sn_item = sn[i];
        let commissioner_item = commissioner[i];
        let ministry_item = ministry[i];
        let geo_zone_item = geo_zone[i];

        let line = format!("{:<5} {:<30} {:<25} {:<20} \n",sn_item,commissioner_item, ministry_item, geo_zone_item);

        file.write_all(line.as_bytes()).expect("File failed to write");
        

    }

     println!("EFCC Database updated\n");
     println!("File ready.");





}