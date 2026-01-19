pub mod errors;

use reqwest::multipart;

use crate::network::errors::{NetworkError, NetworkResult};

#[derive(Debug, Clone)]
pub struct HttpClient {
  base_url: String,
  verbose: bool,
}

impl HttpClient {
  pub fn new(base_url: String, verbose: bool) -> Self {
    return HttpClient { base_url, verbose };
  }

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

    let response_text = response
      .text()
      .await
      .map_err(|_| NetworkError::DecodeError)?;

    let parsed_response: T = serde_json::from_str(&response_text)
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
