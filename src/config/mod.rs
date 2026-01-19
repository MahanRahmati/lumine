pub mod errors;

#[cfg(test)]
mod config_tests;

use std::path::PathBuf;

use xdg::BaseDirectories;

use crate::config::errors::{ConfigError, ConfigResult};
use crate::files::operations;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
  pub whisper: WhisperConfig,
  pub ffmpeg: FFMPEGConfig,
  pub general: GeneralConfig,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperConfig {
  pub url: Option<String>,
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
    let xdg_dirs = BaseDirectories::with_prefix("lumine");
    let config_path = match xdg_dirs.find_config_file("config.toml") {
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
      .unwrap_or(String::from("http://127.0.0.1:9090"));
  }

  pub fn get_verbose(&self) -> bool {
    return self.general.verbose.unwrap_or(false);
  }

  pub fn get_recordings_directory(&self) -> String {
    if let Some(dir) = &self.ffmpeg.recordings_directory
      && !dir.is_empty()
    {
      return dir.clone();
    }

    let xdg_dirs = BaseDirectories::with_prefix("lumine");
    return xdg_dirs
      .create_data_directory("recordings")
      .map(|path| path.to_string_lossy().to_string())
      .unwrap_or_else(|_| String::from("recordings"));
  }

  pub fn get_remove_after_transcript(&self) -> bool {
    return self.general.remove_after_transcript.unwrap_or(true);
  }

  pub fn get_silence_limit(&self) -> i32 {
    return self.ffmpeg.silence_limit.unwrap_or(2);
  }

  pub fn get_silence_detect_noise(&self) -> i32 {
    return self.ffmpeg.silence_detect_noise.unwrap_or(40);
  }

  pub fn get_preferred_audio_input_device(&self) -> String {
    return self
      .ffmpeg
      .preferred_audio_input_device
      .clone()
      .unwrap_or_default();
  }
}

async fn get_config_content(config_path: PathBuf) -> ConfigResult<String> {
  match operations::read_to_string(&config_path.to_string_lossy()).await {
    Ok(content) => return Ok(content),
    Err(e) => {
      return Err(ConfigError::FileRead(e.to_string()));
    }
  };
}

fn parse_config_content(config_content: String) -> ConfigResult<Config> {
  match toml::from_str(&config_content) {
    Ok(config) => return Ok(config),
    Err(e) => {
      return Err(ConfigError::Parse(e.to_string()));
    }
  };
}

impl Default for Config {
  fn default() -> Self {
    return Config {
      whisper: WhisperConfig {
        url: Some(String::from("http://127.0.0.1:9090")),
      },
      ffmpeg: FFMPEGConfig {
        recordings_directory: Some(String::new()),
        silence_limit: Some(2),
        silence_detect_noise: Some(40),
        preferred_audio_input_device: Some(String::new()),
      },
      general: GeneralConfig {
        remove_after_transcript: Some(true),
        verbose: Some(false),
      },
    };
  }
}
