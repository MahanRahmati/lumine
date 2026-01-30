use crate::audio::devices::{AudioInputDevice, AudioInputDevices};
use crate::audio::errors::AudioResult;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

pub trait AudioPlatform {
  /// Get list of available audio input devices
  ///
  /// # Arguments
  ///
  /// * `verbose` - Whether to print verbose output during device detection
  ///
  /// # Returns
  ///
  /// List of available audio input devices or error
  async fn get_audio_input_devices(
    &self,
    verbose: bool,
  ) -> AudioResult<AudioInputDevices>;

  /// Select an audio input device based on the provided list of devices
  ///
  /// # Arguments
  ///
  /// * `devices` - List of available audio input devices
  /// * `preferred_audio_input_device` - Preferred device name
  /// * `verbose` - Whether to print verbose output during device selection
  ///
  /// # Returns
  ///
  /// Audio input device or default device
  async fn select_audio_input_device(
    &self,
    devices: AudioInputDevices,
    preferred_audio_input_device: String,
    verbose: bool,
  ) -> AudioInputDevice;

  /// Build arguments for recording audio with ffmpeg
  ///
  /// # Arguments
  ///
  /// * `device_index` - Platform-specific device identifier
  /// * `silence_limit` - Seconds of silence before stopping
  /// * `silence_detect_noise` - Noise threshold in dB for silence detection
  /// * `max_recording_duration` - Maximum recording duration in seconds (0 = unlimited)
  /// * `output_file` - Path to output audio file
  ///
  /// # Returns
  ///
  /// Vector of FFmpeg command arguments
  fn build_ffmpeg_recording_arguments(
    &self,
    device_index: String,
    silence_limit: i32,
    silence_detect_noise: i32,
    max_recording_duration: i32,
    output_file: String,
  ) -> Vec<String>;
}

/// Get platform-specific audio platform implementation
///
/// # Returns
///
/// Concrete platform type for current compilation target
pub fn get_platform() -> impl AudioPlatform {
  #[cfg(target_os = "macos")]
  {
    return macos::MacOSPlatform::new();
  }

  #[cfg(target_os = "linux")]
  {
    return linux::LinuxPlatform::new();
  }

  #[cfg(not(any(target_os = "macos", target_os = "linux")))]
  compile_error!("Unsupported platform");
}
