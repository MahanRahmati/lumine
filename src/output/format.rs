/// Output format for transcription results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
  /// Plain text output
  Text,
  /// Simple JSON output with just text
  Json,
  /// Full JSON output with all Whisper metadata
  FullJson,
}

impl OutputFormat {
  /// Creates OutputFormat from CLI boolean flags.
  ///
  /// # Arguments
  ///
  /// * `output_json` - Whether to output simple JSON
  /// * `output_json_full` - Whether to output full JSON
  ///
  /// # Returns
  ///
  /// The appropriate `OutputFormat` variant.
  pub fn from_flags(output_json: bool, output_json_full: bool) -> Self {
    if output_json_full {
      return Self::FullJson;
    }
    if output_json {
      return Self::Json;
    }
    return Self::Text;
  }
}
