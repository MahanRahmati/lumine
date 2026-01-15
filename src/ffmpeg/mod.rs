mod devices;
mod errors;

use super::files::operations;
use std::io::BufRead;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use regex::Regex;

use crate::ffmpeg::devices::{AudioInputDevice, AudioInputDevices};
use crate::ffmpeg::errors::{FFMPEGError, FFMPEGResult};

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

  pub fn record_audio(&self) -> FFMPEGResult<String> {
    self.check_ffmpeg()?;
    let devices = self.get_audio_input_devices()?;
    let device = self.select_audio_input_device(devices);
    return self.record_audio_with_device(device);
  }

  fn check_ffmpeg(&self) -> FFMPEGResult<bool> {
    let output = Command::new("ffmpeg").args(["-version"]).output();
    if let Ok(output) = output {
      let output_str = String::from_utf8_lossy(&output.stdout);
      for line in output_str.lines() {
        if line.contains("ffmpeg version") {
          if self.verbose {
            println!("Found ffmpeg: {}", line);
          }
          return Ok(true);
        }
      }
    }
    return Err(FFMPEGError::NotFound);
  }

  fn get_audio_input_devices(&self) -> FFMPEGResult<AudioInputDevices> {
    let output = Command::new("ffmpeg")
      .args(["-f", "avfoundation", "-list_devices", "true", "-i", ""])
      .output();

    if let Ok(output) = output {
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

    return Err(FFMPEGError::CouldNotExecute);
  }

  fn select_audio_input_device(
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

  fn record_audio_with_device(
    &self,
    device: AudioInputDevice,
  ) -> FFMPEGResult<String> {
    if operations::create_directory_all(&self.recordings_directory).is_err() {
      return Err(FFMPEGError::CouldNotCreateDirectory);
    }

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
      .spawn();

    if let Ok(child) = output {
      if self.verbose {
        println!("Recording audio to: {}", output_file);
      }

      if self.verbose {
        println!(
          "Recording... will stop after {}s of silence",
          self.silence_limit
        );
      }

      let child = Arc::new(Mutex::new(child));
      let child_clone = Arc::clone(&child);
      let stderr = child.lock().unwrap().stderr.take();
      if let Some(stderr) = stderr {
        let mut reader = std::io::BufReader::new(stderr);

        let should_kill = Arc::new(Mutex::new(true));
        let should_kill_clone = Arc::clone(&should_kill);

        let verbose = self.verbose;
        let silence_limit = self.silence_limit;

        let handle = thread::spawn(move || {
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
              _timer = Some(thread::spawn(move || {
                thread::sleep(Duration::from_secs(silence_limit as u64));

                // Check if we should still kill the process
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

        if handle.join().is_err() {
          return Err(FFMPEGError::CouldNotReadOutput);
        }

        let result = child.lock().unwrap().wait();
        if let Ok(status) = result {
          if !status.success()
            && status.code() != Some(255)
            && status.signal() != Some(9)
          {
            if self.verbose {
              println!("Process failed with exit code: {:?}", status.code());
            }
            return Err(FFMPEGError::CouldNotExecute);
          }
        } else {
          return Err(FFMPEGError::CouldNotExecute);
        }

        if self.verbose {
          println!("Recording saved to {}", output_file);
        }

        return Ok(output_file);
      }

      return Err(FFMPEGError::CouldNotReadOutput);
    }

    return Err(FFMPEGError::CouldNotExecute);
  }
}
