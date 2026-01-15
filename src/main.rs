mod cli;
mod config;
mod files;

use crate::cli::Cli;
use crate::config::Config;

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

  println!("Hello, world!");
}
