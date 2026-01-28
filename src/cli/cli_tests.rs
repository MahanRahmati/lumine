use clap::Parser;

use crate::cli::{Cli, Commands};

#[test]
fn test_cli_default_no_arguments() {
  let args = vec!["lumine"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_ok());
  let parsed = cli.unwrap();
  assert!(parsed.command.is_none());
}

#[test]
fn test_cli_transcribe_command_with_file() {
  let args = vec!["lumine", "transcribe", "--file", "test_audio.wav"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_ok());
  let parsed = cli.unwrap();
  match parsed.command {
    Some(Commands::Transcribe { file }) => {
      assert_eq!(file, "test_audio.wav");
    }
    _ => panic!("Expected Transcribe command"),
  }
}

#[test]
fn test_cli_transcribe_command_with_short_file_flag() {
  let args = vec!["lumine", "transcribe", "-f", "test_audio.mp3"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_ok());
  let parsed = cli.unwrap();
  match parsed.command {
    Some(Commands::Transcribe { file }) => {
      assert_eq!(file, "test_audio.mp3");
    }
    _ => panic!("Expected Transcribe command"),
  }
}

#[test]
fn test_cli_record_command() {
  let args = vec!["lumine", "record"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_ok());
  let parsed = cli.unwrap();
  match parsed.command {
    Some(Commands::Record) => {}
    _ => panic!("Expected Record command"),
  }
}

#[test]
fn test_cli_invalid_command() {
  let args = vec!["lumine", "invalid_command"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_err());
}

#[test]
fn test_cli_transcribe_missing_file_argument() {
  let args = vec!["lumine", "transcribe"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_err());
}

#[test]
fn test_cli_help() {
  let args = vec!["lumine", "--help"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_err());
}

#[test]
fn test_cli_version() {
  let args = vec!["lumine", "--version"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_err());
}

#[test]
fn test_cli_transcribe_with_file_containing_spaces() {
  let args = vec!["lumine", "transcribe", "--file", "audio with spaces.wav"];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_ok());
  let parsed = cli.unwrap();
  match parsed.command {
    Some(Commands::Transcribe { file }) => {
      assert_eq!(file, "audio with spaces.wav");
    }
    _ => panic!("Expected Transcribe command"),
  }
}

#[test]
fn test_cli_transcribe_with_empty_file_string() {
  let args = vec!["lumine", "transcribe", "--file", ""];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_ok());
  let parsed = cli.unwrap();
  match parsed.command {
    Some(Commands::Transcribe { file }) => {
      assert_eq!(file, "");
    }
    _ => panic!("Expected Transcribe command"),
  }
}

#[test]
fn test_cli_multiple_arguments_ignored_extra() {
  let args = vec![
    "lumine",
    "transcribe",
    "--file",
    "test.wav",
    "extra",
    "args",
  ];
  let cli = Cli::try_parse_from(args);

  assert!(cli.is_err());
}
