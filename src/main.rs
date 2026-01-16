mod cli;
mod config;
mod ffmpeg;
mod files;
mod network;
mod whisper;

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::ffmpeg::FFMPEG;
use crate::files::operations::{remove_file, validate_file_exists};
use crate::whisper::Whisper;

fn main() {
  let cli = Cli::parse();

  let config = match Config::load() {
    Ok(config) => config,
    Err(e) => {
      eprintln!("Configuration Error: {}", e);
      std::process::exit(1);
    }
  };

  match cli.command {
    Some(Commands::Transcribe { file }) => {
      transcribe_file(&config, &file);
    }
    None => {
      record_and_transcribe(&config);
    }
  }
}

fn transcribe_file(config: &Config, file_path: &str) {
  if let Err(e) = validate_file_exists(file_path) {
    eprintln!("File Error: {}", e);
    std::process::exit(1);
  }

  let whisper = Whisper::new(
    config.get_whisper_url(),
    file_path.to_string(),
    config.get_verbose(),
  );

  let transcript = match whisper.send_audio() {
    Ok(transcript) => transcript,
    Err(e) => {
      println!("Transcription Error: {}", e);
      std::process::exit(1);
    }
  };

  println!("{}", transcript);
}

fn record_and_transcribe(config: &Config) {
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
    let result = remove_file(&file_path.clone());
    if result.is_ok() && config.get_verbose() {
      println!("File removed: {}", file_path);
    }
  }

  println!("{}", transcript);
}
