use thiserror::Error;

/// Configuration-related errors.
///
/// Represents errors that can occur during configuration loading and parsing.
#[derive(Error, Debug)]
pub enum ConfigError {
  #[error(
    "Cannot read configuration file: '{0}'. Please check file permissions and ensure the file exists."
  )]
  FileRead(String),

  #[error(
    "Configuration file is invalid: '{0}'. Please check the syntax and ensure all required fields are present."
  )]
  Parse(String),
}

/// Result type for configuration operations.
pub type ConfigResult<T> = Result<T, ConfigError>;
