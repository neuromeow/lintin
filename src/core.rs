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
    let files_to_check = args.file_or_dir;
    // Possible scenario that the argument list is empty (the number of arguments is 0) is checked by clap
    if files_to_check.len() == 1 {
        let file_to_check = files_to_check.get(0).unwrap();
        let lines_from_file = if *file_to_check == PathBuf::from("-") {
            if stdin().is_terminal() {
                Cli::command().print_help().unwrap();
                std::process::exit(2);
            }
            util::read_lines_from_buf_reader(BufReader::new(stdin().lock()))
        } else {
            util::read_lines_from_buf_reader(BufReader::new(File::open(file_to_check).unwrap()))
        };
        for line in lines_from_file {
            println!("{}", line);
        }
    } else {
        println!("Several arguments are present.");
    }
    Ok(())
}
