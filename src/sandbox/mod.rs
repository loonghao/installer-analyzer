//! Sandbox functionality for dynamic analysis

use crate::core::{AnalysisResult, Result, SandboxConfig};
use std::path::Path;

pub mod controller;

// Re-export main types
pub use controller::SandboxController;

/// Main sandbox controller trait
#[allow(async_fn_in_trait)]
pub trait Sandbox {
    /// Run installer in sandbox and return analysis results
    async fn analyze_installer(&mut self, installer_path: &Path) -> Result<AnalysisResult>;

    /// Get current sandbox configuration
    fn config(&self) -> &SandboxConfig;

    /// Update sandbox configuration
    fn set_config(&mut self, config: SandboxConfig);
}
