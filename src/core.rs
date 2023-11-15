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
    // Possible scenario that the argument list is empty (the number of arguments is 0) is checked by `clap`.
    if files_to_check.len() == 1 {
        // `unwrap` never returns `panic` for the reason above.
        let file_to_check = files_to_check.get(0).unwrap();
        let lines_from_file = if *file_to_check == PathBuf::from("-") {
            // It requires that stdin is not interactive because we’re expecting input
            // that’s piped through to the program, not text that’s typed in at runtime.
            // If stdin is a tty, it outputs the help docs so that it’s clear why it doesn't work.
            if stdin().is_terminal() {
                // `unwrap` will never return `panic` because `clap` works with the help message itself.
                Cli::command().print_help().unwrap();
                std::process::exit(2);
            }
            println!("stdin");
            util::read_lines_from_buf_reader(BufReader::new(stdin().lock()))
        } else {
            println!("{:?}", file_to_check);
            util::read_lines_from_buf_reader(BufReader::new(File::open(file_to_check).unwrap()))
        };
        parse_lines_from_file(lines_from_file);
        println!();
    } else {
        for file_to_check in &files_to_check {
            // The `-` argument to read lines from stdin must not be allowed along with other arguments.
            if *file_to_check == PathBuf::from("-") {
                Cli::command()
                    .error(
                        ErrorKind::ArgumentConflict,
                        "argument -: not allowed with argument FILE_OR_DIR",
                    )
                    .exit();
            }
        }
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
