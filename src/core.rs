use clap::Parser;
use std::error::Error;

use crate::cli::Cli;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    println!("{:?}", args.file);
    Ok(())
}
