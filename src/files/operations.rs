use super::errors::{FileError, FileResult};
use std::path::Path;

pub fn remove_file(file_path: &str, verbose: bool) -> FileResult<()> {
  let path = Path::new(file_path);

  match std::fs::remove_file(path) {
    Ok(_) => {
      if verbose {
        println!("File removed: {}", file_path);
      }
      return Ok(());
    }
    Err(e) => {
      return Err(FileError::FileRemove(e.to_string()));
    }
  }
}

pub fn create_directory_all(dir_path: &str) -> FileResult<()> {
  match std::fs::create_dir_all(dir_path) {
    Ok(_) => return Ok(()),
    Err(e) => return Err(FileError::DirectoryCreate(e.to_string())),
  }
}

pub fn validate_file_exists(file_path: &str) -> FileResult<()> {
  if !file_exists(file_path) {
    return Err(FileError::FileNotFound(file_path.to_string()));
  }
  return Ok(());
}

pub fn file_exists(file_path: &str) -> bool {
  Path::new(file_path).exists()
}

pub fn read_to_string(file_path: &str) -> FileResult<String> {
  match std::fs::read_to_string(file_path) {
    Ok(content) => return Ok(content),
    Err(e) => return Err(FileError::FileRead(e.to_string())),
  }
}
