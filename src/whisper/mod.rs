mod errors;

#[cfg(test)]
mod whisper_tests;

use reqwest::multipart;

use crate::files::operations;
use crate::network::{HttpClient, errors::NetworkError};
use crate::whisper::errors::{WhisperError, WhisperResult};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct WhisperResponse {
  pub text: String,
}

#[derive(Debug, Clone)]
pub struct Whisper {
  url: String,
  file_path: String,
  verbose: bool,
}

impl Whisper {
  pub fn new(url: String, file_path: String, verbose: bool) -> Self {
    return Whisper {
      url,
      file_path,
      verbose,
    };
  }

  pub async fn send_audio(&self) -> WhisperResult<String> {
    if self.verbose {
      println!("Sending audio file to Whisper transcription service...");
    }
    let response = self.send_audio_file_to_whisper().await?;
    if self.verbose {
      println!("Transcription completed successfully.");
    }
    return Ok(response.text);
  }

  async fn send_audio_file_to_whisper(&self) -> WhisperResult<WhisperResponse> {
    if self.verbose {
      println!("Validating file path...");
    }

    if operations::validate_file_exists(&self.file_path)
      .await
      .is_err()
    {
      return Err(WhisperError::FileNotFound);
    }

    if self.verbose {
      println!("Preparing multipart form for audio file upload...");
    }

    let file_bytes = match tokio::fs::read(&self.file_path).await {
      Ok(bytes) => bytes,
      Err(_) => return Err(WhisperError::RequestFailed),
    };

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

    let client = HttpClient::new(self.url.clone(), self.verbose);

    match client
      .post_with_form::<WhisperResponse>(form, "inference")
      .await
    {
      Ok(response) => return Ok(response),
      Err(network_error) => {
        let whisper_error = match network_error {
          NetworkError::RequestFailed => WhisperError::RequestFailed,
          NetworkError::InvalidURL => WhisperError::InvalidURL,
          NetworkError::ResponseError => WhisperError::ResponseError,
          NetworkError::DecodeError => WhisperError::DecodeError,
        };
        return Err(whisper_error);
      }
    };
  }
}
