use thiserror::Error;

/// Whisper transcription service errors.
///
/// Represents errors that can occur during audio transcription operations.
#[derive(Error, Debug)]
pub enum WhisperError {
  #[error(
    "Audio file not found: '{0}'. Please ensure the file exists and is readable."
  )]
  FileNotFound(String),

  #[error(
    "Invalid Whisper service URL: '{0}'. Please check your configuration file."
  )]
  InvalidURL(String),

  #[error(
    "Failed to connect to Whisper service. Please verify the service is running and accessible."
  )]
  RequestFailed,

  #[error(
    "Whisper service returned an error. Please check the service logs and try again."
  )]
  ResponseError,

  #[error(
    "Failed to decode Whisper response. The service may be experiencing issues or the audio format may be unsupported."
  )]
  DecodeError,
}

/// Result type for Whisper operations.
pub type WhisperResult<T> = Result<T, WhisperError>;
