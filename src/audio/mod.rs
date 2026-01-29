mod converter;
mod devices;
mod errors;
mod platform;
mod recorder;

use crate::audio::converter::AudioConverter;
use crate::audio::errors::AudioResult;
use crate::audio::platform::get_platform;
use crate::audio::recorder::AudioRecorder;

#[derive(Debug, Clone)]
pub struct Audio {
  recordings_directory: String,
  silence_limit: i32,
  silence_detect_noise: i32,
  preferred_audio_input_device: String,
  verbose: bool,
}

impl Audio {
  pub fn new(
    recordings_directory: String,
    silence_limit: i32,
    silence_detect_noise: i32,
    preferred_audio_input_device: String,
    verbose: bool,
  ) -> Self {
    return Audio {
      recordings_directory,
      silence_limit,
      silence_detect_noise,
      preferred_audio_input_device,
      verbose,
    };
  }

  pub async fn record_audio(&self) -> AudioResult<String> {
    let recorder = AudioRecorder::new(
      self.recordings_directory.clone(),
      self.silence_limit,
      self.silence_detect_noise,
      self.preferred_audio_input_device.clone(),
      self.verbose,
      get_platform(),
    );
    return recorder.record_audio().await;
  }

  pub async fn convert_audio(&self, input_file: &str) -> AudioResult<String> {
    return AudioConverter::convert_audio_for_whisper(input_file, self.verbose)
      .await;
  }
}
