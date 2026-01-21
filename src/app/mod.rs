mod errors;

use crate::app::errors::{RuntimeError, RuntimeResult};
use crate::config::Config;
use crate::ffmpeg::FFMPEG;
use crate::files::operations::{remove_file, validate_file_exists};
use crate::whisper::Whisper;

pub struct App {
  config: Config,
}

impl App {
  pub fn new(config: Config) -> Self {
    return App { config };
  }

  fn create_ffmpeg_instance(&self) -> FFMPEG {
    return FFMPEG::new(
      self.config.get_recordings_directory(),
      self.config.get_silence_limit(),
      self.config.get_silence_detect_noise(),
      self.config.get_preferred_audio_input_device(),
      self.config.get_verbose(),
    );
  }

  fn create_whisper_instance(&self, file_path: String) -> Whisper {
    return Whisper::new(
      self.config.get_whisper_url(),
      self.config.get_whisper_model_path(),
      self.config.get_vad_model_path(),
      file_path,
      self.config.get_verbose(),
    );
  }

  async fn cleanup_file(&self, file_path: &str) {
    if self.config.get_remove_after_transcript() {
      let result = remove_file(file_path).await;
      if result.is_ok() && self.config.get_verbose() {
        println!("File removed: {}", file_path);
      }
    }
  }

  pub async fn transcribe_file(
    &self,
    file_path: &str,
  ) -> RuntimeResult<String> {
    validate_file_exists(file_path)
      .await
      .map_err(|e| RuntimeError::File(e.to_string()))?;

    let ffmpeg = self.create_ffmpeg_instance();
    let converted_file_path = ffmpeg
      .convert_audio_for_whisper(file_path)
      .await
      .map_err(|e| RuntimeError::AudioConversion(e.to_string()))?;

    let whisper = self.create_whisper_instance(converted_file_path.clone());
    let transcript = whisper
      .transcribe()
      .await
      .map_err(|e| RuntimeError::Transcription(e.to_string()))?;

    self.cleanup_file(&converted_file_path).await;

    return Ok(transcript);
  }

  pub async fn record_only(&self) -> RuntimeResult<String> {
    let ffmpeg = self.create_ffmpeg_instance();
    let file_path = ffmpeg
      .record_audio()
      .await
      .map_err(|e| RuntimeError::Recording(e.to_string()))?;

    let converted_file_path = ffmpeg
      .convert_audio_for_whisper(&file_path)
      .await
      .map_err(|e| RuntimeError::AudioConversion(e.to_string()))?;

    self.cleanup_file(&file_path).await;

    if self.config.get_verbose() {
      println!("File saved in: {}", self.config.get_recordings_directory());
      println!("Format: 16kHz mono WAV (Whisper-ready)");
    }

    return Ok(format!(
      "Audio recorded and converted successfully: {}",
      converted_file_path
    ));
  }

  pub async fn record_and_transcribe(&self) -> RuntimeResult<String> {
    let ffmpeg = self.create_ffmpeg_instance();
    let file_path = ffmpeg
      .record_audio()
      .await
      .map_err(|e| RuntimeError::Recording(e.to_string()))?;

    let converted_file_path = ffmpeg
      .convert_audio_for_whisper(&file_path)
      .await
      .map_err(|e| RuntimeError::AudioConversion(e.to_string()))?;

    let whisper = self.create_whisper_instance(converted_file_path.clone());
    let transcript = whisper
      .transcribe()
      .await
      .map_err(|e| RuntimeError::Transcription(e.to_string()))?;

    self.cleanup_file(&file_path).await;
    self.cleanup_file(&converted_file_path).await;

    return Ok(transcript);
  }
}
