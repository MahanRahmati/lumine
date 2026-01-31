//! Process execution utilities.
//!
//! Provides centralized process management and command execution
//! utilities used across the application.
//!
//! ## Main Components
//!
//! - [`ProcessExecutor`]: Centralized process executor for running commands
//! - [`CommandOutput`]: Wrapper for command output with stdout, stderr, and status
//!
//! ## Features
//!
//! - Run commands and capture output
//! - Spawn processes with piped stderr for async streaming
//! - Check command availability

pub mod errors;
pub mod executor;
