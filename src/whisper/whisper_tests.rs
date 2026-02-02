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
    false,
    config.get_whisper_url(),
    config.get_whisper_model_path(),
    config.get_vad_model_path(),
    sample_file_path.to_string(),
    false,
  );

  let result = whisper.transcribe().await;
  match result {
    Ok(transcript) => {
      assert!(!transcript.is_empty());
    }
    Err(error) => match error {
      WhisperError::InvalidURL(_)
      | WhisperError::RequestFailed
      | WhisperError::ResponseError => (),
      _ => panic!("Expected network-related error, got: {:?}", error),
    },
  }
}

#[tokio::test]
async fn test_send_audio_file_not_found() {
  let config = Config::default();
  let whisper = Whisper::new(
    config.get_use_local(),
    config.get_whisper_url(),
    config.get_whisper_model_path(),
    config.get_vad_model_path(),
    "nonexistent_file.wav".to_string(),
    false,
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

  let config = Config::default();
  let whisper = Whisper::new(
    false,
    "invalid-url".to_string(),
    config.get_whisper_model_path(),
    config.get_vad_model_path(),
    sample_file_path.to_string(),
    false,
  );

  let result = whisper.transcribe().await;
  assert!(result.is_err());
  match result.unwrap_err() {
    WhisperError::InvalidURL(_) => (),
    _ => panic!("Expected InvalidURL error"),
  }
}
