use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[allow(dead_code)]
pub fn create_file_bufreader(file_pathname: &PathBuf) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_pathname)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
}

pub fn read_lines_from_bufreader<R: BufRead>(bufreader: R) -> Vec<String> {
    bufreader.lines().map(|line| line.unwrap()).collect()
}

pub fn walk_to_find_file_pathnames(
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

pub fn parse_lines(_lines: Vec<String>) {
    println!("Lines parsed.");
}
