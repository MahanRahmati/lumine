//! Configuration management module with XDG compliance.
//!
//! This module handles loading, parsing, and accessing application configuration
//! from TOML files stored in XDG-compliant directories. It provides default values
//! for all settings and supports configuration reset operations.
//!
//! ## Configuration Sections
//!
//! - [`WhisperConfig`]: Whisper transcription service settings
//! - [`RecorderConfig`]: Audio recording parameters
//! - [`GeneralConfig`]: General application behavior settings
//!
//! ## Configuration File Location
//!
//! Configuration is loaded from:
//! - `$XDG_CONFIG_HOME/lumine/config.toml`
//! - Falls back to defaults if no config file exists

pub mod errors;

#[cfg(test)]
mod config_tests;

use std::path::PathBuf;

use xdg::BaseDirectories;

use crate::config::errors::{ConfigError, ConfigResult};
use crate::files::operations;

const DEFAULT_DIRECTORY: &str = "lumine";
const DEFAULT_CONFIG_NAME: &str = "config.toml";
const DEFAULT_USE_LOCAL: bool = true;
const DEFAULT_WHISPER_URL: &str = "http://127.0.0.1:9090";
const DEFAULT_SILENCE_LIMIT_SECONDS: i32 = 2;
const DEFAULT_SILENCE_DETECT_NOISE_DB: i32 = 40;
const DEFAULT_RECORDINGS_DIRECTORY: &str = "recordings";
const DEFAULT_REMOVE_AFTER_TRANSCRIPT: bool = true;
const DEFAULT_VERBOSE: bool = false;

/// Main configuration structure for the Lumine application.
///
/// This struct contains all configuration sections including Whisper settings,
/// recorder settings, and general application preferences.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
  pub whisper: WhisperConfig,
  pub recorder: RecorderConfig,
  pub general: GeneralConfig,
}

/// Configuration for the Whisper transcription service.
///
/// Contains settings for the Whisper API endpoint and model paths.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperConfig {
  pub use_local: Option<bool>,
  pub url: Option<String>,
  pub model_path: Option<String>,
  pub vad_model_path: Option<String>,
}

/// Configuration for audio recording functionality.
///
/// Contains settings for recording directory, silence detection, and device preferences.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RecorderConfig {
  pub recordings_directory: Option<String>,
  pub silence_limit: Option<i32>,
  pub silence_detect_noise: Option<i32>,
  pub preferred_audio_input_device: Option<String>,
}

/// General application configuration.
///
/// Contains settings that affect overall application behavior.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GeneralConfig {
  pub remove_after_transcript: Option<bool>,
  pub verbose: Option<bool>,
}

impl Config {
  /// Loads configuration from XDG-compliant config directory.
  ///
  /// Attempts to read and parse the configuration file from the standard
  /// XDG config location. If no config file exists, returns default configuration.
  ///
  /// # Returns
  ///
  /// A `ConfigResult<Config>` containing the loaded configuration or an error.
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

  /// Gets whether to use local Whisper transcription.
  ///
  /// Returns the configured setting or true. When enabled, the application
  /// will use the local Whisper model for transcription.
  ///
  /// # Returns
  ///
  /// A `bool` indicating whether to use local transcription mode.
  pub fn get_use_local(&self) -> bool {
    return self.whisper.use_local.unwrap_or(DEFAULT_USE_LOCAL);
  }

  /// Gets the Whisper service URL.
  ///
  /// Returns the configured URL or the default localhost URL if not set.
  ///
  /// # Returns
  ///
  /// A `String` containing the Whisper service URL.
  pub fn get_whisper_url(&self) -> String {
    return self
      .whisper
      .url
      .clone()
      .unwrap_or(String::from(DEFAULT_WHISPER_URL));
  }

  /// Gets the Whisper model file path.
  ///
  /// Returns the configured model path or an empty string if not set.
  ///
  /// # Returns
  ///
  /// A `String` containing the path to the Whisper model file.
  pub fn get_whisper_model_path(&self) -> String {
    return self.whisper.model_path.clone().unwrap_or_default();
  }

  /// Gets the Voice Activity Detection (VAD) model file path.
  ///
  /// Returns the configured VAD model path or an empty string if not set.
  ///
  /// # Returns
  ///
  /// A `String` containing the path to the VAD model file.
  pub fn get_vad_model_path(&self) -> String {
    return self.whisper.vad_model_path.clone().unwrap_or_default();
  }

  /// Gets the recordings directory path.
  ///
  /// Returns the configured recordings directory or creates an XDG-compliant
  /// data directory if not configured. Falls back to default directory name
  /// if XDG directory creation fails.
  ///
  /// # Returns
  ///
  /// A `String` containing the path to the recordings directory.
  pub fn get_recordings_directory(&self) -> String {
    if let Some(dir) = &self.recorder.recordings_directory
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

  /// Gets the silence detection limit in seconds.
  ///
  /// Returns the configured silence limit or the default value of 2 seconds.
  /// This determines how long silence should be detected before stopping recording.
  ///
  /// # Returns
  ///
  /// An `i32` containing the silence limit in seconds.
  pub fn get_silence_limit(&self) -> i32 {
    return self
      .recorder
      .silence_limit
      .unwrap_or(DEFAULT_SILENCE_LIMIT_SECONDS);
  }

  /// Gets the silence detection noise threshold in decibels.
  ///
  /// Returns the configured noise threshold or the default value of 40 dB.
  /// Audio levels below this threshold are considered silence.
  ///
  /// # Returns
  ///
  /// An `i32` containing the noise detection threshold in dB.
  pub fn get_silence_detect_noise(&self) -> i32 {
    return self
      .recorder
      .silence_detect_noise
      .unwrap_or(DEFAULT_SILENCE_DETECT_NOISE_DB);
  }

  /// Gets the preferred audio input device name.
  ///
  /// Returns the configured device name or an empty string if not set.
  /// When set, this device will be prioritized for audio recording.
  ///
  /// # Returns
  ///
  /// A `String` containing the preferred audio input device name.
  pub fn get_preferred_audio_input_device(&self) -> String {
    return self
      .recorder
      .preferred_audio_input_device
      .clone()
      .unwrap_or_default();
  }

  /// Gets whether to remove audio files after transcription.
  ///
  /// Returns the configured setting or the default value of true.
  /// When enabled, recorded audio files are deleted after successful transcription.
  ///
  /// # Returns
  ///
  /// A `bool` indicating whether to remove files after transcription.
  pub fn get_remove_after_transcript(&self) -> bool {
    return self
      .general
      .remove_after_transcript
      .unwrap_or(DEFAULT_REMOVE_AFTER_TRANSCRIPT);
  }

  /// Gets whether verbose output is enabled.
  ///
  /// Returns the configured verbose setting or false if not set.
  /// When enabled, provides detailed output for debugging and monitoring.
  ///
  /// # Returns
  ///
  /// A `bool` indicating whether verbose mode is enabled.
  pub fn get_verbose(&self) -> bool {
    return self.general.verbose.unwrap_or(false);
  }

  /// Resets the configuration to default values and saves it.
  ///
  /// Creates a new default configuration and saves it to the XDG config directory,
  /// overwriting any existing configuration file.
  ///
  /// # Returns
  ///
  /// A `ConfigResult<()>` indicating success or failure.
  pub async fn reset_to_defaults() -> ConfigResult<()> {
    let default_config = Config::default();
    return save_config(default_config).await;
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

async fn save_config(config: Config) -> ConfigResult<()> {
  let xdg_dirs = BaseDirectories::with_prefix(DEFAULT_DIRECTORY);

  let config_path = match xdg_dirs.place_config_file(DEFAULT_CONFIG_NAME) {
    Ok(path) => path,
    Err(e) => return Err(ConfigError::FileRead(e.to_string())),
  };

  let config_content = match toml::to_string_pretty(&config) {
    Ok(content) => content,
    Err(e) => return Err(ConfigError::Parse(e.to_string())),
  };

  match tokio::fs::write(&config_path, config_content).await {
    Ok(_) => return Ok(()),
    Err(e) => return Err(ConfigError::FileRead(e.to_string())),
  };
}

impl Default for Config {
  fn default() -> Self {
    return Config {
      whisper: WhisperConfig {
        use_local: Some(DEFAULT_USE_LOCAL),
        url: Some(String::from(DEFAULT_WHISPER_URL)),
        model_path: Some(String::new()),
        vad_model_path: Some(String::new()),
      },
      recorder: RecorderConfig {
        recordings_directory: Some(String::new()),
        silence_limit: Some(DEFAULT_SILENCE_LIMIT_SECONDS),
        silence_detect_noise: Some(DEFAULT_SILENCE_DETECT_NOISE_DB),
        preferred_audio_input_device: Some(String::new()),
      },
      general: GeneralConfig {
        remove_after_transcript: Some(DEFAULT_REMOVE_AFTER_TRANSCRIPT),
        verbose: Some(DEFAULT_VERBOSE),
      },
    };
  }
}
