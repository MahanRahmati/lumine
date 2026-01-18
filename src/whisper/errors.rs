use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhisperError {
  #[error(
    "Audio file not found. Please ensure the file exists and is readable."
  )]
  FileNotFound,

  #[error("Invalid Whisper service URL. Please check your configuration file.")]
  InvalidURL,

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

pub type WhisperResult<T> = Result<T, WhisperError>;
