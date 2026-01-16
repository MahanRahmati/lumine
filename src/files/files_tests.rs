use std::fs;

use crate::files::errors::*;
use crate::files::operations::*;

const TEST_FILE_CONTENT: &str = "This is test content for file operations.";

#[test]
fn test_remove_file() {
  let temp_dir = std::env::temp_dir();
  let test_file = temp_dir.join("test_remove_file.txt");

  fs::write(&test_file, TEST_FILE_CONTENT).unwrap();
  assert!(test_file.exists());

  let result = remove_file(&test_file.to_string_lossy());
  assert!(result.is_ok());
  assert!(!test_file.exists());
}

#[test]
fn test_remove_nonexistent_file() {
  let temp_dir = std::env::temp_dir();
  let nonexistent_file = temp_dir.join("nonexistent.txt");

  let result = remove_file(&nonexistent_file.to_string_lossy());
  assert!(result.is_err());
  match result.unwrap_err() {
    FileError::FileRemove(_) => (),
    _ => panic!("Expected FileRemove error"),
  }
}

#[test]
fn test_create_directory_all() {
  let temp_dir = std::env::temp_dir();
  let test_dir = temp_dir
    .join("test_create_directory_all")
    .join("nested")
    .join("path");

  let result = create_directory_all(&test_dir.to_string_lossy());
  assert!(result.is_ok());
  assert!(test_dir.exists());

  fs::remove_dir_all(temp_dir.join("test_create_directory_all")).unwrap();
}

#[test]
fn test_create_directory_invalid_path() {
  let invalid_path = "/root/nonexistent/invalid/path";

  let result = create_directory_all(invalid_path);
  assert!(result.is_err());
  match result.unwrap_err() {
    FileError::DirectoryCreate(_) => (),
    _ => panic!("Expected DirectoryCreate error"),
  }
}

#[test]
fn test_file_exists() {
  let temp_dir = std::env::temp_dir();
  let test_file = temp_dir.join("test_file_exists.txt");

  fs::write(&test_file, TEST_FILE_CONTENT).unwrap();
  assert!(file_exists(&test_file.to_string_lossy()));

  fs::remove_file(&test_file).unwrap();
  assert!(!file_exists(&test_file.to_string_lossy()));
}

#[test]
fn test_validate_file_exists() {
  let temp_dir = std::env::temp_dir();
  let test_file = temp_dir.join("test_validate_file_exists.txt");

  fs::write(&test_file, TEST_FILE_CONTENT).unwrap();
  assert!(validate_file_exists(&test_file.to_string_lossy()).is_ok());

  fs::remove_file(&test_file).unwrap();
  assert!(validate_file_exists(&test_file.to_string_lossy()).is_err());
}

#[test]
fn test_read_to_string() {
  let temp_dir = std::env::temp_dir();
  let test_file = temp_dir.join("test_read_to_string.txt");

  fs::write(&test_file, TEST_FILE_CONTENT).unwrap();
  let result = read_to_string(&test_file.to_string_lossy());
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), TEST_FILE_CONTENT);

  fs::remove_file(&test_file).unwrap();
  assert!(read_to_string(&test_file.to_string_lossy()).is_err());
}
