//! HTTP client module for network requests to external services.
//!
//! This module provides a simple HTTP client for communicating with remote
//! services, primarily the Whisper transcription API. It supports multipart
//! form uploads and JSON response parsing.
//!
//! ## Main Components
//!
//! - [`HttpClient`]: HTTP client for making requests to external services
//! - [`NetworkError`]: Error types for network operations
//! - [`NetworkResult<T>`]: Result type alias for network operations
//!
//! ## Features
//!
//! - POST requests with multipart form data
//! - JSON response deserialization
//! - URL validation before requests
//! - Verbose logging support for debugging

pub mod errors;

#[cfg(test)]
mod network_tests;

use reqwest::multipart;

use crate::network::errors::{NetworkError, NetworkResult};

/// HTTP client for network requests to external services.
///
/// Provides generic POST functionality with multipart form support
/// and verbose logging capabilities for debugging.
#[derive(Debug, Clone)]
pub struct HttpClient {
  base_url: String,
  verbose: bool,
}

impl HttpClient {
  /// Creates a new HttpClient with base URL and verbose settings.
  ///
  /// # Arguments
  ///
  /// * `base_url` - Base URL for all HTTP requests
  /// * `verbose` - Whether to show detailed request/response information
  ///
  /// # Returns
  ///
  /// A new `HttpClient` instance.
  pub fn new(base_url: String, verbose: bool) -> Self {
    return HttpClient { base_url, verbose };
  }

  /// Sends a POST request with multipart form data to the given endpoint.
  ///
  /// Validates the service URL, sends the request with form data, and deserializes
  /// the JSON response into the specified type.
  ///
  /// # Type Parameters
  ///
  /// * `T` - Type to deserialize the JSON response into
  ///
  /// # Arguments
  ///
  /// * `form` - Multipart form data to send in the request
  /// * `endpoint` - Endpoint path to append to the base URL
  ///
  /// # Returns
  ///
  /// A `NetworkResult<T>` containing the deserialized response or an error.
  pub async fn post_with_form<T>(
    &self,
    form: multipart::Form,
    endpoint: &str,
  ) -> NetworkResult<T>
  where
    T: serde::de::DeserializeOwned,
  {
    self.check_url().await?;

    let client = reqwest::Client::new();
    let full_url = format!("{}/{}", self.base_url, endpoint);

    if self.verbose {
      println!("Sending POST request to: {}", full_url);
    }

    let response = client
      .post(&full_url)
      .multipart(form)
      .send()
      .await
      .map_err(|_| NetworkError::RequestFailed)?;

    if self.verbose {
      println!(
        "Received response from service. Status: {}",
        response.status()
      );
    }

    if response.status() != reqwest::StatusCode::OK {
      return Err(NetworkError::ResponseError);
    }

    let parsed_response = response
      .json::<T>()
      .await
      .map_err(|_| NetworkError::DecodeError)?;

    return Ok(parsed_response);
  }

  async fn check_url(&self) -> NetworkResult<()> {
    if self.verbose {
      println!("Checking if service URL is reachable...");
    }

    let _url = reqwest::Url::parse(&self.base_url).map_err(|e| {
      if self.verbose {
        println!("Invalid URL format: {}", e);
      }
      NetworkError::InvalidURL
    })?;

    let client = reqwest::Client::new();

    let response = client.get(&self.base_url).send().await.map_err(|e| {
      if self.verbose {
        println!("Failed to connect to URL: {}", e);
      }
      NetworkError::RequestFailed
    })?;

    let status = response.status();
    if status != reqwest::StatusCode::OK
      && status != reqwest::StatusCode::NOT_FOUND
    {
      if self.verbose {
        println!("URL returned unexpected status: {}", status);
      }
      return Err(NetworkError::InvalidURL);
    }

    if self.verbose {
      println!("Service URL is reachable with status: {}", status);
    }

    return Ok(());
  }
}
