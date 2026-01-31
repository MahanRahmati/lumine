use thiserror::Error;

/// Process execution errors.
///
/// Represents errors that can occur during command execution and
/// process management operations.
#[derive(Error, Debug)]
pub enum ProcessError {
  #[error(
    "Command '{0}' failed to execute. Please check the command exists and has proper permissions."
  )]
  ExecutionFailed(String),
}

/// Result type for process operations.
pub type ProcessResult<T> = Result<T, ProcessError>;
