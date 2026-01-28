use crate::files::operations;
use crate::files::temporary::TemporaryFile;
use tokio::fs;

#[tokio::test]
async fn test_temporary_file_auto_cleanup() {
  let file_path = "test_temp_file.txt";

  fs::write(file_path, "test content").await.unwrap();
  assert!(operations::file_exists(file_path).await);

  {
    let _temp_file = TemporaryFile::new(file_path.to_string());
    assert!(operations::file_exists(file_path).await);
  }

  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
  assert!(!operations::file_exists(file_path).await);
}

#[tokio::test]
async fn test_temporary_file_keep() {
  let file_path = "test_temp_file_keep.txt";

  fs::write(file_path, "test content").await.unwrap();
  assert!(operations::file_exists(file_path).await);

  {
    let mut temp_file = TemporaryFile::new(file_path.to_string());
    temp_file.keep();
  }

  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
  assert!(operations::file_exists(file_path).await);

  fs::remove_file(file_path).await.unwrap();
}

#[tokio::test]
async fn test_temporary_file_manual_cleanup() {
  let file_path = "test_temp_file_manual.txt";

  fs::write(file_path, "test content").await.unwrap();
  assert!(operations::file_exists(file_path).await);

  let temp_file = TemporaryFile::new(file_path.to_string());
  assert!(operations::file_exists(file_path).await);

  temp_file.cleanup().await.unwrap();
  assert!(!operations::file_exists(file_path).await);
}

#[tokio::test]
async fn test_temporary_file_path_access() {
  let file_path = "test_temp_file_path.txt";

  let temp_file = TemporaryFile::new(file_path.to_string());
  assert_eq!(temp_file.path(), file_path);
}
