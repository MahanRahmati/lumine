mod errors;

#[cfg(test)]
mod whisper_tests;

use hound::WavReader;
use reqwest::multipart;
use whisper_rs::{
  FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
  install_logging_hooks,
};

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
  model_path: String,
  file_path: String,
  verbose: bool,
}

impl Whisper {
  pub fn new(
    url: String,
    model_path: String,
    file_path: String,
    verbose: bool,
  ) -> Self {
    return Whisper {
      url,
      model_path,
      file_path,
      verbose,
    };
  }

  pub async fn transcribe(&self) -> WhisperResult<String> {
    if self.verbose {
      println!("Sending audio file to Whisper transcription service...");
    }
    if self.model_path.is_empty() {
      let response = self.transcribe_remote().await?;
      if self.verbose {
        println!("Transcription completed successfully.");
      }
      return Ok(response.text);
    } else {
      let response = self.transcribe_local().await?;
      if self.verbose {
        println!("Transcription completed successfully.");
      }
      return Ok(response.text);
    }
  }

  async fn transcribe_remote(&self) -> WhisperResult<WhisperResponse> {
    if self.verbose {
      println!("Validating file path...");
    }

    operations::validate_file_exists(&self.file_path)
      .await
      .map_err(|_| WhisperError::FileNotFound)?;

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
          NetworkError::InvalidURL => WhisperError::InvalidURL,
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
      .map_err(|_| WhisperError::FileNotFound)?;

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
      .map_err(|_| WhisperError::FileNotFound)?;

    let spec = reader.spec();
    if spec.sample_rate != 16000 {
      return Err(WhisperError::UnsupportedAudioFormat);
    }

    let samples: Vec<i16> = reader
      .into_samples::<i16>()
      .map(|x| match x {
        Ok(sample) => sample,
        Err(_) => 0,
      })
      .collect();

    let mut audio = vec![0.0f32; samples.len()];
    if whisper_rs::convert_integer_to_float_audio(&samples, &mut audio).is_err()
    {
      return Err(WhisperError::AudioConversionFailed);
    }

    let audio = if spec.channels == 1 {
      audio
    } else if spec.channels == 2 {
      match whisper_rs::convert_stereo_to_mono_audio(&audio) {
        Ok(output) => output,
        Err(_) => return Err(WhisperError::AudioConversionFailed),
      }
    } else {
      return Err(WhisperError::UnsupportedAudioFormat);
    };

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

    if self.verbose {
      println!("Transcription completed successfully.");
    }

    return Ok(WhisperResponse {
      text: transcript.trim().to_string(),
    });
  }
}
