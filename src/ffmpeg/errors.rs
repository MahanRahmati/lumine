#[derive(Clone, Debug)]
pub enum FFMPEGError {
  NotFound,
  CouldNotExecute,
  CouldNotReadOutput,
  CouldNotCreateDirectory,
}

impl std::error::Error for FFMPEGError {}

impl std::fmt::Display for FFMPEGError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      FFMPEGError::NotFound => {
        write!(
          f,
          "FFmpeg not found. Please install FFmpeg and ensure it's in your PATH."
        )
      }
      FFMPEGError::CouldNotExecute => {
        write!(
          f,
          "Failed to run FFmpeg. Please check if FFmpeg is properly installed and has permission to access audio devices."
        )
      }
      FFMPEGError::CouldNotReadOutput => {
        write!(
          f,
          "Unable to read FFmpeg output. This might be due to permission issues or corrupted FFmpeg installation."
        )
      }
      FFMPEGError::CouldNotCreateDirectory => {
        write!(
          f,
          "Cannot create recordings directory. Please check file permissions and available disk space."
        )
      }
    }
  }
}

pub type FFMPEGResult<T> = Result<T, FFMPEGError>;
