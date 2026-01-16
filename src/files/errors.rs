#[derive(Debug, Clone)]
pub enum FileError {
  DirectoryCreate(String),
  FileRemove(String),
  FileRead(String),
  FileNotFound(String),
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      FileError::DirectoryCreate(msg) => {
        write!(
          f,
          "Cannot create directory '{}'. Please check permissions.",
          msg
        )
      }
      FileError::FileRemove(msg) => {
        write!(
          f,
          "Cannot remove file '{}'. Please check if the file exists and you have permission to delete it.",
          msg
        )
      }
      FileError::FileRead(msg) => {
        write!(
          f,
          "Cannot read file '{}'. Please check if the file exists and you have permission to access it.",
          msg
        )
      }
      FileError::FileNotFound(msg) => {
        write!(
          f,
          "File not found: {}. Please verify the file path and try again.",
          msg
        )
      }
    }
  }
}

pub type FileResult<T> = Result<T, FileError>;
