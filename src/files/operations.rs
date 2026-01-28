use std::path::Path;

use crate::files::errors::{FileError, FileResult};

/// Removes a file from the filesystem.
///
/// # Arguments
///
/// * `file_path` - The path to the file to be removed
///
/// # Returns
///
/// A `FileResult<()>` indicating success or failure.
pub async fn remove_file(file_path: &str) -> FileResult<()> {
  let path = Path::new(file_path);
  return tokio::fs::remove_file(path)
    .await
    .map_err(|e| FileError::FileRemove(e.to_string()));
}

/// Creates a directory and all parent directories if they don't exist.
///
/// # Arguments
///
/// * `dir_path` - The path of the directory to create
///
/// # Returns
///
/// A `FileResult<()>` indicating success or failure.
pub async fn create_directory_all(dir_path: &str) -> FileResult<()> {
  return tokio::fs::create_dir_all(dir_path)
    .await
    .map_err(|e| FileError::DirectoryCreate(e.to_string()));
}

/// Validates that a file exists, returning an error if it doesn't.
///
/// # Arguments
///
/// * `file_path` - The path to the file to validate
///
/// # Returns
///
/// A `FileResult<()>` indicating success if the file exists, or an error if it doesn't.
pub async fn validate_file_exists(file_path: &str) -> FileResult<()> {
  if !file_exists(file_path).await {
    return Err(FileError::FileNotFound(file_path.to_string()));
  }
  return Ok(());
}

/// Checks if a file exists at the given path.
///
/// # Arguments
///
/// * `file_path` - The path to check for file existence
///
/// # Returns
///
/// A `bool` indicating whether the file exists.
pub async fn file_exists(file_path: &str) -> bool {
  return tokio::fs::metadata(file_path).await.is_ok();
}

/// Reads the entire contents of a file into a string.
///
/// # Arguments
///
/// * `file_path` - The path to the file to read
///
/// # Returns
///
/// A `FileResult<String>` containing the file contents or an error.
pub async fn read_to_string(file_path: &str) -> FileResult<String> {
  return tokio::fs::read_to_string(file_path)
    .await
    .map_err(|e| FileError::FileRead(e.to_string()));
}
