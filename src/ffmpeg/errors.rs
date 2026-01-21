use thiserror::Error;

#[derive(Error, Debug)]
pub enum FFMPEGError {
  #[error(
    "FFmpeg not found. Please install FFmpeg and ensure it's in your PATH."
  )]
  NotFound,

  #[error(
    "Failed to run FFmpeg. Please check if FFmpeg is properly installed and has permission to access audio devices."
  )]
  CouldNotExecute,

  #[error(
    "Unable to read FFmpeg output. This might be due to permission issues or corrupted FFmpeg installation."
  )]
  CouldNotReadOutput,

  #[error(
    "Cannot create recordings directory. Please check file permissions and available disk space."
  )]
  CouldNotCreateDirectory,

  #[error(
    "Failed to convert audio to Whisper format. Please check FFmpeg installation and file permissions."
  )]
  AudioConversionFailed,
}

pub type FFMPEGResult<T> = Result<T, FFMPEGError>;
