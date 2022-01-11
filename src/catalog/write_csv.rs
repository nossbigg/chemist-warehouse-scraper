use super::structs::MyItem;
use std::fs::File;

pub fn write_csv(items: Vec<MyItem>, category_id: &str) {
    let filename = format!("catalog_{}.csv", category_id);
    let file = File::create(filename).unwrap();

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
