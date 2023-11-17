mod cli;
mod core;
mod file_utilities;

fn main() {
    if let Err(error) = core::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
