use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
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
