use hierarchy::prelude::*;

fn main() {
    let data = std::fs::read_to_string("./output.json")
        .expect("Unable to read file");

    let unit: Unit = serde_json::from_str(&data).unwrap();

    for asset in unit.assets.iter() {
        for component in asset.components.iter() {
            for cml in component.cmls.iter() {
                println!("CML {:?}", cml)
            }
        }
    }

}
