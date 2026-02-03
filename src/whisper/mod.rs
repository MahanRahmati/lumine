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
mod responses;

#[cfg(test)]
mod whisper_tests;

use reqwest::multipart;

use crate::files::operations;
use crate::network::{HttpClient, errors::NetworkError};
use crate::output::format::OutputFormat;
use crate::vlog;
use crate::whisper::errors::{WhisperError, WhisperResult};
use crate::whisper::responses::{
  WhisperJsonResponse, WhisperResponse, WhisperTextResponse,
  WhisperVerboseJsonResponse, get_whisper_format,
};

/// Whisper transcription interface.
///
/// Handles transcription of audio files using a remote Whisper API service.
#[derive(Debug, Clone)]
pub struct Whisper {
  url: String,
  file_path: String,
  format: OutputFormat,
}

impl Whisper {
  /// Creates a new Whisper transcription instance.
  ///
  /// # Arguments
  ///
  /// * `url` - The Whisper service URL for transcription
  /// * `file_path` - Path to the audio file to transcribe
  /// * `format` - The desired output format
  ///
  /// # Returns
  ///
  /// A new `Whisper` instance.
  pub fn new(url: String, file_path: String, format: OutputFormat) -> Self {
    return Whisper {
      url,
      file_path,
      format,
    };
  }

  /// Transcribes the audio file using Whisper API.
  ///
  /// # Arguments
  ///
  /// * `format` - The desired output format
  ///
  /// # Returns
  ///
  /// A `WhisperResult<WhisperResponse>` containing the transcription data or an error.
  pub async fn transcribe(&self) -> WhisperResult<WhisperResponse> {
    vlog!("Sending audio file to Whisper transcription service...");

    let output = self.transcribe_remote().await?;

    vlog!("Transcription completed successfully.");
    return Ok(output);
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
      .text("response_format", get_whisper_format(self.format))
      .part("file", file_part);

    let client = HttpClient::new(self.url.clone());

    return self.deserialize_response(&client, form, self.format).await;
  }

  async fn deserialize_response(
    &self,
    client: &HttpClient,
    form: multipart::Form,
    format: OutputFormat,
  ) -> WhisperResult<WhisperResponse> {
    match format {
      OutputFormat::Text => {
        let response = client
          .post_with_form::<WhisperJsonResponse>(form, "inference")
          .await
          .map_err(|e| self.map_network_error(e))?;
        return Ok(WhisperResponse::Text(WhisperTextResponse {
          text: response.text,
        }));
      }
      OutputFormat::Json => {
        let response = client
          .post_with_form::<WhisperJsonResponse>(form, "inference")
          .await
          .map_err(|e| self.map_network_error(e))?;
        return Ok(WhisperResponse::Json(response));
      }
      OutputFormat::FullJson => {
        let response = client
          .post_with_form::<WhisperVerboseJsonResponse>(form, "inference")
          .await
          .map_err(|e| self.map_network_error(e))?;
        return Ok(WhisperResponse::VerboseJson(response));
      }
    }
  }

  fn map_network_error(&self, network_error: NetworkError) -> WhisperError {
    return match network_error {
      NetworkError::RequestFailed => WhisperError::RequestFailed,
      NetworkError::InvalidURL(url) => WhisperError::InvalidURL(url),
      NetworkError::ResponseError => WhisperError::ResponseError,
      NetworkError::DecodeError => WhisperError::DecodeError(String::new()),
    };
  }
}
