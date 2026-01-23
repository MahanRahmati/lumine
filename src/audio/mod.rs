mod converter;
mod errors;

use crate::audio::converter::AudioConverter;
use crate::audio::errors::AudioResult;

#[derive(Debug, Clone)]
pub struct Audio {
  verbose: bool,
}

impl Audio {
  pub fn new(verbose: bool) -> Self {
    return Audio { verbose };
  }

  pub async fn convert_audio(&self, input_file: &str) -> AudioResult<String> {
    return AudioConverter::convert_audio_for_whisper(input_file, self.verbose)
      .await;
  }
}
