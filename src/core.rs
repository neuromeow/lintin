use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::error::Error;
use std::{
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::cli::Cli;
use crate::util;

fn validate_inventory_and_print_result<R: BufRead>(reader: R, source: Option<&Path>) {
    let validation_errors = util::validate_inventory(reader);
    if !validation_errors.is_empty() {
        match source {
            Some(path) => println!("{}", path.display()),
            None => println!("stdin"),
        }
        for error in validation_errors {
            println!("{}", error);
        }
        println!();
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    // Paths to existing files or directories are expected to be passed as arguments.
    // The arguments passed may also include `-` to attempt to read lines from standard input.
    let pathnames = args.file_or_dir;
    // The first condition handles a scenario where only one `-` argument is passed to attempt to read lines from standard input.
    // Possible scenario that the argument list is empty (the number of arguments is 0) is checked by `clap`.
    // `unwrap` never returns `panic` because the scenario described above is always respected.
    if pathnames.len() == 1 && *pathnames.get(0).unwrap() == PathBuf::from("-") {
        // It requires that stdin is not interactive because we’re expecting input
        // that’s piped through to the program, not text that’s typed in at runtime.
        // If stdin is a tty, it outputs the help docs so that it’s clear why it doesn't work.
        if io::stdin().is_terminal() {
            // `unwrap` never returns `panic` because `clap` itself works with a help message.
            Cli::command().print_help().unwrap();
            std::process::exit(2);
        }
        let stdin_bufreader = BufReader::new(io::stdin().lock());
        validate_inventory_and_print_result(stdin_bufreader, None);
    } else {
        // The `-` argument to attempt to read lines from standard input must not be allowed along with other arguments.
        if pathnames
            .iter()
            .any(|pathname| *pathname == PathBuf::from("-"))
        {
            Cli::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    "argument -: not allowed with argument FILE_OR_DIR",
                )
                .exit();
        }
        // A list of all paths to all files passed in arguments and files contained in directory paths passed in arguments.
        let mut file_pathnames: Vec<PathBuf> = Vec::new();
        // The number of arguments can be one or more for the current conditional branch.
        // The validating is the same in both cases.
        for pathname in &pathnames {
            util::walk_to_find_and_update_file_pathnames(pathname, &mut file_pathnames)?;
        }
        for file_pathname in file_pathnames {
            // All errors when trying to access a file are propagated.
            let file_bufreader = util::create_file_bufreader(&file_pathname)?;
            validate_inventory_and_print_result(file_bufreader, Some(&file_pathname));
        }
    }
    Ok(())
}
