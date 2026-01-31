use std::process::Stdio;

use tokio::process::Command;

use crate::process::errors::{ProcessError, ProcessResult};

/// Command output wrapper.
///
/// Contains the standard output, standard error, and exit status of a completed command.
#[derive(Debug, Clone)]
pub struct CommandOutput {
  pub stdout: String,
  pub stderr: String,
  pub status: std::process::ExitStatus,
}

impl CommandOutput {
  /// Creates a new CommandOutput instance.
  ///
  /// # Arguments
  ///
  /// * `stdout` - Standard output from the command
  /// * `stderr` - Standard error from the command
  /// * `status` - Exit status of the command
  ///
  /// # Returns
  ///
  /// A new `CommandOutput` instance.
  pub fn new(
    stdout: String,
    stderr: String,
    status: std::process::ExitStatus,
  ) -> Self {
    return CommandOutput {
      stdout,
      stderr,
      status,
    };
  }
}

/// Centralized process executor.
///
/// Provides utilities for running commands and managing processes.
pub struct ProcessExecutor;

impl ProcessExecutor {
  /// Run a command and return output.
  ///
  /// Executes a command with the given arguments and captures both
  /// standard output and standard error output.
  ///
  /// # Arguments
  ///
  /// * `command` - The command to execute
  /// * `args` - Arguments to pass to the command
  ///
  /// # Returns
  ///
  /// A `ProcessResult<CommandOutput>` containing the command output
  /// or an error if execution failed.
  pub async fn run(
    command: &str,
    args: &[&str],
  ) -> ProcessResult<CommandOutput> {
    let output = Command::new(command)
      .args(args)
      .output()
      .await
      .map_err(|_| ProcessError::ExecutionFailed(command.to_string()))?;

    let command_output = CommandOutput::new(
      String::from_utf8_lossy(&output.stdout).to_string(),
      String::from_utf8_lossy(&output.stderr).to_string(),
      output.status,
    );

    return Ok(command_output);
  }

  /// Spawn a process with standard error piped.
  ///
  /// Spawns a command with piped standard error for async streaming.
  /// Returns the child process handle for further interaction.
  ///
  /// # Arguments
  ///
  /// * `command` - The command to execute
  /// * `args` - Arguments to pass to the command
  ///
  /// # Returns
  ///
  /// A `ProcessResult<tokio::process::Child>` containing the spawned
  /// child process or an error if spawning failed.
  pub async fn spawn_with_stderr_piped(
    command: &str,
    args: &[&str],
  ) -> ProcessResult<tokio::process::Child> {
    let child = Command::new(command)
      .args(args)
      .stderr(Stdio::piped())
      .spawn()
      .map_err(|_| ProcessError::ExecutionFailed(command.to_string()))?;

    return Ok(child);
  }
}
