//! Response types for Whisper transcription API.
//!
//! This module defines the various response formats that Whisper API can return,
//! including plain text, simple JSON, and verbose JSON with full metadata.

use crate::output::format::OutputFormat;
use crate::whisper::errors::{WhisperError, WhisperResult};

/// Response from the Whisper transcription service.
///
/// This enum wraps all possible response formats from the Whisper API:
/// - `Text`: Plain text response
/// - `Json`: Simple JSON with just the text field
/// - `VerboseJson`: Full JSON with segments, words, and all metadata
#[derive(Debug, Clone)]
pub enum WhisperResponse {
  Text(WhisperTextResponse),
  Json(WhisperJsonResponse),
  VerboseJson(WhisperVerboseJsonResponse),
}

impl WhisperResponse {
  pub fn format(&self, format: OutputFormat) -> WhisperResult<String> {
    return match (&self, format) {
      (WhisperResponse::Text(text_response), OutputFormat::Text) => {
        Ok(text_response.text.clone())
      }
      (WhisperResponse::Json(json_response), OutputFormat::Json) => {
        serde_json::to_string_pretty(json_response)
          .map_err(|e| WhisperError::DecodeError(e.to_string()))
      }
      (
        WhisperResponse::VerboseJson(verbose_response),
        OutputFormat::FullJson,
      ) => serde_json::to_string_pretty(verbose_response)
        .map_err(|e| WhisperError::DecodeError(e.to_string())),
      _ => Err(WhisperError::DecodeError(
        "Response format mismatch".to_string(),
      )),
    };
  }
}

/// Response from Whisper API when using `text` response format.
///
/// This is a plain text response (not JSON).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct WhisperTextResponse {
  pub text: String,
}

/// Response from Whisper API when using `json` response format.
///
/// Contains just the transcribed text.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperJsonResponse {
  pub text: String,
}

/// Word-level information within a segment.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperWord {
  pub word: String,
  pub start: f64,
  pub end: f64,
  #[serde(rename = "t_dtw")]
  pub t_dtw: i64,
  pub probability: f64,
}

/// Segment information containing text, timing, and word-level data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperSegment {
  pub id: i64,
  pub text: String,
  pub start: f64,
  pub end: f64,
  pub tokens: Vec<i64>,
  pub words: Vec<WhisperWord>,
  pub temperature: f64,
  #[serde(rename = "avg_logprob")]
  pub avg_logprob: f64,
  #[serde(rename = "no_speech_prob")]
  pub no_speech_prob: f64,
}

/// Response from Whisper API when using `verbose_json` response format.
///
/// Contains full metadata including segments, word-level timing, and
/// language detection probabilities.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperVerboseJsonResponse {
  pub task: String,
  pub language: String,
  pub duration: f64,
  pub text: String,
  pub segments: Vec<WhisperSegment>,
  #[serde(rename = "detected_language")]
  pub detected_language: String,
  #[serde(rename = "detected_language_probability")]
  pub detected_language_probability: f64,
  #[serde(rename = "language_probabilities")]
  pub language_probabilities: std::collections::HashMap<String, f64>,
}

/// Maps the internal OutputFormat to the Whisper API response format string.
///
/// Whisper API accepts different format parameters that control the level of
/// detail in the transcription response. This function translates our internal
/// format representation to the API's expected format parameter.
///
/// # Arguments
///
/// * `format` - The desired output format variant
///
/// # Returns
///
/// The Whisper API format string to use in the request ("json" or "verbose_json")
pub fn get_whisper_format(format: OutputFormat) -> String {
  let whisper_format = match format {
    OutputFormat::Text => String::from("json"),
    OutputFormat::Json => String::from("json"),
    OutputFormat::FullJson => String::from("verbose_json"),
  };
  return whisper_format;
}
