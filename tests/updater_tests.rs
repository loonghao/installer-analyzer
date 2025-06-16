//! Integration tests for auto-update functionality

use installer_analyzer::updater::{
    client::{Release, ReleaseAsset, UpdateClient},
    download::DownloadManager,
    version::{Version, VersionChecker},
    UpdateConfig, UpdateInfo, Updater,
};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Mock HTTP server for testing GitHub API responses
#[allow(dead_code)] // Prepared for future network testing
struct MockGitHubServer {
    responses: Arc<Mutex<HashMap<String, serde_json::Value>>>,
}

#[allow(dead_code)] // Prepared for future network testing
impl MockGitHubServer {
    fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn set_latest_release_response(&self, release: Release) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(
            "/repos/test-owner/test-repo/releases/latest".to_string(),
            serde_json::to_value(release).unwrap(),
        );
    }

    fn set_error_response(&self, path: &str, status_code: u16) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(
            path.to_string(),
            json!({
                "error": true,
                "status": status_code,
                "message": "Mock error response"
            }),
        );
    }
}

/// Create a mock release for testing
fn create_mock_release(version: &str, has_windows_asset: bool) -> Release {
    let mut assets = Vec::new();

    if has_windows_asset {
        assets.push(ReleaseAsset {
            name: "installer-analyzer-windows-amd64.exe".to_string(),
            label: Some("Windows executable".to_string()),
            content_type: "application/octet-stream".to_string(),
            size: 1024 * 1024, // 1MB
            download_count: 100,
            browser_download_url: format!(
                "https://github.com/test-owner/test-repo/releases/download/{}/installer-analyzer-windows-amd64.exe",
                version
            ),
        });
    }

    // Add other platform assets
    assets.push(ReleaseAsset {
        name: "installer-analyzer-linux-amd64.tar.gz".to_string(),
        label: Some("Linux binary".to_string()),
        content_type: "application/gzip".to_string(),
        size: 800 * 1024, // 800KB
        download_count: 50,
        browser_download_url: format!(
            "https://github.com/test-owner/test-repo/releases/download/{}/installer-analyzer-linux-amd64.tar.gz",
            version
        ),
    });

    Release {
        tag_name: version.to_string(),
        name: format!("Release {}", version),
        body: Some(format!("Release notes for version {}", version)),
        draft: false,
        prerelease: version.contains("alpha") || version.contains("beta"),
        assets,
        published_at: Some("2025-01-01T00:00:00Z".to_string()),
    }
}

#[tokio::test]
async fn test_version_checker_current_version() {
    let checker = VersionChecker::new();
    let version = checker.get_current_version().unwrap();

    // Should return a valid version
    assert!(!version.to_string().is_empty());
    assert!(version.to_string().contains('.'));
}

#[tokio::test]
async fn test_version_checker_parse_version() {
    let checker = VersionChecker::new();

    // Test valid version formats
    let version1 = checker.parse_version("v1.2.3").unwrap();
    assert_eq!(version1.to_string(), "1.2.3");

    let version2 = checker.parse_version("1.2.3").unwrap();
    assert_eq!(version2.to_string(), "1.2.3");

    let version3 = checker.parse_version("v1.2.3-alpha.1").unwrap();
    assert_eq!(version3.to_string(), "1.2.3-alpha.1");

    // Test version comparison
    let v1 = checker.parse_version("1.0.0").unwrap();
    let v2 = checker.parse_version("1.0.1").unwrap();
    let v3 = checker.parse_version("1.1.0").unwrap();

    assert!(v2 > v1);
    assert!(v3 > v2);
    assert!(v3 > v1);
}

#[tokio::test]
async fn test_version_checker_invalid_version() {
    let checker = VersionChecker::new();

    // Test invalid version formats
    assert!(checker.parse_version("invalid").is_err());
    assert!(checker.parse_version("").is_err());
    assert!(checker.parse_version("v").is_err());
    assert!(checker.parse_version("1.2").is_err()); // Missing patch version
}

#[tokio::test]
async fn test_update_client_creation() {
    let _client = UpdateClient::new("test-owner", "test-repo");

    // Client should be created successfully
    // We can't test much without making actual HTTP requests
    // This test mainly ensures the constructor works
}

#[tokio::test]
async fn test_update_client_with_token() {
    let _client =
        UpdateClient::new("test-owner", "test-repo").with_token(Some("test-token".to_string()));

    // Client should be created successfully with token
    // Token is stored internally and used in requests
}

#[tokio::test]
async fn test_update_client_with_timeout() {
    let _client = UpdateClient::new("test-owner", "test-repo").with_timeout(60);

    // Client should be created successfully with custom timeout
}

#[tokio::test]
async fn test_download_manager_creation() {
    let _manager = DownloadManager::new();

    // Manager should be created successfully
    // We can't test download functionality without a real server
}

#[tokio::test]
async fn test_updater_creation() {
    let _updater = Updater::new();

    // Updater should be created with default configuration
}

#[tokio::test]
async fn test_updater_with_custom_config() {
    let config = UpdateConfig {
        repo_owner: "custom-owner".to_string(),
        repo_name: "custom-repo".to_string(),
        github_token: Some("custom-token".to_string()),
        timeout_seconds: 60,
        verify_signatures: true,
    };

    let _updater = Updater::with_config(config);

    // Updater should be created with custom configuration
}

#[tokio::test]
async fn test_update_config_default() {
    let config = UpdateConfig::default();

    assert_eq!(config.repo_owner, "loonghao");
    assert_eq!(config.repo_name, "installer-analyzer");
    assert_eq!(config.timeout_seconds, 30);
    assert!(!config.verify_signatures);
    assert!(config.github_token.is_none());
}

#[tokio::test]
async fn test_update_info_creation() {
    let current_version = Version::parse("1.0.0").unwrap();
    let latest_version = Version::parse("1.1.0").unwrap();

    let update_info = UpdateInfo {
        latest_version: latest_version.clone(),
        current_version: current_version.clone(),
        update_available: latest_version > current_version,
        download_url: "https://example.com/download".to_string(),
        file_size: 1024,
        file_hash: Some("abc123".to_string()),
        release_notes: Some("Test release notes".to_string()),
    };

    assert!(update_info.update_available);
    assert_eq!(update_info.download_url, "https://example.com/download");
    assert_eq!(update_info.file_size, 1024);
    assert_eq!(update_info.file_hash, Some("abc123".to_string()));
}

#[tokio::test]
async fn test_release_asset_serialization() {
    let asset = ReleaseAsset {
        name: "test.exe".to_string(),
        label: Some("Test executable".to_string()),
        content_type: "application/octet-stream".to_string(),
        size: 1024,
        download_count: 10,
        browser_download_url: "https://example.com/test.exe".to_string(),
    };

    // Test serialization and deserialization
    let json = serde_json::to_string(&asset).unwrap();
    let deserialized: ReleaseAsset = serde_json::from_str(&json).unwrap();

    assert_eq!(asset.name, deserialized.name);
    assert_eq!(asset.size, deserialized.size);
    assert_eq!(
        asset.browser_download_url,
        deserialized.browser_download_url
    );
}

#[tokio::test]
async fn test_release_serialization() {
    let release = create_mock_release("v1.0.0", true);

    // Test serialization and deserialization
    let json = serde_json::to_string(&release).unwrap();
    let deserialized: Release = serde_json::from_str(&json).unwrap();

    assert_eq!(release.tag_name, deserialized.tag_name);
    assert_eq!(release.name, deserialized.name);
    assert_eq!(release.assets.len(), deserialized.assets.len());
}

// Platform-specific tests
#[cfg(windows)]
mod windows_tests {
    use installer_analyzer::updater::windows::{
        can_self_update, get_update_strategy, UpdateStrategy, WindowsUpdater,
    };

    #[test]
    fn test_can_self_update() {
        // This test checks if we can write to the current executable location
        let can_update = can_self_update();

        // Result depends on where the test is running and permissions
        // We just ensure the function doesn't panic
        println!("Can self-update: {}", can_update);
    }

    #[test]
    fn test_get_update_strategy() {
        let strategy = get_update_strategy();

        // Should return a valid strategy
        match strategy {
            UpdateStrategy::Direct => println!("Strategy: Direct update"),
            UpdateStrategy::InPlace => println!("Strategy: In-place update"),
            UpdateStrategy::RequireElevation => println!("Strategy: Requires elevation"),
        }
    }

    #[tokio::test]
    async fn test_windows_updater_creation() {
        let _updater = WindowsUpdater::new();

        // Updater should be created successfully
        // We can't test the actual update process without admin rights
    }
}

// CLI integration tests for update command
mod cli_integration {
    use super::*;
    use std::process::Command;

    /// Get the path to the compiled binary
    fn get_binary_path() -> PathBuf {
        let mut path = std::env::current_exe().unwrap();
        path.pop(); // Remove test executable name
        if path.ends_with("deps") {
            path.pop(); // Remove deps directory
        }
        path.push("installer-analyzer");
        if cfg!(windows) {
            path.set_extension("exe");
        }
        path
    }

    #[test]
    fn test_update_command_help() {
        let binary = get_binary_path();

        let output = Command::new(&binary)
            .args(["update", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Check for and install updates"));
        assert!(stdout.contains("--check-only"));
        assert!(stdout.contains("--force"));
        assert!(stdout.contains("--yes"));
    }

    #[test]
    fn test_update_command_check_only() {
        let binary = get_binary_path();

        let output = Command::new(&binary)
            .args(["update", "--check-only"])
            .output()
            .expect("Failed to execute command");

        // Command should succeed (or fail gracefully)
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        // Should show version information or error message
        assert!(
            stdout.contains("Current version")
                || stdout.contains("Latest version")
                || stderr.contains("Error")
                || stdout.contains("You are running the latest version")
        );
    }

    #[test]
    fn test_update_command_in_main_help() {
        let binary = get_binary_path();

        let output = Command::new(&binary)
            .arg("--help")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("update"));
        assert!(stdout.contains("Check for and install updates"));
    }
}
