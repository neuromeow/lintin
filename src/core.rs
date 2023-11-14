use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::error::Error;
use std::{
    fs::File,
    io::{stdin, BufReader},
    path::PathBuf,
};

use crate::cli::Cli;
use crate::util;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let file = args.file;
    let lines_from_file = if *file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            std::process::exit(2);
        }
        util::read_lines_from_buf_reader(BufReader::new(stdin().lock()))
    } else {
        util::read_lines_from_buf_reader(BufReader::new(File::open(file).unwrap()))
    };
    for line in lines_from_file {
        println!("{}", line);
    }
    Ok(())
}
