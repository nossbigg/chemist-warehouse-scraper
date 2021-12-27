use std::env;

mod catalogs;
use catalogs::handle_catalogs::handle_catalogs;

fn main() {
    let mut args = env::args();

    let command_option = args.nth(1);
    let command = match command_option {
        Some(v) => v,
        None => String::from(""),
    };

    if command == "catalogs" {
        handle_catalogs();
    }
}
