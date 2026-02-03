mod app;
mod audio;
mod cli;
mod config;
mod files;
mod logging;
mod network;
mod output;
mod process;
mod whisper;

use clap::Parser;

use crate::app::App;
use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::logging::set_verbose;
use crate::output::format::OutputFormat;

#[tokio::main]
async fn main() {
  let cli = Cli::parse();

  set_verbose(cli.verbose);

  let config = match Config::load().await {
    Ok(config) => config,
    Err(e) => {
      eprintln!("Configuration Error: {}", e);
      std::process::exit(1);
    }
  };

  let app = App::new(config);
  let format = OutputFormat::from_flags(cli.output_json, cli.output_json_full);

  let result = match cli.command {
    Some(Commands::Transcribe {
      file,
      output_json,
      output_json_full,
    }) => {
      let format = OutputFormat::from_flags(output_json, output_json_full);
      app.transcribe_file(&file, format).await
    }
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
    None => app.record_and_transcribe(format).await,
  };

  match result {
    Ok(output) => println!("{}", output),
    Err(e) => {
      eprintln!("{}", e);
      std::process::exit(1);
    }
  }
}
