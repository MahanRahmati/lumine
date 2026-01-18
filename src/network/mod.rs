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

    let response = match client.post(&full_url).multipart(form).send().await {
      Ok(response) => response,
      Err(_) => return Err(NetworkError::RequestFailed),
    };

    if self.verbose {
      println!(
        "Received response from service. Status: {}",
        response.status()
      );
    }

    if response.status() != reqwest::StatusCode::OK {
      return Err(NetworkError::ResponseError);
    }

    let response_text = match response.text().await {
      Ok(text) => text,
      Err(_) => return Err(NetworkError::DecodeError),
    };

    let parsed_response: T = match serde_json::from_str(&response_text) {
      Ok(response) => response,
      Err(_) => return Err(NetworkError::DecodeError),
    };

    return Ok(parsed_response);
  }

  async fn check_url(&self) -> NetworkResult<()> {
    if self.verbose {
      println!("Checking if service URL is reachable...");
    }

    let _url = match reqwest::Url::parse(&self.base_url) {
      Ok(url) => url,
      Err(e) => {
        if self.verbose {
          println!("Invalid URL format: {}", e);
        }
        return Err(NetworkError::InvalidURL);
      }
    };

    let client = reqwest::Client::new();

    let response = match client.get(&self.base_url).send().await {
      Ok(response) => response,
      Err(e) => {
        if self.verbose {
          println!("Failed to connect to URL: {}", e);
        }
        return Err(NetworkError::RequestFailed);
      }
    };

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
