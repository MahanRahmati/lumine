#[derive(Debug, Clone)]
pub enum ConfigError {
  FileRead(String),
  Parse(String),
}

impl std::error::Error for ConfigError {}

impl std::fmt::Display for ConfigError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      ConfigError::FileRead(msg) => {
        write!(
          f,
          "Cannot read configuration file: {}. Please check file permissions and ensure the file exists.",
          msg
        )
      }
      ConfigError::Parse(msg) => {
        write!(
          f,
          "Configuration file is invalid: {}. Please check the syntax and ensure all required fields are present.",
          msg
        )
      }
    }
  }
}
