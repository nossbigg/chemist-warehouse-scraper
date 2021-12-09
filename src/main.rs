use std::env;

mod catalog;

fn main() {
    let mut args = env::args();

    let command_option = args.nth(1);
    let command = match command_option {
        Some(v) => v,
        None => String::from(""),
    };

    if command == "catalog" {
        catalog::handle_catalogs();
    }
}
