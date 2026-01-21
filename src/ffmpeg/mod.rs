mod devices;
mod errors;

#[cfg(test)]
mod ffmpeg_tests;

use std::io::BufRead;
use std::os::unix::process::ExitStatusExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use regex::Regex;
use tokio::task;
use tokio::task::JoinHandle;

use crate::ffmpeg::devices::{AudioInputDevice, AudioInputDevices};
use crate::ffmpeg::errors::{FFMPEGError, FFMPEGResult};
use crate::files::operations;

#[derive(Debug, Clone)]
pub struct FFMPEG {
  recordings_directory: String,
  silence_limit: i32,
  silence_detect_noise: i32,
  prefered_audio_input_device: String,
  verbose: bool,
}

impl FFMPEG {
  pub fn new(
    recordings_directory: String,
    silence_limit: i32,
    silence_detect_noise: i32,
    prefered_audio_input_device: String,
    verbose: bool,
  ) -> Self {
    return FFMPEG {
      recordings_directory,
      silence_limit,
      silence_detect_noise,
      prefered_audio_input_device,
      verbose,
    };
  }

  pub async fn record_audio(&self) -> FFMPEGResult<String> {
    self.check_ffmpeg().await?;
    let devices = self.get_audio_input_devices().await?;
    let device = self.select_audio_input_device(devices);
    return self.record_audio_with_device(device).await;
  }

  async fn check_ffmpeg(&self) -> FFMPEGResult<bool> {
    let output = tokio::process::Command::new("ffmpeg")
      .args(["-version"])
      .output()
      .await
      .map_err(|_| FFMPEGError::NotFound)?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
      if line.contains("ffmpeg version") {
        if self.verbose {
          println!("Found ffmpeg: {}", line);
        }
        return Ok(true);
      }
    }
    return Err(FFMPEGError::NotFound);
  }

  async fn get_audio_input_devices(&self) -> FFMPEGResult<AudioInputDevices> {
    let output = tokio::process::Command::new("ffmpeg")
      .args(["-f", "avfoundation", "-list_devices", "true", "-i", ""])
      .output()
      .await
      .map_err(|_| FFMPEGError::CouldNotExecute)?;

    let output_str = String::from_utf8_lossy(&output.stderr);
    let mut audio_section = false;
    let mut devices = Vec::new();

    let regex = Regex::new(r"\[(\d+)\]\s+(.*)").unwrap();

    for line in output_str.lines() {
      if line.contains("AVFoundation audio devices") {
        audio_section = true;
        continue;
      }

      if audio_section
        && let Some(caps) = regex.captures(line)
        && caps.len() >= 3
      {
        let index = &caps[1];
        let name = &caps[2];
        devices.push(AudioInputDevice::new(
          String::from(index),
          String::from(name),
        ));
      }
    }

    if self.verbose {
      println!("Audio Devices Found:");
      for device in &devices {
        println!("- {}", device.get_name());
      }
    }

    return Ok(devices);
  }

  pub(crate) fn select_audio_input_device(
    &self,
    devices: AudioInputDevices,
  ) -> AudioInputDevice {
    let default_device = AudioInputDevice::default();

    if self.prefered_audio_input_device.is_empty() {
      if self.verbose {
        println!(
          "No preferred audio input device specified, using default device"
        );
      }
      return default_device;
    }

    for device in devices {
      if device
        .get_name()
        .contains(&self.prefered_audio_input_device)
      {
        if self.verbose {
          println!(
            "Selected preferred audio input device: {}",
            device.get_name()
          );
        }
        return device;
      }
    }

    if self.verbose {
      println!("No preferred audio input device found, using default device");
    }

    return default_device;
  }

  async fn record_audio_with_device(
    &self,
    device: AudioInputDevice,
  ) -> FFMPEGResult<String> {
    operations::create_directory_all(&self.recordings_directory)
      .await
      .map_err(|_| FFMPEGError::CouldNotCreateDirectory)?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let output_file = format!(
      "{}/audiocapture_{}.wav",
      self.recordings_directory, timestamp
    );

    let output = Command::new("ffmpeg")
      .args([
        "-f",
        "avfoundation",
        "-i",
        format!(":{}", device.get_index()).as_str(),
        "-acodec",
        "pcm_s16le",
        "-af",
        format!(
          "silencedetect=n=-{}dB:d={}",
          self.silence_detect_noise, self.silence_limit,
        )
        .as_str(),
        output_file.as_str(),
        "-y",
      ])
      .stderr(Stdio::piped())
      .spawn()
      .map_err(|_| FFMPEGError::CouldNotExecute)?;

    if self.verbose {
      println!("Recording audio to: {}", output_file);
    }

    if self.verbose {
      println!(
        "Recording... will stop after {}s of silence",
        self.silence_limit
      );
    }

    let child = Arc::new(Mutex::new(output));
    let child_clone = Arc::clone(&child);
    let stderr = child
      .lock()
      .unwrap()
      .stderr
      .take()
      .ok_or(FFMPEGError::CouldNotReadOutput)?;

    let mut reader = std::io::BufReader::new(stderr);

    let should_kill = Arc::new(Mutex::new(true));
    let should_kill_clone = Arc::clone(&should_kill);

    let verbose = self.verbose;
    let silence_limit = self.silence_limit;

    let handle = task::spawn_blocking(move || {
      let mut line = String::new();
      let mut _timer: Option<JoinHandle<()>> = None;

      while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
          break;
        }

        if line.contains("silence_start") {
          if verbose {
            println!(
              "Possible silence detected... starting {}s countdown.",
              silence_limit
            );
          }

          *should_kill.lock().unwrap() = true;

          let child_for_timer = Arc::clone(&child_clone);
          let kill_flag = Arc::clone(&should_kill_clone);
          _timer = Some(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(silence_limit as u64)).await;

            if *kill_flag.lock().unwrap() {
              if verbose {
                println!("Silence limit reached. Stopping recording...");
              }
              let _ = child_for_timer.lock().unwrap().kill();
            }
          }));
        }

        if line.contains("silence_end") {
          if verbose {
            println!("Sound detected. Resetting silence timer.");
          }
          *should_kill.lock().unwrap() = false;
          _timer = None;
        }

        line.clear();
      }

      if verbose {
        println!("Recording ended.");
      }
    });

    if handle.await.is_err() {
      return Err(FFMPEGError::CouldNotReadOutput);
    }

    let result = child.lock().unwrap().wait();
    let status = result.map_err(|_| FFMPEGError::CouldNotExecute)?;

    if !status.success()
      && status.code() != Some(255)
      && status.signal() != Some(9)
    {
      if self.verbose {
        println!("Process failed with exit code: {:?}", status.code());
      }
      return Err(FFMPEGError::CouldNotExecute);
    }

    if self.verbose {
      println!("Recording saved to {}", output_file);
    }

    return Ok(output_file);
  }

  pub async fn convert_audio_for_whisper(
    &self,
    input_file: &str,
  ) -> FFMPEGResult<String> {
    operations::validate_file_exists(input_file)
      .await
      .map_err(|_| FFMPEGError::AudioConversionFailed)?;

    let input_path = Path::new(input_file);
    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = input_path
      .file_stem()
      .and_then(|s| s.to_str())
      .unwrap_or("audio");
    let output_file = parent_dir.join(format!("{}_whisper.wav", stem));
    let output_file_str = output_file.to_string_lossy();

    if self.verbose {
      println!(
        "Converting audio to Whisper format: {} → {}",
        input_file, output_file_str
      );
    }

    let output = tokio::process::Command::new("ffmpeg")
      .args([
        "-i",
        input_file,
        "-ar",
        "16000",
        "-ac",
        "1",
        "-c:a",
        "pcm_s16le",
        &output_file_str,
        "-y",
      ])
      .output()
      .await
      .map_err(|_| FFMPEGError::AudioConversionFailed)?;

    if !output.status.success() {
      if self.verbose {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("FFmpeg conversion error: {}", stderr);
      }
      return Err(FFMPEGError::AudioConversionFailed);
    }

    if self.verbose {
      println!("Audio conversion completed: {}", output_file_str);
    }

    return Ok(output_file_str.to_string());
  }
}
