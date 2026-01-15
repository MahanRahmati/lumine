use super::files::operations;
use xdg::BaseDirectories;

pub mod errors;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
  pub whisper: WhisperConfig,
  pub ffmpeg: FFMPEGConfig,
  pub general: GeneralConfig,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WhisperConfig {
  pub url: String,
  pub verbose: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FFMPEGConfig {
  pub recordings_directory: Option<String>,
  pub silence_limit: Option<i32>,
  pub silence_detect_noise: Option<i32>,
  pub preferred_audio_input_device: Option<String>,
  pub verbose: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GeneralConfig {
  pub remove_after_transcript: Option<bool>,
  pub verbose: Option<bool>,
}

impl Config {
  pub fn load() -> Result<Config, ConfigError> {
    let xdg_dirs = BaseDirectories::with_prefix("lumine");

    let config_path = match xdg_dirs.find_config_file("lumine.toml") {
      Some(path) => path,
      None => {
        let default_config = Config::default();
        let config_path: std::path::PathBuf =
          match xdg_dirs.place_config_file("lumine.toml") {
            Ok(path) => path,
            Err(e) => {
              return Err(ConfigError::FileWrite(format!(
                "Failed to place config file: {}",
                e
              )));
            }
          };

        if let Some(parent) = config_path.parent()
          && let Err(e) =
            operations::create_directory_all(&parent.to_string_lossy())
        {
          return Err(ConfigError::FileWrite(format!(
            "Failed to create config directory: {}",
            e
          )));
        }

        default_config.save_to_file(&config_path.to_string_lossy())?;
        return Ok(default_config);
      }
    };

    let config_content =
      match operations::read_to_string(&config_path.to_string_lossy()) {
        Ok(content) => content,
        Err(e) => {
          return Err(ConfigError::FileRead(e.to_string()));
        }
      };

    let config: Config = match toml::from_str(&config_content) {
      Ok(config) => config,
      Err(e) => {
        return Err(ConfigError::Parse(e.to_string()));
      }
    };

    return Ok(config);
  }

  fn save_to_file(&self, path: &str) -> Result<(), ConfigError> {
    let toml_string = toml::to_string_pretty(self)
      .map_err(|e| ConfigError::Serialize(e.to_string()))?;

    operations::write_string(path, &toml_string)
      .map_err(|e| ConfigError::FileWrite(e.to_string()))?;

    return Ok(());
  }

  pub fn get_whisper_url(&self) -> String {
    self.whisper.url.clone()
  }

  pub fn get_verbose(&self) -> bool {
    self
      .general
      .verbose
      .or(self.whisper.verbose)
      .or(self.ffmpeg.verbose)
      .unwrap_or(false)
  }

  pub fn get_recordings_directory(&self) -> String {
    self
      .ffmpeg
      .recordings_directory
      .clone()
      .unwrap_or_else(|| String::from("recordings"))
  }

  pub fn get_remove_after_transcript(&self) -> bool {
    self.general.remove_after_transcript.unwrap_or(true)
  }

  pub fn get_silence_limit(&self) -> i32 {
    self.ffmpeg.silence_limit.unwrap_or(2)
  }

  pub fn get_silence_detect_noise(&self) -> i32 {
    self.ffmpeg.silence_detect_noise.unwrap_or(40)
  }

  pub fn get_preferred_audio_input_device(&self) -> String {
    self
      .ffmpeg
      .preferred_audio_input_device
      .clone()
      .unwrap_or_default()
  }
}

pub use errors::ConfigError;

impl Default for Config {
  fn default() -> Self {
    return Config {
      whisper: WhisperConfig {
        url: String::from("http://127.0.0.1:9090"),
        verbose: Some(true),
      },
      ffmpeg: FFMPEGConfig {
        recordings_directory: Some(String::from("recordings")),
        silence_limit: Some(2),
        silence_detect_noise: Some(40),
        preferred_audio_input_device: Some(String::new()),
        verbose: Some(true),
      },
      general: GeneralConfig {
        remove_after_transcript: Some(true),
        verbose: Some(true),
      },
    };
  }
}
