//! Whisper transcription module for audio-to-text conversion.
//!
//! This module provides audio transcription using OpenAI's Whisper model.
//! It supports both remote HTTP API transcription and local model inference
//! using the `whisper-rs` crate. Includes optional VAD (Voice Activity Detection)
//! preprocessing for improved accuracy.
//!
//! ## Main Components
//!
//! - [`Whisper`]: Main transcription interface
//! - [`WhisperResponse`]: Response structure containing transcribed text
//! - [`WhisperError`]: Error types for transcription failures
//! - [`WhisperResult<T>`]: Result type alias for transcription operations
//!
//! ## Transcription Modes
//!
//! - **Remote**: Send audio to HTTP API endpoint
//! - **Local**: Run inference with local Whisper model
//!
//! ## Audio Requirements
//!
//! - Sample rate: 16kHz (automatically validated)
//! - Channels: Mono or stereo (stereo converted to mono)
//! - Format: WAV PCM 16-bit

mod errors;

#[cfg(test)]
mod whisper_tests;

use hound::WavReader;
use reqwest::multipart;
use whisper_rs::{
  FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
  WhisperVadContext, WhisperVadContextParams, WhisperVadParams,
  install_logging_hooks,
};

use crate::files::operations;
use crate::network::{HttpClient, errors::NetworkError};
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
/// Handles both remote and local transcription of audio files using Whisper.
#[derive(Debug, Clone)]
pub struct Whisper {
  use_local: bool,
  url: String,
  model_path: String,
  vad_model_path: String,
  file_path: String,
  verbose: bool,
}

impl Whisper {
  /// Creates a new Whisper transcription instance.
  ///
  /// # Arguments
  ///
  /// * `use_local` - Whether to use the local Whisper model for transcription
  /// * `url` - The Whisper service URL for remote transcription
  /// * `model_path` - Path to local Whisper model (empty for remote mode)
  /// * `vad_model_path` - Path to VAD model for speech filtering (optional)
  /// * `file_path` - Path to the audio file to transcribe
  /// * `verbose` - Whether to enable verbose output
  ///
  /// # Returns
  ///
  /// A new `Whisper` instance.
  pub fn new(
    use_local: bool,
    url: String,
    model_path: String,
    vad_model_path: String,
    file_path: String,
    verbose: bool,
  ) -> Self {
    return Whisper {
      use_local,
      url,
      model_path,
      vad_model_path,
      file_path,
      verbose,
    };
  }

  /// Transcribes the audio file using Whisper.
  ///
  /// Automatically chooses between remote and local transcription based on
  /// whether a model path is configured.
  ///
  /// # Returns
  ///
  /// A `WhisperResult<String>` containing the transcribed text or an error.
  pub async fn transcribe(&self) -> WhisperResult<String> {
    if self.verbose {
      println!("Sending audio file to Whisper transcription service...");
    }

    let response = if self.use_local {
      self.transcribe_local().await?
    } else {
      self.transcribe_remote().await?
    };

    if self.verbose {
      println!("Transcription completed successfully.");
    }
    return Ok(response.text);
  }

  async fn transcribe_remote(&self) -> WhisperResult<WhisperResponse> {
    if self.verbose {
      println!("Validating file path...");
    }

    operations::validate_file_exists(&self.file_path)
      .await
      .map_err(|_| WhisperError::FileNotFound(self.file_path.clone()))?;

    if self.verbose {
      println!("Preparing multipart form for audio file upload...");
    }

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

    let client = HttpClient::new(self.url.clone(), self.verbose);

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

  async fn transcribe_local(&self) -> WhisperResult<WhisperResponse> {
    install_logging_hooks();

    if self.verbose {
      println!("Validating file path...");
    }

    operations::validate_file_exists(&self.file_path)
      .await
      .map_err(|_| WhisperError::FileNotFound(self.file_path.clone()))?;

    if self.verbose {
      println!("Loading Whisper model...");
    }

    let ctx = WhisperContext::new_with_params(
      &self.model_path,
      WhisperContextParameters::default(),
    )
    .map_err(|_| WhisperError::ModelNotFound)?;

    let mut state = ctx
      .create_state()
      .map_err(|_| WhisperError::StateCreationFailed)?;

    if self.verbose {
      println!("Reading audio file...");
    }

    let reader = WavReader::open(&self.file_path)
      .map_err(|_| WhisperError::FileNotFound(self.file_path.clone()))?;

    let spec = reader.spec();
    if spec.sample_rate != 16000 {
      return Err(WhisperError::UnsupportedAudioFormat);
    }

    let samples: Vec<i16> = reader
      .into_samples::<i16>()
      .map(|x| x.unwrap_or_default())
      .collect();

    let mut audio = vec![0.0f32; samples.len()];
    if whisper_rs::convert_integer_to_float_audio(&samples, &mut audio).is_err()
    {
      return Err(WhisperError::AudioConversionFailed);
    }

    let audio = if spec.channels == 1 {
      audio
    } else if spec.channels == 2 {
      whisper_rs::convert_stereo_to_mono_audio(&audio)
        .map_err(|_| WhisperError::AudioConversionFailed)?
    } else {
      return Err(WhisperError::UnsupportedAudioFormat);
    };

    let audio = self.apply_vad_preprocessing(audio, spec.sample_rate)?;

    if self.verbose {
      println!("Running transcription...");
    }

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
    params.set_n_threads(1);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    if state.full(params, &audio).is_err() {
      return Err(WhisperError::TranscriptionFailed);
    }

    let mut transcript = String::new();
    for segment in state.as_iter() {
      transcript.push_str(&segment.to_string());
      transcript.push(' ');
    }

    return Ok(WhisperResponse {
      text: transcript.trim().to_string(),
    });
  }

  fn apply_vad_preprocessing(
    &self,
    audio: Vec<f32>,
    sample_rate: u32,
  ) -> WhisperResult<Vec<f32>> {
    if self.vad_model_path.is_empty() {
      return Ok(audio);
    }

    if self.verbose {
      println!("Running VAD preprocessing to filter speech segments...");
    }

    let mut vad_ctx_params = WhisperVadContextParams::default();
    vad_ctx_params.set_n_threads(1);
    vad_ctx_params.set_use_gpu(false);

    let mut vad_ctx =
      WhisperVadContext::new(&self.vad_model_path, vad_ctx_params)
        .map_err(|_| WhisperError::VadModelNotFound)?;

    let vad_params = WhisperVadParams::new();
    let segments = vad_ctx
      .segments_from_samples(vad_params, &audio)
      .map_err(|_| WhisperError::TranscriptionFailed)?;

    if self.verbose {
      println!("VAD detected speech segments");
    }

    let mut speech_audio = Vec::new();
    for segment in segments {
      let start_ts = segment.start / 100.0;
      let end_ts = segment.end / 100.0;

      let start_sample_idx = (start_ts * sample_rate as f32) as usize;
      let end_sample_idx = (end_ts * sample_rate as f32) as usize;

      if start_sample_idx < audio.len() && end_sample_idx <= audio.len() {
        speech_audio
          .extend_from_slice(&audio[start_sample_idx..end_sample_idx]);
      }
    }

    if self.verbose {
      println!(
        "VAD extracted {} samples of speech audio",
        speech_audio.len()
      );
    }

    return Ok(speech_audio);
  }
}
