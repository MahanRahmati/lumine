mod errors;

use crate::app::errors::{RuntimeError, RuntimeResult};
use crate::audio::Audio;
use crate::config::Config;
use crate::files::operations::validate_file_exists;
use crate::files::temporary::TemporaryFile;
use crate::whisper::Whisper;

/// Main application orchestrator for Lumine.
///
/// Coordinates audio recording, conversion, and transcription operations
/// using the provided configuration settings.
pub struct App {
  config: Config,
}

impl App {
  /// Creates a new App instance with the given configuration.
  ///
  /// # Arguments
  ///
  /// * `config` - Configuration containing all application settings
  ///
  /// # Returns
  ///
  /// A new `App` instance.
  pub fn new(config: Config) -> Self {
    return App { config };
  }

  fn create_audio(&self) -> Audio {
    return Audio::new(
      self.config.get_recordings_directory(),
      self.config.get_silence_limit(),
      self.config.get_silence_detect_noise(),
      self.config.get_preferred_audio_input_device(),
      self.config.get_verbose(),
    );
  }

  fn create_whisper_instance(&self, file_path: String) -> Whisper {
    return Whisper::new(
      self.config.get_use_local(),
      self.config.get_whisper_url(),
      self.config.get_whisper_model_path(),
      self.config.get_vad_model_path(),
      file_path,
      self.config.get_verbose(),
    );
  }

  async fn cleanup_file(&self, temp_file: &mut TemporaryFile) {
    if self.config.get_remove_after_transcript() {
      let result = temp_file.cleanup().await;
      if result.is_ok() && self.config.get_verbose() {
        println!("File removed: {}", temp_file.path());
      }
    } else {
      temp_file.keep();
    }
  }

  /// Transcribes an existing audio file.
  ///
  /// Converts the input audio to Whisper-compatible format and performs
  /// transcription using the configured Whisper service or local model.
  ///
  /// # Arguments
  ///
  /// * `file_path` - Path to the audio file to transcribe
  ///
  /// # Returns
  ///
  /// A `RuntimeResult<String>` containing the transcription text or an error.
  pub async fn transcribe_file(
    &self,
    file_path: &str,
  ) -> RuntimeResult<String> {
    validate_file_exists(file_path)
      .await
      .map_err(|e| RuntimeError::File(e.to_string()))?;

    let audio = self.create_audio();
    let converted_file_path = audio
      .convert_audio(file_path)
      .await
      .map_err(|e| RuntimeError::AudioConversion(e.to_string()))?;

    let mut temp_converted_file = TemporaryFile::new(converted_file_path);

    let whisper =
      self.create_whisper_instance(temp_converted_file.path().to_string());
    let transcript = whisper
      .transcribe()
      .await
      .map_err(|e| RuntimeError::Transcription(e.to_string()))?;

    self.cleanup_file(&mut temp_converted_file).await;

    return Ok(transcript);
  }

  /// Records audio without transcription.
  ///
  /// Records audio using configured settings and converts it to Whisper-compatible
  /// format, keeping both original and converted files based on configuration.
  ///
  /// # Returns
  ///
  /// A `RuntimeResult<String>` containing the path to the converted audio file
  /// and a success message.
  pub async fn record_only(&self) -> RuntimeResult<String> {
    let audio = self.create_audio();
    let file_path = audio
      .record_audio()
      .await
      .map_err(|e| RuntimeError::Recording(e.to_string()))?;

    let mut temp_original_file = TemporaryFile::new(file_path.clone());

    let converted_file_path = audio
      .convert_audio(&file_path)
      .await
      .map_err(|e| RuntimeError::AudioConversion(e.to_string()))?;

    let mut temp_converted_file = TemporaryFile::new(converted_file_path);

    if self.config.get_verbose() {
      println!("File saved in: {}", self.config.get_recordings_directory());
      println!("Format: 16kHz mono WAV (Whisper-ready)");
    }

    let result = Ok(format!(
      "Audio recorded and converted successfully: {}",
      temp_converted_file.path()
    ));

    self.cleanup_file(&mut temp_original_file).await;
    temp_converted_file.keep();

    return result;
  }

  /// Records audio and transcribes it in sequence.
  ///
  /// Records audio using configured settings, converts it to Whisper-compatible
  /// format, and performs transcription using the configured Whisper service.
  ///
  /// # Returns
  ///
  /// A `RuntimeResult<String>` containing the transcription text or an error.
  pub async fn record_and_transcribe(&self) -> RuntimeResult<String> {
    let audio = self.create_audio();
    let file_path = audio
      .record_audio()
      .await
      .map_err(|e| RuntimeError::Recording(e.to_string()))?;

    let mut temp_original_file = TemporaryFile::new(file_path.clone());

    let converted_file_path = audio
      .convert_audio(&file_path)
      .await
      .map_err(|e| RuntimeError::AudioConversion(e.to_string()))?;

    let mut temp_converted_file = TemporaryFile::new(converted_file_path);

    let whisper =
      self.create_whisper_instance(temp_converted_file.path().to_string());
    let transcript = whisper
      .transcribe()
      .await
      .map_err(|e| RuntimeError::Transcription(e.to_string()))?;

    self.cleanup_file(&mut temp_original_file).await;
    self.cleanup_file(&mut temp_converted_file).await;

    return Ok(transcript);
  }
}
