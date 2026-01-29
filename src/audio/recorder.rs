use std::os::unix::process::ExitStatusExt;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::audio::devices::AudioInputDevice;
use crate::audio::errors::{AudioError, AudioResult};
use crate::audio::platform::AudioPlatform;
use crate::files::operations;

#[derive(Debug, Clone)]
pub struct AudioRecorder<P: AudioPlatform> {
  recordings_directory: String,
  silence_limit: i32,
  silence_detect_noise: i32,
  preferred_audio_input_device: String,
  verbose: bool,
  platform: P,
}

impl<P: AudioPlatform> AudioRecorder<P> {
  pub fn new(
    recordings_directory: String,
    silence_limit: i32,
    silence_detect_noise: i32,
    preferred_audio_input_device: String,
    verbose: bool,
    platform: P,
  ) -> Self {
    return Self {
      recordings_directory,
      silence_limit,
      silence_detect_noise,
      preferred_audio_input_device,
      verbose,
      platform,
    };
  }

  pub async fn record_audio(&self) -> AudioResult<String> {
    self.check_ffmpeg().await?;
    let devices = self.platform.get_audio_input_devices(self.verbose).await?;
    let device = self
      .platform
      .select_audio_input_device(
        devices,
        self.preferred_audio_input_device.clone(),
        self.verbose,
      )
      .await;
    return self.record_audio_with_device(device).await;
  }

  async fn check_ffmpeg(&self) -> AudioResult<bool> {
    let output = tokio::process::Command::new("ffmpeg")
      .args(["-version"])
      .output()
      .await
      .map_err(|_| AudioError::FFMPEGNotFound)?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if line.contains("ffmpeg version") {
        if self.verbose {
          println!("Found ffmpeg: {}", line);
        }
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
      output_file.clone(),
    );

    let mut child = Command::new("ffmpeg")
      .args(args)
      .stderr(Stdio::piped())
      .spawn()
      .map_err(|_| AudioError::CouldNotExecuteFFMPEG)?;

    if self.verbose {
      println!("Recording audio to: {}", output_file);
    }

    if self.verbose {
      println!(
        "Recording... will stop after {}s of silence",
        self.silence_limit
      );
    }

    let stderr = child
      .stderr
      .take()
      .ok_or(AudioError::CouldNotReadFFMPEGOutput)?;

    let mut reader = BufReader::new(stderr).lines();

    let verbose = self.verbose;
    let silence_limit = self.silence_limit;
    let child_mutex = Arc::new(Mutex::new(child));
    let mut timer_handle: Option<JoinHandle<()>> = None;

    while let Ok(Some(line)) = reader.next_line().await {
      if line.contains("silence_start") {
        if verbose {
          println!(
            "Possible silence detected... starting {}s countdown.",
            silence_limit
          );
        }

        let child_for_timer = Arc::clone(&child_mutex);
        timer_handle = Some(tokio::spawn(async move {
          tokio::time::sleep(Duration::from_secs(silence_limit as u64)).await;
          if verbose {
            println!("Silence limit reached. Stopping recording...");
          }
          let _ = child_for_timer.lock().await.kill().await;
        }));
      }

      if line.contains("silence_end") {
        if verbose {
          println!("Sound detected. Resetting silence timer.");
        }
        if let Some(handle) = timer_handle.take() {
          handle.abort();
        }
      }
    }

    if verbose {
      println!("Recording ended.");
    }

    if let Ok(status) = child_mutex.lock().await.wait().await
      && !status.success()
      && status.code() != Some(255)
      && status.signal() != Some(9)
    {
      if self.verbose {
        println!("Process failed with exit code: {:?}", status.code());
      }
      return Err(AudioError::CouldNotExecuteFFMPEG);
    }

    if self.verbose {
      println!("Recording saved to {}", output_file);
    }

    return Ok(output_file);
  }
}
