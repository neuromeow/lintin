use clap::Parser;
use std::path::PathBuf;

/// lintin
#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// The path to the file to check, use - to read from stdin (must not be a tty)
    pub file_or_dir: Vec<PathBuf>,
}
