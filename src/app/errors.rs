use thiserror::Error;

/// Application runtime errors.
///
/// Represents high-level errors that can occur during application workflows.
#[derive(Error, Debug)]
pub enum RuntimeError {
  #[error("File Error: {0}")]
  File(String),

  #[error("Recording Error: {0}")]
  Recording(String),

  #[error("Audio Conversion Error: {0}")]
  AudioConversion(String),

  #[error("Transcription Error: {0}")]
  Transcription(String),
}

/// Result type for application runtime operations.
pub type RuntimeResult<T> = Result<T, RuntimeError>;
