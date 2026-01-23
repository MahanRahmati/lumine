use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
  #[error("Audio file not found. Please check the file path.")]
  FileNotFound,

  #[error(
    "Failed to convert audio to Whisper format. Please check FFmpeg installation and file permissions."
  )]
  ConversionFailed,
}

pub type AudioResult<T> = Result<T, AudioError>;
