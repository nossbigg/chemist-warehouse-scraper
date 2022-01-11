use super::structs::MyNode;
use crate::common::utils::create_output_dir;
use std::fs::File;
use std::path::Path;

pub fn write_csv(nodes: Vec<MyNode>) {
    create_output_dir();

    let filename = "catalogs.csv";
    let target = Path::join(Path::new("output"), filename);

    let file = File::create(target).unwrap();
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
