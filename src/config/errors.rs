#[derive(Debug, Clone)]
pub enum ConfigError {
  FileRead(String),
  FileWrite(String),
  Parse(String),
  Serialize(String),
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
      ConfigError::FileWrite(msg) => {
        write!(
          f,
          "Cannot save configuration file: {}. Please check permissions and available disk space.",
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
      ConfigError::Serialize(msg) => {
        write!(
          f,
          "Failed to process configuration: {}. Please check your configuration values.",
          msg
        )
      }
    }
  }
}
