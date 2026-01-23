use std::path::Path;

use crate::audio::errors::{AudioError, AudioResult};
use crate::files::operations;

pub struct AudioConverter;

impl AudioConverter {
  pub async fn convert_audio_for_whisper(
    input_file: &str,
    verbose: bool,
  ) -> AudioResult<String> {
    operations::validate_file_exists(input_file)
      .await
      .map_err(|_| AudioError::FileNotFound)?;

    let input_path = Path::new(input_file);
    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let stem = input_path
      .file_stem()
      .and_then(|s| s.to_str())
      .unwrap_or("audio");
    let output_file = parent_dir.join(format!("{}_whisper.wav", stem));
    let output_file_str = output_file.to_string_lossy();

    if verbose {
      println!(
        "Converting audio to Whisper format: {} → {}",
        input_file, output_file_str
      );
    }

    convert_with_ffmpeg(input_file, &output_file_str, verbose).await?;

    if verbose {
      println!("Audio conversion completed: {}", output_file_str);
    }

    return Ok(output_file_str.to_string());
  }
}

async fn convert_with_ffmpeg(
  input_file: &str,
  output_file: &str,
  verbose: bool,
) -> AudioResult<()> {
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
      output_file,
      "-y",
    ])
    .output()
    .await
    .map_err(|_| AudioError::ConversionFailed)?;

  if !output.status.success() {
    if verbose {
      let stderr = String::from_utf8_lossy(&output.stderr);
      println!("FFmpeg conversion error: {}", stderr);
    }
    return Err(AudioError::ConversionFailed);
  }

  return Ok(());
}
