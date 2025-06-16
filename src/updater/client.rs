//! GitHub API client for fetching release information

use crate::core::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// GitHub release information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Release {
    /// Release tag name (version)
    pub tag_name: String,
    /// Release name
    pub name: String,
    /// Release body (description/notes)
    pub body: Option<String>,
    /// Whether this is a draft release
    pub draft: bool,
    /// Whether this is a prerelease
    pub prerelease: bool,
    /// Release assets (downloadable files)
    pub assets: Vec<ReleaseAsset>,
    /// Publication date
    pub published_at: Option<String>,
}

/// GitHub release asset information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReleaseAsset {
    /// Asset name
    pub name: String,
    /// Asset label
    pub label: Option<String>,
    /// Content type
    pub content_type: String,
    /// File size in bytes
    pub size: u64,
    /// Download count
    pub download_count: u64,
    /// Browser download URL
    pub browser_download_url: String,
}

/// GitHub API client for update checking
pub struct UpdateClient {
    client: Client,
    repo_owner: String,
    repo_name: String,
    github_token: Option<String>,
    base_url: String,
}

impl UpdateClient {
    /// Create a new GitHub API client
    pub fn new(repo_owner: &str, repo_name: &str) -> Self {
        let client = Client::builder()
            .user_agent("installer-analyzer-updater/1.0")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            repo_owner: repo_owner.to_string(),
            repo_name: repo_name.to_string(),
            github_token: None,
            base_url: "https://api.github.com".to_string(),
        }
    }

    /// Set GitHub API token for authentication
    pub fn with_token(mut self, token: Option<String>) -> Self {
        self.github_token = token;
        self
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.client = Client::builder()
            .user_agent("installer-analyzer-updater/1.0")
            .timeout(Duration::from_secs(timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        self
    }

    /// Get the latest release information
    pub async fn get_latest_release(&self) -> Result<Release> {
        let url = format!(
            "{}/repos/{}/{}/releases/latest",
            self.base_url, self.repo_owner, self.repo_name
        );

        tracing::debug!("Fetching latest release from: {}", url);

        let mut request = self.client.get(&url);

        // Add authentication header if token is provided
        if let Some(token) = &self.github_token {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to fetch release info: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(crate::core::AnalyzerError::generic(format!(
                "GitHub API request failed with status: {}",
                response.status()
            )));
        }

        let release: Release = response.json().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to parse release info: {}", e))
        })?;

        tracing::info!(
            "Retrieved latest release: {} ({})",
            release.tag_name,
            release.name
        );

        Ok(release)
    }

    /// Get all releases (paginated)
    pub async fn get_releases(&self, per_page: u32, page: u32) -> Result<Vec<Release>> {
        let url = format!(
            "{}/repos/{}/{}/releases?per_page={}&page={}",
            self.base_url, self.repo_owner, self.repo_name, per_page, page
        );

        tracing::debug!("Fetching releases from: {}", url);

        let mut request = self.client.get(&url);

        // Add authentication header if token is provided
        if let Some(token) = &self.github_token {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to fetch releases: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(crate::core::AnalyzerError::generic(format!(
                "GitHub API request failed with status: {}",
                response.status()
            )));
        }

        let releases: Vec<Release> = response.json().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to parse releases: {}", e))
        })?;

        tracing::info!("Retrieved {} releases", releases.len());

        Ok(releases)
    }

    /// Get a specific release by tag
    pub async fn get_release_by_tag(&self, tag: &str) -> Result<Release> {
        let url = format!(
            "{}/repos/{}/{}/releases/tags/{}",
            self.base_url, self.repo_owner, self.repo_name, tag
        );

        tracing::debug!("Fetching release by tag from: {}", url);

        let mut request = self.client.get(&url);

        // Add authentication header if token is provided
        if let Some(token) = &self.github_token {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to fetch release: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(crate::core::AnalyzerError::generic(format!(
                "GitHub API request failed with status: {}",
                response.status()
            )));
        }

        let release: Release = response.json().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to parse release: {}", e))
        })?;

        tracing::info!("Retrieved release: {} ({})", release.tag_name, release.name);

        Ok(release)
    }

    /// Check API rate limit status
    pub async fn check_rate_limit(&self) -> Result<RateLimitInfo> {
        let url = format!("{}/rate_limit", self.base_url);

        let mut request = self.client.get(&url);

        // Add authentication header if token is provided
        if let Some(token) = &self.github_token {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request.send().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to check rate limit: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(crate::core::AnalyzerError::generic(format!(
                "Rate limit check failed with status: {}",
                response.status()
            )));
        }

        let rate_limit: RateLimitResponse = response.json().await.map_err(|e| {
            crate::core::AnalyzerError::generic(format!("Failed to parse rate limit: {}", e))
        })?;

        Ok(rate_limit.rate)
    }
}

/// Rate limit information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RateLimitInfo {
    /// Maximum requests per hour
    pub limit: u32,
    /// Remaining requests
    pub remaining: u32,
    /// Reset time (Unix timestamp)
    pub reset: u64,
}

/// Rate limit response wrapper
#[derive(Debug, Deserialize)]
struct RateLimitResponse {
    rate: RateLimitInfo,
}
