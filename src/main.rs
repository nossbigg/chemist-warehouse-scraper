// exports
pub mod common;

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
    let command = command_option.unwrap_or_default();

    if command == "catalogs" {
        let max_catalogs_depth_str = args.nth(0).unwrap_or("1".to_string());
        handle_catalogs(max_catalogs_depth_str);
    }

    if command == "catalog" {
        let category_id = args.nth(0).unwrap_or_default();
        handle_catalog(category_id);
    }
}
