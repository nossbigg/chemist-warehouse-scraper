use std::env;

mod catalogs;
use catalogs::handle_catalogs::handle_catalogs;

mod catalog;
use catalog::handle_catalog::handle_catalog;

fn main() {
    let mut args = env::args();

    // skip first arg
    args.nth(0);

    let command_option = args.nth(0);
    let command = match command_option {
        Some(v) => v,
        None => String::from(""),
    };

    if command == "catalogs" {
        handle_catalogs();
    }

    if command == "catalog" {
        let category_id = args.nth(0);
        handle_catalog(category_id.unwrap());
    }
}
