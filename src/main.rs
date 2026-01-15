mod cli;
mod files;

use crate::cli::Cli;

use clap::Parser;

fn main() {
  let _cli = Cli::parse();
  println!("Hello, world!");
}
