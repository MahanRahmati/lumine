mod errors;

use reqwest::blocking::multipart;

use super::files::operations;

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

  pub fn send_audio(&self) -> WhisperResult<String> {
    if self.verbose {
      println!("Sending audio file to Whisper transcription service...");
    }
    let response = self.send_audio_file_to_whisper()?;
    if self.verbose {
      println!("Transcription completed successfully.");
    }
    return Ok(response.text);
  }

  fn send_audio_file_to_whisper(&self) -> WhisperResult<WhisperResponse> {
    if self.verbose {
      println!("Validating file path...");
    }

    if operations::validate_file_exists(&self.file_path).is_err() {
      return Err(WhisperError::FileNotFound);
    }

    self.check_url()?;

    if self.verbose {
      println!("Preparing multipart form for audio file upload...");
    }

    let form = match multipart::Form::new()
      .text("response_format", "json")
      .file("file", &self.file_path)
    {
      Ok(form) => form,
      Err(_) => return Err(WhisperError::RequestFailed),
    };

    let client = reqwest::blocking::Client::new();
    let inference_url = format!("{}/inference", self.url);

    if self.verbose {
      println!(
        "Sending audio file to Whisper via POST request to: {}",
        inference_url
      );
    }

    let response = match client.post(&inference_url).multipart(form).send() {
      Ok(response) => response,
      Err(_) => return Err(WhisperError::RequestFailed),
    };

    if self.verbose {
      println!(
        "Received response from Whisper service. Status: {}",
        response.status()
      );
    }

    if response.status() != reqwest::StatusCode::OK {
      return Err(WhisperError::ResponseError);
    }

    let response_text = match response.text() {
      Ok(text) => text,
      Err(_) => return Err(WhisperError::DecodeError),
    };

    let whisper_response: WhisperResponse =
      match serde_json::from_str(&response_text) {
        Ok(response) => response,
        Err(_) => return Err(WhisperError::DecodeError),
      };

    return Ok(whisper_response);
  }

  fn check_url(&self) -> WhisperResult<()> {
    if self.verbose {
      println!("Checking if Whisper service URL is reachable...");
    }

    let _url = match reqwest::Url::parse(&self.url) {
      Ok(url) => url,
      Err(e) => {
        if self.verbose {
          println!("Invalid URL format: {}", e);
        }
        return Err(WhisperError::InvalidURL);
      }
    };

    let client = reqwest::blocking::Client::new();

    let response = match client.get(&self.url).send() {
      Ok(response) => response,
      Err(e) => {
        if self.verbose {
          println!("Failed to connect to URL: {}", e);
        }
        return Err(WhisperError::RequestFailed);
      }
    };

    let status = response.status();
    if status != reqwest::StatusCode::OK
      && status != reqwest::StatusCode::NOT_FOUND
    {
      if self.verbose {
        println!("URL returned unexpected status: {}", status);
      }
      return Err(WhisperError::InvalidURL);
    }

    if self.verbose {
      println!("Whisper service URL is reachable with status: {}", status);
    }

    return Ok(());
  }
}
