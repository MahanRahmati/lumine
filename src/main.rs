mod cli;
mod config;
mod ffmpeg;
mod files;

use crate::cli::Cli;
use crate::config::Config;
use crate::ffmpeg::FFMPEG;

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

  println!("Hello, world!");
}
