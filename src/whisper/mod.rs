//! Whisper transcription module for audio-to-text conversion.
//!
//! This module provides audio transcription using OpenAI's Whisper model
//! via HTTP API endpoint.
//!
//! ## Main Components
//!
//! - [`Whisper`]: Main transcription interface
//! - [`WhisperResponse`]: Response structure containing transcribed text
//! - [`WhisperError`]: Error types for transcription failures
//! - [`WhisperResult<T>`]: Result type alias for transcription operations

mod errors;

#[cfg(test)]
mod whisper_tests;

use reqwest::multipart;

use crate::files::operations;
use crate::network::{HttpClient, errors::NetworkError};
use crate::vlog;
use crate::whisper::errors::{WhisperError, WhisperResult};

/// Response from the Whisper transcription service.
///
/// Contains the transcribed text from an audio file.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WhisperResponse {
  pub text: String,
}

/// Whisper transcription interface.
///
/// Handles transcription of audio files using a remote Whisper API service.
#[derive(Debug, Clone)]
pub struct Whisper {
  url: String,
  file_path: String,
}

impl Whisper {
  /// Creates a new Whisper transcription instance.
  ///
  /// # Arguments
  ///
  /// * `url` - The Whisper service URL for transcription
  /// * `file_path` - Path to the audio file to transcribe
  ///
  /// # Returns
  ///
  /// A new `Whisper` instance.
  pub fn new(url: String, file_path: String) -> Self {
    return Whisper { url, file_path };
  }

  /// Transcribes the audio file using Whisper API.
  ///
  /// # Returns
  ///
  /// A `WhisperResult<String>` containing the transcribed text or an error.
  pub async fn transcribe(&self) -> WhisperResult<String> {
    vlog!("Sending audio file to Whisper transcription service...");

    let response = self.transcribe_remote().await?;

    vlog!("Transcription completed successfully.");
    return Ok(response.text);
  }

  async fn transcribe_remote(&self) -> WhisperResult<WhisperResponse> {
    vlog!("Validating file path...");

    operations::validate_file_exists(&self.file_path)
      .await
      .map_err(|_| WhisperError::FileNotFound(self.file_path.clone()))?;

    vlog!("Preparing multipart form for audio file upload...");

    let file_bytes = tokio::fs::read(&self.file_path)
      .await
      .map_err(|_| WhisperError::RequestFailed)?;

    let file_part = multipart::Part::bytes(file_bytes).file_name(
      std::path::Path::new(&self.file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("audio.wav")
        .to_string(),
    );

    let form = multipart::Form::new()
      .text("response_format", "json")
      .part("file", file_part);

    let client = HttpClient::new(self.url.clone());

    match client
      .post_with_form::<WhisperResponse>(form, "inference")
      .await
    {
      Ok(response) => return Ok(response),
      Err(network_error) => {
        let whisper_error = match network_error {
          NetworkError::RequestFailed => WhisperError::RequestFailed,
          NetworkError::InvalidURL(url) => WhisperError::InvalidURL(url),
          NetworkError::ResponseError => WhisperError::ResponseError,
          NetworkError::DecodeError => WhisperError::DecodeError,
        };
        return Err(whisper_error);
      }
    };
  }
}
