//! File operations and temporary file management module.
//!
//! This module provides centralized file system operations and RAII-style
//! temporary file management. All file I/O operations are async and use
//! Tokio for non-blocking execution.
//!
//! ## Submodules
//!
//! - [`operations`]: Core file system operations (read, write, delete, etc.)
//! - [`temporary`]: RAII temporary file management with automatic cleanup
//! - [`errors`]: Error types for file operations
//!
//! ## Features
//!
//! - Async file operations using Tokio
//! - RAII pattern for temporary files (auto-cleanup on drop)
//! - XDG directory compliance helpers
//! - Comprehensive error handling with context

pub mod errors;
pub mod operations;
pub mod temporary;

#[cfg(test)]
mod files_tests;
#[cfg(test)]
mod temporary_tests;
