//! Auto-update functionality for installer-analyzer
//!
//! This module provides automatic update capabilities, including version checking,
//! downloading new releases, and performing self-updates while handling file locking issues.

use crate::core::Result;

pub mod client;
pub mod download;
pub mod version;

#[cfg(windows)]
pub mod windows;

// Re-export main types
pub use client::UpdateClient;
pub use download::DownloadManager;
pub use version::{Version, VersionChecker};

#[cfg(windows)]
pub use windows::{can_self_update, get_update_strategy, UpdateStrategy, WindowsUpdater};

/// Update configuration
#[derive(Debug, Clone)]
pub struct UpdateConfig {
    /// GitHub repository owner
    pub repo_owner: String,
    /// GitHub repository name
    pub repo_name: String,
    /// GitHub API token (optional, for rate limiting)
    pub github_token: Option<String>,
    /// Update check timeout in seconds
    pub timeout_seconds: u64,
    /// Whether to verify file signatures
    pub verify_signatures: bool,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            repo_owner: "loonghao".to_string(),
            repo_name: "installer-analyzer".to_string(),
            github_token: None,
            timeout_seconds: 30,
            verify_signatures: false,
        }
    }
}

/// Update information
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    /// Latest available version
    pub latest_version: Version,
    /// Current version
    pub current_version: Version,
    /// Whether an update is available
    pub update_available: bool,
    /// Download URL for the update
    pub download_url: String,
    /// File size in bytes
    pub file_size: u64,
    /// SHA256 hash of the file
    pub file_hash: Option<String>,
    /// Release notes
    pub release_notes: Option<String>,
}

/// Main updater interface
pub struct Updater {
    #[allow(dead_code)] // Used in future implementations
    config: UpdateConfig,
    client: UpdateClient,
    version_checker: VersionChecker,
    download_manager: DownloadManager,
}

impl Updater {
    /// Create a new updater with default configuration
    pub fn new() -> Self {
        Self::with_config(UpdateConfig::default())
    }

    /// Create a new updater with custom configuration
    pub fn with_config(config: UpdateConfig) -> Self {
        let client = UpdateClient::new(&config.repo_owner, &config.repo_name)
            .with_timeout(config.timeout_seconds)
            .with_token(config.github_token.clone());

        let version_checker = VersionChecker::new();
        let download_manager = DownloadManager::new();

        Self {
            config,
            client,
            version_checker,
            download_manager,
        }
    }

    /// Check for available updates
    pub async fn check_for_updates(&self) -> Result<UpdateInfo> {
        tracing::info!("Checking for updates...");

        let current_version = self.version_checker.get_current_version()?;
        let latest_release = self.client.get_latest_release().await?;
        let latest_version = self
            .version_checker
            .parse_version(&latest_release.tag_name)?;

        let update_available = latest_version > current_version;

        // Find the appropriate download URL for the current platform
        let download_url = self.find_platform_download_url(&latest_release.assets)?;

        let update_info = UpdateInfo {
            latest_version: latest_version.clone(),
            current_version,
            update_available,
            download_url: download_url.clone(),
            file_size: 0,    // Will be filled by download manager
            file_hash: None, // Will be filled by download manager
            release_notes: latest_release.body,
        };

        tracing::info!(
            "Update check completed. Current: {}, Latest: {}, Available: {}",
            update_info.current_version,
            update_info.latest_version,
            update_info.update_available
        );

        Ok(update_info)
    }

    /// Download and install an update
    pub async fn perform_update(&self, update_info: &UpdateInfo) -> Result<()> {
        if !update_info.update_available {
            return Ok(());
        }

        tracing::info!("Starting update process...");

        // Download the new version
        let downloaded_file = self
            .download_manager
            .download_file(&update_info.download_url)
            .await?;

        // Verify the downloaded file
        if let Some(expected_hash) = &update_info.file_hash {
            self.download_manager
                .verify_file_hash(&downloaded_file, expected_hash)
                .await?;
        }

        // Perform platform-specific update
        #[cfg(windows)]
        {
            let windows_updater = crate::updater::windows::WindowsUpdater::new();
            windows_updater.perform_self_update(&downloaded_file).await
        }

        #[cfg(not(windows))]
        {
            Err(crate::core::AnalyzerError::generic(
                "Self-update is currently only supported on Windows",
            ))
        }
    }

    /// Find the appropriate download URL for the current platform
    fn find_platform_download_url(&self, assets: &[client::ReleaseAsset]) -> Result<String> {
        // Look for Windows executable
        #[cfg(windows)]
        {
            for asset in assets {
                if asset.name.contains("windows") && asset.name.ends_with(".exe") {
                    return Ok(asset.browser_download_url.clone());
                }
            }
        }

        // Look for generic executable
        for asset in assets {
            if asset.name.ends_with(".exe") {
                return Ok(asset.browser_download_url.clone());
            }
        }

        Err(crate::core::AnalyzerError::generic(
            "No suitable download found for current platform",
        ))
    }
}

impl Default for Updater {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl Updater {
    /// Create a test updater with mock configuration
    pub fn new_for_testing(repo_owner: &str, repo_name: &str) -> Self {
        let config = UpdateConfig {
            repo_owner: repo_owner.to_string(),
            repo_name: repo_name.to_string(),
            github_token: None,
            timeout_seconds: 5, // Shorter timeout for tests
            verify_signatures: false,
        };
        Self::with_config(config)
    }

    /// Get the current configuration (for testing)
    pub fn get_config(&self) -> &UpdateConfig {
        &self.config
    }
}

#[cfg(test)]
impl UpdateInfo {
    /// Create a test UpdateInfo instance
    pub fn new_for_testing(
        current_version: &str,
        latest_version: &str,
        download_url: &str,
    ) -> Result<Self> {
        let current = Version::parse(current_version)?;
        let latest = Version::parse(latest_version)?;

        Ok(Self {
            latest_version: latest.clone(),
            current_version: current.clone(),
            update_available: latest > current,
            download_url: download_url.to_string(),
            file_size: 1024 * 1024, // 1MB default
            file_hash: Some("test-hash".to_string()),
            release_notes: Some("Test release notes".to_string()),
        })
    }
}
