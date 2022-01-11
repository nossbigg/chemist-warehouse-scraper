use super::structs::MyItem;
use crate::common::utils::create_output_dir;
use std::fs::File;
use std::path::Path;

pub fn write_csv(items: Vec<MyItem>, category_id: &str) {
    create_output_dir();

    let filename = format!("catalog_{}.csv", category_id);
    let target = Path::join(Path::new("output"), filename);

    let file = File::create(target).unwrap();
    let mut wtr = csv::Writer::from_writer(file);

    for item in items {
        match wtr.serialize(&item) {
            _ => (),
        };
    }
    match wtr.flush() {
        _ => (),
    };
}
