use regex::Regex;

use crate::audio::devices::{AudioInputDevice, AudioInputDevices};
use crate::audio::errors::{AudioError, AudioResult};
use crate::audio::platform::AudioPlatform;

/// Linux implementation of AudioPlatform trait.
pub struct LinuxPlatform {}

impl LinuxPlatform {
  /// Creates a new LinuxPlatform instance.
  ///
  /// # Returns
  ///
  /// A new `LinuxPlatform` instance.
  pub fn new() -> Self {
    return Self {};
  }
}

impl AudioPlatform for LinuxPlatform {
  async fn get_audio_input_devices(
    &self,
    verbose: bool,
  ) -> AudioResult<AudioInputDevices> {
    let output = tokio::process::Command::new("ffmpeg")
      .args(["-sources", "pulse"])
      .output()
      .await
      .map_err(|_| AudioError::CouldNotExecuteFFMPEG)?;

    let output_str = String::from_utf8_lossy(&output.stderr);
    let mut audio_section = false;
    let mut devices: AudioInputDevices = Vec::new();

    let regex = Regex::new(r"^\s*(?:\*\s)?([^\s]+)\s+\[([^\]]+)\]").unwrap();

    for line in output_str.lines() {
      if line.contains("Auto-detected sources for pulse") {
        audio_section = true;
        continue;
      }

      if audio_section
        && line.contains("_input")
        && let Some(caps) = regex.captures(line)
      {
        let index = caps.get(1).unwrap().as_str();
        let name = caps.get(2).unwrap().as_str();
        devices.push(AudioInputDevice::new(
          String::from(index),
          String::from(name),
        ));
      }
    }

    if verbose {
      println!("Audio Devices Found:");
      for device in &devices {
        println!("- {}", device.get_name());
      }
    }

    return Ok(devices);
  }

  async fn select_audio_input_device(
    &self,
    devices: AudioInputDevices,
    preferred_audio_input_device: String,
    verbose: bool,
  ) -> AudioInputDevice {
    let default_device = AudioInputDevice::default();

    if preferred_audio_input_device.is_empty() {
      if verbose {
        println!(
          "No preferred audio input device specified, using default device"
        );
      }
      return default_device;
    }

    for device in devices {
      if device.get_name().contains(&preferred_audio_input_device) {
        if verbose {
          println!(
            "Selected preferred audio input device: {}",
            device.get_name()
          );
        }
        return device;
      }
    }

    if verbose {
      println!("No preferred audio input device found, using default device");
    }

    return default_device;
  }

  fn build_ffmpeg_recording_arguments(
    &self,
    device_index: String,
    silence_limit: i32,
    silence_detect_noise: i32,
    output_file: String,
  ) -> Vec<String> {
    let args = vec![
      "-f".to_string(),
      "pulse".to_string(),
      "-i".to_string(),
      format!(":{}", device_index),
      "-acodec".to_string(),
      "pcm_s16le".to_string(),
      "-af".to_string(),
      format!(
        "silencedetect=n=-{}dB:d={}",
        silence_detect_noise, silence_limit,
      ),
      output_file,
      "-y".to_string(),
    ];
    return args;
  }
}
