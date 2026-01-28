use thiserror::Error;

/// Whisper transcription service errors.
///
/// Represents errors that can occur during audio transcription operations.
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

  #[error(
    "Failed to load Whisper model. Please ensure the model file is valid and accessible."
  )]
  ModelNotFound,

  #[error(
    "Failed to create Whisper state. Please ensure the model is properly initialized."
  )]
  StateCreationFailed,

  #[error(
    "Failed to convert audio data. Please ensure the audio format is supported."
  )]
  AudioConversionFailed,

  #[error("Audio format not supported. Expected 16kHz mono PCM.")]
  UnsupportedAudioFormat,

  #[error(
    "Transcription failed. Please check the audio file and model compatibility."
  )]
  TranscriptionFailed,

  #[error(
    "VAD model not found. Please ensure the VAD model file is valid and accessible."
  )]
  VadModelNotFound,
}

/// Result type for Whisper operations.
pub type WhisperResult<T> = Result<T, WhisperError>;
