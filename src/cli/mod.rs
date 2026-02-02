//! Command-line interface and argument parsing module.
//!
//! This module defines the CLI structure using `clap` for parsing command-line
//! arguments and subcommands. It provides type-safe argument handling and
//! automatic help generation.
//!
//! ## Commands
//!
//! - **Default (no subcommand)**: Record audio and transcribe
//! - `transcribe --file <path>`: Transcribe an existing audio file
//! - `record`: Record audio and save to file only
//! - `reset-config`: Reset configuration to default values

#[cfg(test)]
mod cli_tests;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lumine")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = concat!("Lumine v", env!("CARGO_PKG_VERSION")))]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,

  /// Use verbose output
  #[arg(short, long, default_value_t = false)]
  pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
  /// Transcribe an existing audio file
  Transcribe {
    /// Path to the audio file to transcribe
    #[arg(short, long)]
    file: String,
  },

  /// Record audio and save it to a file
  Record,

  /// Reset configuration to default values
  ResetConfig,
}
