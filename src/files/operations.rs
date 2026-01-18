use std::path::Path;

use crate::files::errors::{FileError, FileResult};

pub async fn remove_file(file_path: &str) -> FileResult<()> {
  let path = Path::new(file_path);
  match tokio::fs::remove_file(path).await {
    Ok(_) => return Ok(()),
    Err(e) => return Err(FileError::FileRemove(e.to_string())),
  }
}

pub async fn create_directory_all(dir_path: &str) -> FileResult<()> {
  match tokio::fs::create_dir_all(dir_path).await {
    Ok(_) => return Ok(()),
    Err(e) => return Err(FileError::DirectoryCreate(e.to_string())),
  }
}

pub async fn validate_file_exists(file_path: &str) -> FileResult<()> {
  if !file_exists(file_path).await {
    return Err(FileError::FileNotFound(file_path.to_string()));
  }
  return Ok(());
}

pub async fn file_exists(file_path: &str) -> bool {
  return tokio::fs::metadata(file_path).await.is_ok();
}

pub async fn read_to_string(file_path: &str) -> FileResult<String> {
  match tokio::fs::read_to_string(file_path).await {
    Ok(content) => return Ok(content),
    Err(e) => return Err(FileError::FileRead(e.to_string())),
  }
}
