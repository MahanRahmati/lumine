use std::fs;

use crate::config::*;
use crate::whisper::*;

#[tokio::test]
async fn test_send_audio() {
  let sample_file_path = "sample/jfk.wav";

  assert!(
    fs::metadata(sample_file_path).is_ok(),
    "Sample file should exist"
  );

  let config = Config::default();
  let whisper = Whisper::new(
    config.get_whisper_url(),
    sample_file_path.to_string(),
    OutputFormat::Text,
  );

  let result = whisper.transcribe().await;
  match result {
    Ok(transcript) => match transcript {
      WhisperResponse::Text(text_response) => {
        assert!(!text_response.text.is_empty());
      }
      _ => panic!("Expected Text response variant"),
    },
    Err(error) => match error {
      WhisperError::InvalidURL(_)
      | WhisperError::RequestFailed
      | WhisperError::ResponseError
      | WhisperError::DecodeError(_) => (),
      _ => panic!("Expected network-related error, got: {:?}", error),
    },
  }
}

#[tokio::test]
async fn test_send_audio_file_not_found() {
  let config = Config::default();
  let whisper = Whisper::new(
    config.get_whisper_url(),
    "nonexistent_file.wav".to_string(),
    OutputFormat::Text,
  );

  let result = whisper.transcribe().await;
  assert!(result.is_err());
  match result.unwrap_err() {
    WhisperError::FileNotFound(_) => (),
    _ => panic!("Expected FileNotFound error"),
  }
}

#[tokio::test]
async fn test_send_audio_with_sample_file_invalid_url() {
  let sample_file_path = "sample/jfk.wav";

  assert!(
    fs::metadata(sample_file_path).is_ok(),
    "Sample file should exist"
  );

  let whisper = Whisper::new(
    "invalid-url".to_string(),
    sample_file_path.to_string(),
    OutputFormat::Text,
  );

  let result = whisper.transcribe().await;
  assert!(result.is_err());
  match result.unwrap_err() {
    WhisperError::InvalidURL(_) => (),
    _ => panic!("Expected InvalidURL error"),
  }
}
