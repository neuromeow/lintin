use clap::error::ErrorKind;
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

fn parse_lines_from_file(_lines_from_file: Vec<String>) {
    println!("Lines parsed.");
}

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
        parse_lines_from_file(lines_from_file);
    } else {
        for file_to_check in &files_to_check {
            if *file_to_check == PathBuf::from("-") {
                let mut cmd = Cli::command();
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "argument -: not allowed with argument FILE_OR_DIR",
                )
                .exit();
            }
        }
        // If everything is fine with the arguments
        for file_to_check in files_to_check {
            println!("{:?}", file_to_check);
            let lines_from_file = util::read_lines_from_buf_reader(BufReader::new(
                File::open(file_to_check.clone()).unwrap(),
            ));
            parse_lines_from_file(lines_from_file);
            println!();
        }
    }
    Ok(())
}
