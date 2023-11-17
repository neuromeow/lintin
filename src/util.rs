use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn walk_to_find_and_update_file_pathnames(
    pathname: &PathBuf,
    file_pathnames: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if pathname.is_dir() {
        for entry_result in fs::read_dir(pathname)? {
            let entry = entry_result?;
            let entry_pathname = entry.path();
            if entry_pathname.is_dir() {
                walk_to_find_and_update_file_pathnames(&entry_pathname, file_pathnames)?;
            } else {
                file_pathnames.push(entry_pathname);
            }
        }
    } else {
        file_pathnames.push(pathname.clone());
    }
    Ok(())
}

pub fn create_file_bufreader(file_pathname: &PathBuf) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_pathname)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
}

pub fn validate_inventory<R: BufRead>(reader: R) -> Vec<String> {
    let mut found_errors = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(_text) => {
                // As a stub for future checks. If an error occurs, add its description to `found_errors`.
                // You can use `index` here which contains the line number.
            }
            Err(err) => {
                // If there was an error reading a line, add its description along with line number to `found_errors`.
                found_errors.push(format!("Error reading line {}: {}", index + 1, err));
            }
        }
    }
    found_errors
}
