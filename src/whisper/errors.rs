#[derive(Debug, Clone)]
pub enum WhisperError {
  FileNotFound,
  InvalidURL,
  RequestFailed,
  ResponseError,
  DecodeError,
}

impl std::fmt::Display for WhisperError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      WhisperError::FileNotFound => {
        write!(
          f,
          "Audio file not found. Please ensure the file exists and is readable."
        )
      }
      WhisperError::InvalidURL => {
        write!(
          f,
          "Invalid Whisper service URL. Please check your configuration file."
        )
      }
      WhisperError::RequestFailed => {
        write!(
          f,
          "Failed to connect to Whisper service. Please verify the service is running and accessible."
        )
      }
      WhisperError::ResponseError => {
        write!(
          f,
          "Whisper service returned an error. Please check the service logs and try again."
        )
      }
      WhisperError::DecodeError => {
        write!(
          f,
          "Failed to decode Whisper response. The service may be experiencing issues or the audio format may be unsupported."
        )
      }
    }
  }
}

impl std::error::Error for WhisperError {}

pub type WhisperResult<T> = Result<T, WhisperError>;
