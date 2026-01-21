use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
  #[error("File Error: {0}. Please verify the file path and try again.")]
  File(String),

  #[error(
    "Recording Error: {0}. Please verify the recording settings and try again."
  )]
  Recording(String),

  #[error(
    "Audio Conversion Error: {0}. Please verify the audio settings and try again."
  )]
  AudioConversion(String),

  #[error(
    "Transcription Error: {0}. Please check the audio file and model compatibility."
  )]
  Transcription(String),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
