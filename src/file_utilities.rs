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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_walk_to_find_and_update_file_pathnames_single_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_path_buf();
        let mut file_pathnames: Vec<PathBuf> = Vec::new();
        walk_to_find_and_update_file_pathnames(&temp_file_path, &mut file_pathnames).unwrap();
        assert_eq!(file_pathnames.len(), 1);
    }

    #[test]
    fn test_walk_to_find_and_update_file_pathnames_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        let mut file_pathnames: Vec<PathBuf> = Vec::new();
        walk_to_find_and_update_file_pathnames(&temp_dir_path, &mut file_pathnames).unwrap();
        assert_eq!(file_pathnames.len(), 0);
    }

    #[test]
    fn test_walk_to_find_and_update_file_pathnames_dir_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        for i in 0..3 {
            let temp_file_path = temp_dir_path.join(format!("temp_file_{}", i));
            File::create(&temp_file_path).unwrap();
        }
        let mut file_pathnames: Vec<PathBuf> = Vec::new();
        walk_to_find_and_update_file_pathnames(&temp_dir_path, &mut file_pathnames).unwrap();
        assert_eq!(file_pathnames.len(), 3);
    }

    #[test]
    fn test_walk_to_find_and_update_file_pathnames_dir_with_subdir_and_files() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        for i in 0..2 {
            let temp_file_path = temp_dir_path.join(format!("temp_file_{}", i));
            File::create(&temp_file_path).unwrap();
            let temp_subdir_path = temp_dir_path.join(format!("temp-subdir-{}", i));
            fs::create_dir(&temp_subdir_path).unwrap();
            for i in 0..3 {
                let temp_file_path = temp_subdir_path.join(format!("temp_file_{}", i));
                File::create(&temp_file_path).unwrap();
            }
        }
        let mut file_pathnames: Vec<PathBuf> = Vec::new();
        walk_to_find_and_update_file_pathnames(&temp_dir_path, &mut file_pathnames).unwrap();
        assert_eq!(file_pathnames.len(), 8);
    }
}
