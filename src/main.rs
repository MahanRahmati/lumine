mod cli;
mod config;
mod ffmpeg;
mod files;
mod whisper;

use crate::cli::Cli;
use crate::config::Config;
use crate::ffmpeg::FFMPEG;
use crate::files::operations::remove_file;
use crate::whisper::Whisper;

use clap::Parser;

fn main() {
  let _cli = Cli::parse();

  let config = match Config::load() {
    Ok(config) => config,
    Err(e) => {
      eprintln!("Configuration Error: {}", e);
      std::process::exit(1);
    }
  };

  let ffmpeg = FFMPEG::new(
    config.get_recordings_directory(),
    config.get_silence_limit(),
    config.get_silence_detect_noise(),
    config.get_preferred_audio_input_device(),
    config.get_verbose(),
  );

  let file_path = match ffmpeg.record_audio() {
    Ok(file_path) => file_path,
    Err(e) => {
      eprintln!("Recording Error: {}", e);
      std::process::exit(1);
    }
  };

  let whisper = Whisper::new(
    config.get_whisper_url(),
    file_path.clone(),
    config.get_verbose(),
  );

  let transcript = match whisper.send_audio() {
    Ok(transcript) => transcript,
    Err(e) => {
      println!("Transcription Error: {}", e);
      std::process::exit(1);
    }
  };

  if config.get_remove_after_transcript() {
    let _ = remove_file(&file_path.clone(), config.get_verbose());
  }

  println!("{}", transcript);
}
