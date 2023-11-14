use clap::Parser;
use std::path::PathBuf;

/// lintin
#[derive(Parser)]
struct Cli {
    /// The path to the file to check, use - to read from stdin (must not be a tty)
    file: PathBuf,
}
