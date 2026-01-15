use clap::Parser;

#[derive(Parser)]
#[command(name = "lumine")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {}
