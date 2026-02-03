use crate::files::errors::FileResult;
use crate::files::operations;

/// RAII-style temporary file management.
///
/// Automatically cleans up files when the struct goes out of scope,
/// ensuring no temporary files are left behind even if operations fail.
#[derive(Debug, Clone)]
pub struct TemporaryFile {
  path: String,
  should_cleanup: bool,
}

impl TemporaryFile {
  /// Creates a new temporary file tracking instance.
  ///
  /// # Arguments
  ///
  /// * `path` - The path to temporary file
  ///
  /// # Returns
  ///
  /// A `TemporaryFile` instance that will clean up the file on drop.
  pub fn new(path: String) -> Self {
    return TemporaryFile {
      path,
      should_cleanup: true,
    };
  }

  /// Gets the path to the temporary file.
  ///
  /// # Returns
  ///
  /// A reference to the file path string.
  pub fn path(&self) -> &str {
    return &self.path;
  }

  /// Prevents automatic cleanup of the file.
  ///
  /// Call this if you want to keep the file after the TemporaryFile goes out of scope.
  pub fn keep(&mut self) {
    self.should_cleanup = false;
  }

  /// Manually cleans up the temporary file.
  ///
  /// Can be called before drop to perform explicit cleanup.
  /// On success, prevents the automatic cleanup from running again.
  pub async fn cleanup(&mut self) -> FileResult<()> {
    operations::remove_file(&self.path).await?;
    self.should_cleanup = false;
    return Ok(());
  }
}

impl Drop for TemporaryFile {
  fn drop(&mut self) {
    if self.should_cleanup {
      let path = self.path.clone();
      tokio::spawn(async move {
        if let Err(e) = operations::remove_file(&path).await {
          eprintln!("Failed to cleanup temporary file '{}': {}", path, e);
        }
      });
    }
  }
}
