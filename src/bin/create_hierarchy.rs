use std::path::Path;
use hierarchy::prelude::*;

fn main() {
    let cml = Cml::new(1);

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
