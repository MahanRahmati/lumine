//! Audio recording and format conversion module.
//!
//! This module handles all audio-related operations including device detection,
//! audio recording with silence detection, and format conversion for Whisper
//! compatibility. It provides platform-specific implementations for macOS and Linux.
//!
//! ## Module Structure
//!
//! - [`Audio`]: Main coordinator for recording and conversion operations
//! - [`AudioRecorder`]: Platform-specific audio recording implementation
//! - [`AudioConverter`]: Audio format conversion to Whisper-compatible format
//! - [`AudioInputDevice`]: Represents available audio input devices
//! - [`AudioPlatform`]: Platform abstraction trait (macOS/Linux)
//!
//! ## Platform Support
//!
//! - **macOS**: Uses AVFoundation framework via FFmpeg
//! - **Linux**: Uses PulseAudio via FFmpeg
//! - **Windows**: Not supported (compile-time error)

mod converter;
mod devices;
mod errors;
mod platform;
mod recorder;

use crate::audio::converter::AudioConverter;
use crate::audio::errors::AudioResult;
use crate::audio::platform::get_platform;
use crate::audio::recorder::AudioRecorder;

/// Main audio recording and conversion coordinator.
///
/// Coordinates audio recording and format conversion operations using platform-specific
/// implementations and configured settings.
#[derive(Debug, Clone)]
pub struct Audio {
  recordings_directory: String,
  silence_limit: i32,
  silence_detect_noise: i32,
  preferred_audio_input_device: String,
  max_recording_duration: i32,
}

impl Audio {
  /// Creates a new Audio instance with recording configuration.
  ///
  /// # Arguments
  ///
  /// * `recordings_directory` - Directory path to save audio recordings
  /// * `silence_limit` - Seconds of silence before stopping recording
  /// * `silence_detect_noise` - Noise threshold in decibels for silence detection
  /// * `preferred_audio_input_device` - Name of preferred audio input device
  /// * `max_recording_duration` - Maximum recording duration in seconds (0 for unlimited)
  ///
  /// # Returns
  ///
  /// A new `Audio` instance configured with the provided settings.
  pub fn new(
    recordings_directory: String,
    silence_limit: i32,
    silence_detect_noise: i32,
    preferred_audio_input_device: String,
    max_recording_duration: i32,
  ) -> Self {
    return Audio {
      recordings_directory,
      silence_limit,
      silence_detect_noise,
      preferred_audio_input_device,
      max_recording_duration,
    };
  }

  /// Records audio using configured settings and platform implementation.
  ///
  /// Delegates to a platform-specific AudioRecorder for actual recording
  /// with silence detection and device management.
  ///
  /// # Returns
  ///
  /// An `AudioResult<String>` containing the path to the recorded audio file
  /// or an error if recording failed.
  pub async fn record_audio(&self) -> AudioResult<String> {
    let recorder = AudioRecorder::new(
      self.recordings_directory.clone(),
      self.silence_limit,
      self.silence_detect_noise,
      self.preferred_audio_input_device.clone(),
      self.max_recording_duration,
      get_platform(),
    );
    return recorder.record_audio().await;
  }

  /// Converts audio input file to Whisper-compatible format.
  ///
  /// Delegates to AudioConverter to transform input audio to 16kHz mono WAV
  /// format required by Whisper transcription service.
  ///
  /// # Arguments
  ///
  /// * `input_file` - Path to the audio file to convert
  ///
  /// # Returns
  ///
  /// An `AudioResult<String>` containing the path to the converted audio file
  /// or an error if conversion failed.
  pub async fn convert_audio(&self, input_file: &str) -> AudioResult<String> {
    return AudioConverter::convert_audio_for_whisper(input_file).await;
  }
}
