use super::structs::MyNode;
use std::fs::File;

pub fn write_csv(nodes: Vec<MyNode>) {
    let filename = format!("catalogs.csv");
    let file = File::create(filename).unwrap();

    let mut wtr = csv::Writer::from_writer(file);

    for node in nodes {
        match wtr.serialize(&node) {
            _ => (),
        };
    }
    match wtr.flush() {
        _ => (),
    };
}
