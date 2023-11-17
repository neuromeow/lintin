use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use std::error::Error;
use std::{
    io::{self, BufReader},
    path::PathBuf,
};

use crate::cli::Cli;
use crate::util;

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
        println!("stdin");
        let stdin_bufreader = BufReader::new(io::stdin().lock());
        let lines = util::read_lines_from_bufreader(stdin_bufreader);
        util::parse_lines(lines);
        println!();
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
        // The processing is the same in both cases.
        for pathname in &pathnames {
            util::walk_to_find_and_update_file_pathnames(pathname, &mut file_pathnames)?;
        }
        for file_pathname in file_pathnames {
            println!("{}", file_pathname.display());
            // All errors when trying to access a file are propagated.
            let file_bufreader = util::create_file_bufreader(&file_pathname)?;
            let lines = util::read_lines_from_bufreader(file_bufreader);
            util::parse_lines(lines);
            println!()
        }
    }
    Ok(())
}
