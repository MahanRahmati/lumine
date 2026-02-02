use std::os::unix::process::ExitStatusExt;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::audio::devices::AudioInputDevice;
use crate::audio::errors::{AudioError, AudioResult};
use crate::audio::platform::AudioPlatform;
use crate::files::operations;
use crate::process::executor::ProcessExecutor;
use crate::vlog;

/// Generic audio recorder with platform-specific implementation.
///
/// Records audio using FFmpeg with silence detection and device management
/// through platform-specific AudioPlatform implementations.
#[derive(Debug, Clone)]
pub(crate) struct AudioRecorder<P: AudioPlatform> {
  recordings_directory: String,
  silence_limit: i32,
  silence_detect_noise: i32,
  preferred_audio_input_device: String,
  max_recording_duration: i32,
  platform: P,
}

impl<P: AudioPlatform> AudioRecorder<P> {
  /// Creates a new AudioRecorder with configuration and platform implementation.
  ///
  /// # Arguments
  ///
  /// * `recordings_directory` - Directory path to save audio recordings
  /// * `silence_limit` - Seconds of silence before stopping recording
  /// * `silence_detect_noise` - Noise threshold in decibels for silence detection
  /// * `preferred_audio_input_device` - Name of preferred audio input device
  /// * `max_recording_duration` - Maximum recording duration in seconds (0 for unlimited)
  /// * `platform` - Platform-specific implementation for audio operations
  ///
  /// # Returns
  ///
  /// A new `AudioRecorder<P>` instance.
  pub fn new(
    recordings_directory: String,
    silence_limit: i32,
    silence_detect_noise: i32,
    preferred_audio_input_device: String,
    max_recording_duration: i32,
    platform: P,
  ) -> Self {
    return Self {
      recordings_directory,
      silence_limit,
      silence_detect_noise,
      preferred_audio_input_device,
      max_recording_duration,
      platform,
    };
  }

  /// Records audio with silence detection using platform-specific implementation.
  ///
  /// Validates FFmpeg availability, selects appropriate audio device, and records
  /// audio with automatic silence detection based on configured thresholds.
  ///
  /// # Returns
  ///
  /// An `AudioResult<String>` containing the path to the recorded audio file
  /// or an error if recording failed.
  pub async fn record_audio(&self) -> AudioResult<String> {
    self.check_ffmpeg().await?;
    let devices = self.platform.get_audio_input_devices().await?;
    let device = self
      .platform
      .select_audio_input_device(
        devices,
        self.preferred_audio_input_device.clone(),
      )
      .await;
    return self.record_audio_with_device(device).await;
  }

  async fn check_ffmpeg(&self) -> AudioResult<bool> {
    let output = ProcessExecutor::run("ffmpeg", &["-version"])
      .await
      .map_err(|_| AudioError::FFMPEGNotFound)?;

    for line in output.stdout.lines() {
      if line.contains("ffmpeg version") {
        vlog!("Found ffmpeg: {}", line);
        return Ok(true);
      }
    }
    return Err(AudioError::FFMPEGNotFound);
  }

  async fn record_audio_with_device(
    &self,
    device: AudioInputDevice,
  ) -> AudioResult<String> {
    operations::create_directory_all(&self.recordings_directory)
      .await
      .map_err(|_| AudioError::CouldNotCreateDirectory)?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let output_file = format!(
      "{}/audiocapture_{}.wav",
      self.recordings_directory, timestamp
    );

    let args = self.platform.build_ffmpeg_recording_arguments(
      device.get_index().clone(),
      self.silence_limit,
      self.silence_detect_noise,
      self.max_recording_duration,
      output_file.clone(),
    );

    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let mut child =
      ProcessExecutor::spawn_with_stderr_piped("ffmpeg", &args_refs)
        .await
        .map_err(|_| AudioError::CouldNotExecuteFFMPEG)?;

    vlog!("Recording audio to: {}", output_file);
    vlog!(
      "Recording... will stop after {}s of silence",
      self.silence_limit
    );
    if self.max_recording_duration > 0 {
      vlog!(
        "Maximum recording duration: {} seconds",
        self.max_recording_duration
      );
    }

    let stderr = child
      .stderr
      .take()
      .ok_or(AudioError::CouldNotReadFFMPEGOutput)?;

    let mut reader = BufReader::new(stderr).lines();

    let silence_limit = self.silence_limit;
    let child_mutex = Arc::new(Mutex::new(child));
    let mut timer_handle: Option<JoinHandle<()>> = None;

    while let Ok(Some(line)) = reader.next_line().await {
      if line.contains("silence_start") {
        vlog!(
          "Possible silence detected... starting {}s countdown.",
          silence_limit
        );

        let child_for_timer = Arc::clone(&child_mutex);
        timer_handle = Some(tokio::spawn(async move {
          tokio::time::sleep(Duration::from_secs(silence_limit as u64)).await;
          vlog!("Silence limit reached. Stopping recording...");
          let _ = child_for_timer.lock().await.kill().await;
        }));
      }

      if line.contains("silence_end") {
        vlog!("Sound detected. Resetting silence timer.");
        if let Some(handle) = timer_handle.take() {
          handle.abort();
        }
      }
    }

    vlog!("Recording ended.");

    if let Ok(status) = child_mutex.lock().await.wait().await
      && !status.success()
      && status.code() != Some(255)
      && status.signal() != Some(9)
    {
      vlog!("Process failed with exit code: {:?}", status.code());
      return Err(AudioError::CouldNotExecuteFFMPEG);
    }

    vlog!("Recording saved to {}", output_file);

    return Ok(output_file);
  }
}
