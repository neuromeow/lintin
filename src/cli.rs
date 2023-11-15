use clap::Parser;
use std::path::PathBuf;

/// lintin
#[derive(Parser)]
pub struct Cli {
    /// The path to the file to check, use - to read from stdin (must not be a tty)
    // pub file: PathBuf,
    // #[arg(short, long, group = "specified_lines", value_parser = parse_specified_lines, use_value_delimiter = true, value_delimiter = ',')]
    pub file_or_dir: Vec<PathBuf>,
}
