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

pub fn read_lines_from_buf_reader<R: BufRead>(buf_reader: R) -> Vec<String> {
    buf_reader.lines().map(|line| line.unwrap()).collect()
}
