use thiserror::Error;

/// Audio-related errors.
///
/// Represents errors that can occur during audio recording and conversion operations.
#[derive(Error, Debug)]
pub enum AudioError {
  #[error("Audio file not found: '{0}'. Please check the file path.")]
  FileNotFound(String),

  #[error(
    "Failed to convert audio to Whisper format. Please check FFmpeg installation and file permissions."
  )]
  ConversionFailed,

  #[error(
    "FFmpeg not found. Please install FFmpeg and ensure it's in your PATH."
  )]
  FFMPEGNotFound,

  #[error(
    "Failed to run FFmpeg. Please check if FFmpeg is properly installed and has permission to access audio devices."
  )]
  CouldNotExecuteFFMPEG,

  #[error(
    "Unable to read FFmpeg output. This might be due to permission issues or corrupted FFmpeg installation."
  )]
  CouldNotReadFFMPEGOutput,

  #[error(
    "Cannot create recordings directory. Please check file permissions and available disk space."
  )]
  CouldNotCreateDirectory,
}

/// Result type for audio operations.
pub type AudioResult<T> = Result<T, AudioError>;
