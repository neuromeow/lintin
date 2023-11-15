use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::error::Error;
use std::{
    io::{stdin, BufReader},
    path::PathBuf,
};

use crate::cli::Cli;
use crate::util;

fn parse_lines(_lines: Vec<String>) {
    println!("Lines parsed.");
}

fn walk_to_find_file_pathnames(
    file_or_dir: &PathBuf,
    file_pathnames: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if file_or_dir.is_dir() {
        for entry in std::fs::read_dir(file_or_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_to_find_file_pathnames(&path, file_pathnames)?;
            } else {
                file_pathnames.push(path);
            }
        }
    } else {
        file_pathnames.push(file_or_dir.clone());
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    // The arguments passed may also include `-` to attempt to read lines from standard input.
    let files_or_dirs = args.file_or_dir;
    // Possible scenario that the argument list is empty (the number of arguments is 0) is checked by `clap`.
    if files_or_dirs.len() == 1 {
        // There can also be only one `-` argument to attempt to read lines from standard input.
        // `unwrap` never returns `panic` due to the success of the condition above.
        let file_or_dir = files_or_dirs.get(0).unwrap();
        if *file_or_dir == PathBuf::from("-") {
            // It requires that stdin is not interactive because we’re expecting input
            // that’s piped through to the program, not text that’s typed in at runtime.
            // If stdin is a tty, it outputs the help docs so that it’s clear why it doesn't work.
            if stdin().is_terminal() {
                // `unwrap` will never return `panic` because `clap` works with the help message itself.
                Cli::command().print_help().unwrap();
                std::process::exit(2);
            }
            println!("stdin");
            let lines = util::read_lines_from_bufreader(BufReader::new(stdin().lock()));
            parse_lines(lines);
            println!();
        } else {
            let mut file_pathnames: Vec<PathBuf> = Vec::new();
            walk_to_find_file_pathnames(file_or_dir, &mut file_pathnames)?;
            for file_pathname in file_pathnames {
                println!("{}", file_pathname.display());
                let file_bufreader = util::create_file_bufreader(&file_pathname)?;
                let lines = util::read_lines_from_bufreader(file_bufreader);
                parse_lines(lines);
                println!();
            }
        }
    } else {
        for file_or_dir in &files_or_dirs {
            // The `-` argument to attempt to read lines from standard input must not be allowed along with other arguments.
            if *file_or_dir == PathBuf::from("-") {
                Cli::command()
                    .error(
                        ErrorKind::ArgumentConflict,
                        "argument -: not allowed with argument FILE_OR_DIR",
                    )
                    .exit();
            }
        }
        let mut file_pathnames: Vec<PathBuf> = Vec::new();
        for file_or_dir in &files_or_dirs {
            walk_to_find_file_pathnames(file_or_dir, &mut file_pathnames)?;
        }
        for file_pathname in file_pathnames {
            println!("{}", file_pathname.display());
            let file_bufreader = util::create_file_bufreader(&file_pathname)?;
            let lines = util::read_lines_from_bufreader(file_bufreader);
            parse_lines(lines);
            println!()
        }
    }
    Ok(())
}
