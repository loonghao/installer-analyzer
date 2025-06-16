//! File download and verification functionality for updates

use crate::analyzers::common::{calculate_file_hash, get_file_size};
use crate::core::Result;
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Download manager for handling file downloads and verification
pub struct DownloadManager {
    client: Client,
    temp_dir: PathBuf,
}

impl DownloadManager {
    /// Create a new download manager
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("installer-analyzer-updater/1.0")
            .timeout(Duration::from_secs(300)) // 5 minutes for large files
            .build()
            .expect("Failed to create HTTP client");

        let temp_dir = std::env::temp_dir().join("installer-analyzer-updates");

        Self { client, temp_dir }
    }

    /// Create a new download manager with custom temp directory
    pub fn with_temp_dir(temp_dir: PathBuf) -> Self {
        let client = Client::builder()
            .user_agent("installer-analyzer-updater/1.0")
            .timeout(Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, temp_dir }
    }

    /// Download a file from the given URL
    pub async fn download_file(&self, url: &str) -> Result<PathBuf> {
        tracing::info!("Starting download from: {}", url);

        // Ensure temp directory exists
        tokio::fs::create_dir_all(&self.temp_dir)
            .await
            .map_err(|e| {
                crate::core::AnalyzerError::generic(format!(
                    "Failed to create temp directory: {}",
                    e
                ))
            })?;

        // Extract filename from URL
        let filename = self.extract_filename_from_url(url)?;
        let file_path = self.temp_dir.join(&filename);

        // Start the download
        let response = self.client.get(url).send().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to start download: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(crate::core::AnalyzerError::generic(format!(
                "Download failed with status: {}",
                response.status()
            )));
        }

        // Get content length for progress tracking
        let total_size = response.content_length().unwrap_or(0);
        tracing::info!("Download size: {} bytes", total_size);

        // Create the file
        let mut file = File::create(&file_path).await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to create file: {}", e))
        })?;

        // Read the entire response body
        let bytes = response.bytes().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to read response: {}", e))
        })?;

        // Write to file
        file.write_all(&bytes).await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to write file: {}", e))
        })?;

        let downloaded = bytes.len() as u64;

        // Log progress
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64) * 100.0;
            tracing::info!("Download progress: {:.1}%", progress);
        }

        file.flush().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to flush file: {}", e))
        })?;

        tracing::info!("Download completed: {}", file_path.display());
        Ok(file_path)
    }

    /// Download a file with progress callback
    pub async fn download_file_with_progress<F>(
        &self,
        url: &str,
        mut progress_callback: F,
    ) -> Result<PathBuf>
    where
        F: FnMut(u64, u64) + Send,
    {
        tracing::info!("Starting download with progress tracking from: {}", url);

        // Ensure temp directory exists
        tokio::fs::create_dir_all(&self.temp_dir)
            .await
            .map_err(|e| {
                crate::core::AnalyzerError::generic(format!(
                    "Failed to create temp directory: {}",
                    e
                ))
            })?;

        // Extract filename from URL
        let filename = self.extract_filename_from_url(url)?;
        let file_path = self.temp_dir.join(&filename);

        // Start the download
        let response = self.client.get(url).send().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to start download: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(crate::core::AnalyzerError::generic(format!(
                "Download failed with status: {}",
                response.status()
            )));
        }

        // Get content length for progress tracking
        let total_size = response.content_length().unwrap_or(0);

        // Create the file
        let mut file = File::create(&file_path).await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to create file: {}", e))
        })?;

        // Download with progress tracking
        // For now, use simple approach - in future can implement streaming with progress
        let bytes = response.bytes().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to read response: {}", e))
        })?;

        // Write to file
        file.write_all(&bytes).await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to write file: {}", e))
        })?;

        let downloaded = bytes.len() as u64;
        progress_callback(downloaded, total_size);

        file.flush().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to flush file: {}", e))
        })?;

        tracing::info!("Download completed: {}", file_path.display());
        Ok(file_path)
    }

    /// Verify the SHA256 hash of a downloaded file
    pub async fn verify_file_hash(&self, file_path: &Path, expected_hash: &str) -> Result<()> {
        tracing::info!("Verifying file hash: {}", file_path.display());

        let actual_hash = calculate_file_hash(file_path).await?;

        if actual_hash.to_lowercase() != expected_hash.to_lowercase() {
            return Err(crate::core::AnalyzerError::generic(format!(
                "File hash verification failed. Expected: {}, Actual: {}",
                expected_hash, actual_hash
            )));
        }

        tracing::info!("File hash verification successful");
        Ok(())
    }

    /// Get the size of a downloaded file
    pub async fn get_downloaded_file_size(&self, file_path: &Path) -> Result<u64> {
        get_file_size(file_path).await
    }

    /// Clean up temporary files
    pub async fn cleanup(&self) -> Result<()> {
        if self.temp_dir.exists() {
            tokio::fs::remove_dir_all(&self.temp_dir)
                .await
                .map_err(|e| {
                    crate::core::AnalyzerError::generic(format!(
                        "Failed to cleanup temp directory: {}",
                        e
                    ))
                })?;
        }
        Ok(())
    }

    /// Extract filename from URL
    fn extract_filename_from_url(&self, url: &str) -> Result<String> {
        let url_path = url
            .split('/')
            .next_back()
            .ok_or_else(|| crate::core::AnalyzerError::generic("Invalid download URL"))?;

        // Remove query parameters if present
        let filename = url_path.split('?').next().unwrap_or(url_path);

        if filename.is_empty() {
            return Err(crate::core::AnalyzerError::generic(
                "Could not extract filename from URL",
            ));
        }

        Ok(filename.to_string())
    }

    /// Get the temp directory path
    pub fn get_temp_dir(&self) -> &Path {
        &self.temp_dir
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Download progress information
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// Bytes downloaded so far
    pub downloaded: u64,
    /// Total bytes to download (0 if unknown)
    pub total: u64,
    /// Download speed in bytes per second
    pub speed: f64,
    /// Estimated time remaining in seconds
    pub eta: Option<u64>,
}

impl DownloadProgress {
    /// Calculate download percentage (0-100)
    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.downloaded as f64 / self.total as f64) * 100.0
        }
    }

    /// Check if download is complete
    pub fn is_complete(&self) -> bool {
        self.total > 0 && self.downloaded >= self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_download_manager_creation() {
        let manager = DownloadManager::new();
        assert!(manager
            .get_temp_dir()
            .to_string_lossy()
            .contains("installer-analyzer-updates"));
    }

    #[test]
    fn test_download_manager_with_custom_temp_dir() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DownloadManager::with_temp_dir(temp_dir.path().to_path_buf());
        assert_eq!(manager.get_temp_dir(), temp_dir.path());
    }

    #[test]
    fn test_extract_filename_from_url() {
        let manager = DownloadManager::new();

        // Test normal URL
        let filename = manager
            .extract_filename_from_url("https://example.com/file.exe")
            .unwrap();
        assert_eq!(filename, "file.exe");

        // Test URL with query parameters
        let filename = manager
            .extract_filename_from_url("https://example.com/file.exe?version=1.0")
            .unwrap();
        assert_eq!(filename, "file.exe");

        // Test URL with path
        let filename = manager
            .extract_filename_from_url("https://example.com/path/to/file.exe")
            .unwrap();
        assert_eq!(filename, "file.exe");

        // Test invalid URL
        assert!(manager
            .extract_filename_from_url("https://example.com/")
            .is_err());
        assert!(manager.extract_filename_from_url("").is_err());
    }

    #[test]
    fn test_download_progress() {
        let progress = DownloadProgress {
            downloaded: 50,
            total: 100,
            speed: 1024.0,
            eta: Some(50),
        };

        assert_eq!(progress.percentage(), 50.0);
        assert!(!progress.is_complete());

        let complete_progress = DownloadProgress {
            downloaded: 100,
            total: 100,
            speed: 1024.0,
            eta: Some(0),
        };

        assert_eq!(complete_progress.percentage(), 100.0);
        assert!(complete_progress.is_complete());

        // Test unknown total size
        let unknown_progress = DownloadProgress {
            downloaded: 50,
            total: 0,
            speed: 1024.0,
            eta: None,
        };

        assert_eq!(unknown_progress.percentage(), 0.0);
        assert!(!unknown_progress.is_complete());
    }

    #[tokio::test]
    async fn test_cleanup() {
        let temp_dir = TempDir::new().unwrap();
        let manager = DownloadManager::with_temp_dir(temp_dir.path().to_path_buf());

        // Create the temp directory
        tokio::fs::create_dir_all(manager.get_temp_dir())
            .await
            .unwrap();
        assert!(manager.get_temp_dir().exists());

        // Cleanup should succeed
        manager.cleanup().await.unwrap();
    }
}
