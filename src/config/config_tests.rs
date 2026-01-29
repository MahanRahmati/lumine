use crate::config::*;

const VALID_CONFIG: &str = r#"
[whisper]
use_local = true
url = "http://localhost:8080"
model_path = ""
vad_model_path = ""

[recorder]
recordings_directory = "test_recordings"
silence_limit = 5
silence_detect_noise = 30
preferred_audio_input_device = "test_device"

[general]
remove_after_transcript = true
verbose = false
"#;

const INVALID_CONFIG: &str = r#"
[whisper]
use_local = true
url = "http://localhost:8080"
model_path = ""
vad_model_path = ""

[recorder]
recordings_directory = "test_recordings"
# Missing required fields and invalid TOML syntax
silence_limit =

[general]
remove_after_transcript = not_a_boolean
verbose = false
"#;

#[test]
fn test_config_default() {
  let config = Config::default();
  assert!(config.get_use_local());
  assert_eq!(config.get_whisper_url(), "http://127.0.0.1:9090");
  let recordings_dir = config.get_recordings_directory();
  assert!(recordings_dir.contains("recordings"));
  assert!(
    recordings_dir.contains(".local") || recordings_dir.contains("share")
  );
  assert_eq!(config.get_silence_limit(), 2);
  assert_eq!(config.get_silence_detect_noise(), 40);
  assert_eq!(config.get_preferred_audio_input_device(), "");
  assert!(config.get_remove_after_transcript());
  assert!(!config.get_verbose());
}

#[tokio::test]
async fn test_get_config_content() {
  let temp_dir = std::env::temp_dir();
  let config_path = temp_dir.join("test_config.toml");
  std::fs::write(&config_path, VALID_CONFIG).unwrap();

  let result = get_config_content(config_path).await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), VALID_CONFIG);

  std::fs::remove_file(temp_dir.join("test_config.toml")).unwrap();
}

#[tokio::test]
async fn test_get_config_content_with_wrong_path() {
  let wrong_path = std::path::PathBuf::from("/non-existent-path/config.toml");
  let result = get_config_content(wrong_path).await;
  assert!(result.is_err());
  match result.unwrap_err() {
    ConfigError::FileRead(_) => (),
    _ => panic!("Expected FileRead error"),
  }
}

#[test]
fn test_parse_config_content() {
  let result = parse_config_content(VALID_CONFIG.to_string());
  assert!(result.is_ok());

  let config = result.unwrap();
  assert!(config.get_use_local());
  assert_eq!(config.get_whisper_url(), "http://localhost:8080");
  assert_eq!(config.get_recordings_directory(), "test_recordings");
  assert_eq!(config.get_silence_limit(), 5);
  assert_eq!(config.get_silence_detect_noise(), 30);
  assert_eq!(config.get_preferred_audio_input_device(), "test_device");
  assert!(config.get_remove_after_transcript());
  assert!(!config.get_verbose());
}

#[test]
fn test_parse_config_content_with_wrong_content() {
  let result = parse_config_content(INVALID_CONFIG.to_string());
  assert!(result.is_err());
  match result.unwrap_err() {
    ConfigError::Parse(_) => (),
    _ => panic!("Expected Parse error"),
  }
}

#[tokio::test]
async fn test_config_reset_to_defaults() {
  let result = Config::reset_to_defaults().await;
  assert!(result.is_ok(), "Reset to defaults should succeed");

  let config = Config::load().await.unwrap();
  assert!(config.get_use_local());
  assert_eq!(config.get_whisper_url(), "http://127.0.0.1:9090");
  assert_eq!(config.get_silence_limit(), 2);
  assert_eq!(config.get_silence_detect_noise(), 40);
  assert_eq!(config.get_preferred_audio_input_device(), "");
  assert!(config.get_remove_after_transcript());
  assert!(!config.get_verbose());
}
