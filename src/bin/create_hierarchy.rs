use std::path::Path;
use chrono::NaiveDate;
use hierarchy::prelude::*;

fn main() {
    let start_date = NaiveDate::from_ymd_opt(1987, 3, 27).expect("Need a valid date");
    let end_date = NaiveDate::from_ymd_opt(2030, 3, 27).expect("Need a valid date");
    let cml = Cml::new(1, start_date, end_date);

    let component = ComponentBuilder::new()
        .id(02)
        .cmls(vec![cml])
        .build();

    let asset = AssetBuilder::new()
        .id(03)
        .components(vec![component])
        .build();

    let unit = UnitBuilder::new()
        .id(04)
        .assets(vec![asset])
        .build();

    println!("Our unit hierarchy...");
    let json = serde_json::to_string_pretty(&unit).unwrap();
    println!("{}", json);
        
    std::fs::write(
        Path::new("./output.json"),
        json
    )
    .unwrap();
}
