mod app;
mod audio;
mod cli;
mod config;
mod files;
mod network;
mod process;
mod whisper;

use clap::Parser;

use crate::app::App;
use crate::cli::{Cli, Commands};
use crate::config::Config;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  let config = match Config::load().await {
    Ok(config) => config,
    Err(e) => {
      eprintln!("Configuration Error: {}", e);
      std::process::exit(1);
    }
  };

  let app = App::new(config);

  let result = match cli.command {
    Some(Commands::Transcribe { file }) => app.transcribe_file(&file).await,
    Some(Commands::Record) => app.record_only().await,
    Some(Commands::ResetConfig) => match Config::reset_to_defaults().await {
      Ok(_) => {
        println!("Configuration has been reset to default values.");
        return;
      }
      Err(e) => {
        eprintln!("Failed to reset configuration: {}", e);
        std::process::exit(1);
      }
    },
    None => app.record_and_transcribe().await,
  };

  match result {
    Ok(output) => println!("{}", output),
    Err(e) => {
      eprintln!("{}", e);
      std::process::exit(1);
    }
  }
}
