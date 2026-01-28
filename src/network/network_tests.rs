use reqwest::multipart;
use serde::{Deserialize, Serialize};

use crate::network::{HttpClient, NetworkError};

#[derive(Debug, Serialize, Deserialize)]
struct TestResponse {
  message: String,
  status: String,
}

#[tokio::test]
async fn test_check_url_invalid_format() {
  let client = HttpClient::new("not-a-valid-url".to_string(), false);
  let result = client.check_url().await;

  assert!(result.is_err());
  match result.unwrap_err() {
    NetworkError::InvalidURL => {}
    _ => panic!("Expected InvalidURL error"),
  }
}

#[tokio::test]
async fn test_check_url_invalid_format_verbose() {
  let client = HttpClient::new("invalid-url".to_string(), true);
  let result = client.check_url().await;

  assert!(result.is_err());
  match result.unwrap_err() {
    NetworkError::InvalidURL => {}
    _ => panic!("Expected InvalidURL error"),
  }
}

#[tokio::test]
async fn test_check_url_unreachable_service() {
  let client = HttpClient::new("http://localhost:99999".to_string(), false);
  let result = client.check_url().await;

  assert!(result.is_err());
  match result.unwrap_err() {
    NetworkError::RequestFailed => {}
    NetworkError::InvalidURL => {}
    _ => panic!("Expected RequestFailed or InvalidURL error"),
  }
}

#[tokio::test]
async fn test_post_with_form_invalid_endpoint() {
  let client = HttpClient::new("invalid-url".to_string(), false);
  let form = multipart::Form::new();

  let result: Result<TestResponse, _> =
    client.post_with_form(form, "test").await;
  assert!(result.is_err());
  match result.unwrap_err() {
    NetworkError::InvalidURL => {}
    _ => panic!("Expected InvalidURL error"),
  }
}

#[tokio::test]
async fn test_post_with_form_unreachable_service() {
  let client = HttpClient::new("http://localhost:99999".to_string(), false);
  let form = multipart::Form::new();

  let result: Result<TestResponse, _> =
    client.post_with_form(form, "test").await;
  assert!(result.is_err());
  match result.unwrap_err() {
    NetworkError::RequestFailed => {}
    NetworkError::InvalidURL => {}
    _ => panic!("Expected RequestFailed or InvalidURL error"),
  }
}

#[tokio::test]
async fn test_url_parsing_edge_cases() {
  let invalid_urls = vec![
    "",
    "not-a-url",
    "http://",
    "https://",
    "ftp://example.com",
    "javascript:void(0)",
  ];

  for url in invalid_urls {
    let client = HttpClient::new(url.to_string(), false);
    let result = client.check_url().await;
    assert!(result.is_err(), "URL '{}' should fail", url);
  }
}

#[tokio::test]
async fn test_post_with_form_with_zero_length_file() {
  let client = HttpClient::new("http://localhost:99999".to_string(), false);
  let form = multipart::Form::new().part(
    "file",
    multipart::Part::bytes(vec![]).file_name("empty.txt"),
  );

  let result: Result<TestResponse, _> =
    client.post_with_form(form, "test").await;
  assert!(result.is_err());
}
