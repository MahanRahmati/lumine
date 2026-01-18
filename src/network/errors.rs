use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
  #[error("Invalid service URL. Please check your configuration file.")]
  InvalidURL,

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

pub type NetworkResult<T> = Result<T, NetworkError>;
