use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub fn walk_to_find_and_update_file_paths_list(
    path: &PathBuf,
    file_paths_list: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if path.is_dir() {
        for entry_result in fs::read_dir(path)? {
            let entry = entry_result?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                walk_to_find_and_update_file_paths_list(&entry_path, file_paths_list)?;
            } else {
                file_paths_list.push(entry_path);
            }
        }
    } else {
        file_paths_list.push(path.clone());
    }
    Ok(())
}

pub fn create_file_bufreader(file_path: &Path) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let file_bufreader = BufReader::new(file);
    Ok(file_bufreader)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_walk_to_find_and_update_file_paths_list_single_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_path_buf();
        let mut temp_file_paths_list = Vec::new();
        walk_to_find_and_update_file_paths_list(&temp_file_path, &mut temp_file_paths_list)
            .unwrap();
        assert_eq!(temp_file_paths_list.len(), 1);
    }

    #[test]
    fn test_walk_to_find_and_update_file_paths_list_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        let mut temp_file_paths_list = Vec::new();
        walk_to_find_and_update_file_paths_list(&temp_dir_path, &mut temp_file_paths_list).unwrap();
        assert_eq!(temp_file_paths_list.len(), 0);
    }

    #[test]
    fn test_walk_to_find_and_update_file_paths_list_dir_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        for i in 0..3 {
            let temp_file_path = temp_dir_path.join(format!("temp_file_{}", i));
            File::create(&temp_file_path).unwrap();
        }
        let mut temp_file_paths_list = Vec::new();
        walk_to_find_and_update_file_paths_list(&temp_dir_path, &mut temp_file_paths_list).unwrap();
        assert_eq!(temp_file_paths_list.len(), 3);
    }

    #[test]
    fn test_walk_to_find_and_update_file_paths_list_dir_with_subdirs_and_files() {
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
        let mut temp_file_paths_list = Vec::new();
        walk_to_find_and_update_file_paths_list(&temp_dir_path, &mut temp_file_paths_list).unwrap();
        assert_eq!(temp_file_paths_list.len(), 8);
    }

    #[test]
    fn test_create_file_bufreader_file_exists() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_path_buf();
        let temp_file_bufreader_result = create_file_bufreader(&temp_file_path);
        assert!(temp_file_bufreader_result.is_ok());
    }

    #[test]
    fn test_create_file_bufreader_file_does_not_exist() {
        let temp_file = NamedTempFile::new().unwrap();
        let non_existent_temp_file_path = temp_file.path().join("non_existent");
        let non_existent_temp_file_bufreader_result =
            create_file_bufreader(&non_existent_temp_file_path);
        assert!(non_existent_temp_file_bufreader_result.is_err());
    }
}
