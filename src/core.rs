use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::error::Error;
// use std::path::Path;
use std::{
    // fs::File,
    io::{stdin, BufReader},
    path::PathBuf,
};

use crate::cli::Cli;
use crate::util;

fn parse_lines_from_file(_lines_from_file: Vec<String>) {
    println!("Lines parsed.");
}

fn walk_to_find_files(
    file_or_dir_pathname: &PathBuf,
    files_to_check: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if file_or_dir_pathname.is_dir() {
        for entry in std::fs::read_dir(file_or_dir_pathname)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_to_find_files(&path, files_to_check)?;
            } else {
                files_to_check.push(path);
            }
        }
    } else {
        files_to_check.push(file_or_dir_pathname.clone());
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    // It could also just be `-` to start the process of reading lines from stdin.
    let files_or_dirs_to_check = args.file_or_dir;
    // Possible scenario that the argument list is empty (the number of arguments is 0) is checked by `clap`.
    if files_or_dirs_to_check.len() == 1 {
        // `unwrap` never returns `panic` for the reason above.
        let file_or_dir_to_check = files_or_dirs_to_check.get(0).unwrap();
        if *file_or_dir_to_check == PathBuf::from("-") {
            // It requires that stdin is not interactive because we’re expecting input
            // that’s piped through to the program, not text that’s typed in at runtime.
            // If stdin is a tty, it outputs the help docs so that it’s clear why it doesn't work.
            if stdin().is_terminal() {
                // `unwrap` will never return `panic` because `clap` works with the help message itself.
                Cli::command().print_help().unwrap();
                std::process::exit(2);
            }
            println!("stdin");
            let lines_from_stdin = util::read_lines_from_buf_reader(BufReader::new(stdin().lock()));
            parse_lines_from_file(lines_from_stdin);
            println!();
        } else {
            // println!("{:?}", file_or_dir_to_check);
            let mut files_to_check: Vec<PathBuf> = Vec::new();
            walk_to_find_files(file_or_dir_to_check, &mut files_to_check)?;
            // println!("{:?}", files_to_check);
            for file_to_check_pathname in files_to_check {
                println!("{}", file_to_check_pathname.display());
                let file_bufreader = util::create_file_bufreader(&file_to_check_pathname)?;
                let lines_from_file = util::read_lines_from_buf_reader(file_bufreader);
                parse_lines_from_file(lines_from_file);
                println!();
            }
        }
    } else {
        for file_or_dir_to_check in &files_or_dirs_to_check {
            // The `-` argument to read lines from stdin must not be allowed along with other arguments.
            if *file_or_dir_to_check == PathBuf::from("-") {
                Cli::command()
                    .error(
                        ErrorKind::ArgumentConflict,
                        "argument -: not allowed with argument FILE_OR_DIR",
                    )
                    .exit();
            }
        }
        let mut files_to_check: Vec<PathBuf> = Vec::new();
        for file_or_dir_to_check in &files_or_dirs_to_check {
            walk_to_find_files(file_or_dir_to_check, &mut files_to_check)?;
        }
        for file_to_check_pathname in files_to_check {
            println!("{}", file_to_check_pathname.display());
            let file_bufreader = util::create_file_bufreader(&file_to_check_pathname)?;
            let lines_from_file = util::read_lines_from_buf_reader(file_bufreader);
            parse_lines_from_file(lines_from_file);
            println!()
        }
    }
    Ok(())
}
