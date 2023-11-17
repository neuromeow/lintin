mod cli;
mod core;
mod file_utilities;
mod inventory_validator;

fn main() {
    if let Err(error) = core::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
