mod cli;
mod core;
mod util;

const EXIT_CODE_ERROR: i32 = 1;

fn main() {
    if let Err(error) = core::run() {
        eprintln!("{}", error);
        std::process::exit(EXIT_CODE_ERROR);
    }
}
