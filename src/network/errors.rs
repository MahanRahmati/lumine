#[derive(Debug, Clone)]
pub enum NetworkError {
  InvalidURL,
  RequestFailed,
  ResponseError,
  DecodeError,
}

impl std::fmt::Display for NetworkError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      NetworkError::InvalidURL => {
        write!(
          f,
          "Invalid service URL. Please check your configuration file."
        )
      }
      NetworkError::RequestFailed => {
        write!(
          f,
          "Failed to connect to service. Please verify the service is running and accessible."
        )
      }
      NetworkError::ResponseError => {
        write!(
          f,
          "Service returned an error. Please check the service logs and try again."
        )
      }
      NetworkError::DecodeError => {
        write!(
          f,
          "Failed to decode service response. The service may be experiencing issues or the format may be unsupported."
        )
      }
    }
  }
}

impl std::error::Error for NetworkError {}

pub type NetworkResult<T> = Result<T, NetworkError>;
