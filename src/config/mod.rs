pub mod errors;

#[cfg(test)]
mod config_tests;

use std::path::PathBuf;

use xdg::BaseDirectories;

use crate::config::errors::{ConfigError, ConfigResult};
use crate::files::operations;

const DEFAULT_DIRECTORY: &str = "lumine";
const DEFAULT_CONFIG_NAME: &str = "config.toml";
const DEFAULT_WHISPER_URL: &str = "http://127.0.0.1:9090";
const DEFAULT_SILENCE_LIMIT_SECONDS: i32 = 2;
const DEFAULT_SILENCE_DETECT_NOISE_DB: i32 = 40;
const DEFAULT_RECORDINGS_DIRECTORY: &str = "recordings";
const DEFAULT_REMOVE_AFTER_TRANSSRIPT: bool = true;
const DEFAULT_VERBOSE: bool = false;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
  pub whisper: WhisperConfig,
  pub ffmpeg: FFMPEGConfig,
  pub general: GeneralConfig,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperConfig {
  pub url: Option<String>,
  pub model_path: Option<String>,
  pub vad_model_path: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FFMPEGConfig {
  pub recordings_directory: Option<String>,
  pub silence_limit: Option<i32>,
  pub silence_detect_noise: Option<i32>,
  pub preferred_audio_input_device: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GeneralConfig {
  pub remove_after_transcript: Option<bool>,
  pub verbose: Option<bool>,
}

impl Config {
  pub async fn load() -> ConfigResult<Config> {
    let xdg_dirs = BaseDirectories::with_prefix(DEFAULT_DIRECTORY);
    let config_path = match xdg_dirs.find_config_file(DEFAULT_CONFIG_NAME) {
      Some(path) => path,
      None => {
        let default_config = Config::default();
        return Ok(default_config);
      }
    };
    let config_content = get_config_content(config_path).await?;
    let config = parse_config_content(config_content)?;
    return Ok(config);
  }

  pub fn get_whisper_url(&self) -> String {
    return self
      .whisper
      .url
      .clone()
      .unwrap_or(String::from(DEFAULT_WHISPER_URL));
  }

  pub fn get_whisper_model_path(&self) -> String {
    return self.whisper.model_path.clone().unwrap_or_default();
  }

  pub fn get_vad_model_path(&self) -> String {
    return self.whisper.vad_model_path.clone().unwrap_or_default();
  }

  pub fn get_recordings_directory(&self) -> String {
    if let Some(dir) = &self.ffmpeg.recordings_directory
      && !dir.is_empty()
    {
      return dir.clone();
    }

    let xdg_dirs = BaseDirectories::with_prefix(DEFAULT_DIRECTORY);
    return xdg_dirs
      .create_data_directory(DEFAULT_RECORDINGS_DIRECTORY)
      .map(|path| path.to_string_lossy().to_string())
      .unwrap_or_else(|_| String::from(DEFAULT_RECORDINGS_DIRECTORY));
  }

  pub fn get_silence_limit(&self) -> i32 {
    return self
      .ffmpeg
      .silence_limit
      .unwrap_or(DEFAULT_SILENCE_LIMIT_SECONDS);
  }

  pub fn get_silence_detect_noise(&self) -> i32 {
    return self
      .ffmpeg
      .silence_detect_noise
      .unwrap_or(DEFAULT_SILENCE_DETECT_NOISE_DB);
  }

  pub fn get_preferred_audio_input_device(&self) -> String {
    return self
      .ffmpeg
      .preferred_audio_input_device
      .clone()
      .unwrap_or_default();
  }

  pub fn get_remove_after_transcript(&self) -> bool {
    return self
      .general
      .remove_after_transcript
      .unwrap_or(DEFAULT_REMOVE_AFTER_TRANSSRIPT);
  }

  pub fn get_verbose(&self) -> bool {
    return self.general.verbose.unwrap_or(false);
  }
}

async fn get_config_content(config_path: PathBuf) -> ConfigResult<String> {
  return operations::read_to_string(&config_path.to_string_lossy())
    .await
    .map_err(|e| ConfigError::FileRead(e.to_string()));
}

fn parse_config_content(config_content: String) -> ConfigResult<Config> {
  return toml::from_str(&config_content)
    .map_err(|e| ConfigError::Parse(e.to_string()));
}

impl Default for Config {
  fn default() -> Self {
    return Config {
      whisper: WhisperConfig {
        url: Some(String::from(DEFAULT_WHISPER_URL)),
        model_path: Some(String::new()),
        vad_model_path: Some(String::new()),
      },
      ffmpeg: FFMPEGConfig {
        recordings_directory: Some(String::new()),
        silence_limit: Some(DEFAULT_SILENCE_LIMIT_SECONDS),
        silence_detect_noise: Some(DEFAULT_SILENCE_DETECT_NOISE_DB),
        preferred_audio_input_device: Some(String::new()),
      },
      general: GeneralConfig {
        remove_after_transcript: Some(DEFAULT_REMOVE_AFTER_TRANSSRIPT),
        verbose: Some(DEFAULT_VERBOSE),
      },
    };
  }
}
