use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lumine")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = concat!("Lumine v", env!("CARGO_PKG_VERSION")))]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Commands>,
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
}
