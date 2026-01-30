use thiserror::Error;

/// Network-related errors.
///
/// Represents errors that can occur during HTTP requests and network communication.
#[derive(Error, Debug)]
pub enum NetworkError {
  #[error("Invalid service URL: '{0}'. Please check your configuration file.")]
  InvalidURL(String),

  #[error(
    "Failed to connect to service. Please verify the service is running and accessible."
  )]
  RequestFailed,

  #[error(
    "Service returned an error. Please check the service logs and try again."
  )]
  ResponseError,

  #[error(
    "Failed to decode service response. The service may be experiencing issues or the format may be unsupported."
  )]
  DecodeError,
}

/// Result type for network operations.
pub type NetworkResult<T> = Result<T, NetworkError>;
