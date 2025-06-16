//! Windows-specific self-update implementation
//!
//! This module handles the Windows-specific logic for self-updating a single executable
//! while avoiding file locking issues through a "replace-and-restart" mechanism.

use crate::core::Result;
use crate::utils::is_admin;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Windows-specific updater for handling self-update operations
pub struct WindowsUpdater {
    /// Temporary directory for update operations
    temp_dir: PathBuf,
}

impl WindowsUpdater {
    /// Create a new Windows updater
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir().join("installer-analyzer-update");
        Self { temp_dir }
    }

    /// Create a new Windows updater with custom temp directory
    pub fn with_temp_dir(temp_dir: PathBuf) -> Self {
        Self { temp_dir }
    }

    /// Perform self-update by replacing the current executable
    pub async fn perform_self_update(&self, new_binary_path: &Path) -> Result<()> {
        tracing::info!("Starting Windows self-update process");

        // Validate the new binary exists and is accessible
        if !new_binary_path.exists() {
            return Err(crate::core::AnalyzerError::file_not_found(new_binary_path));
        }

        // Get current executable path
        let current_exe = std::env::current_exe().map_err(|e| {
            crate::core::AnalyzerError::generic(format!(
                "Failed to get current executable path: {}",
                e
            ))
        })?;

        tracing::info!(
            "Current executable: {}, New binary: {}",
            current_exe.display(),
            new_binary_path.display()
        );

        // Check if we have admin privileges (recommended for updates)
        if !is_admin() {
            tracing::warn!("Running without administrator privileges. Update may fail if the executable is in a protected location.");
        }

        // Ensure temp directory exists
        tokio::fs::create_dir_all(&self.temp_dir)
            .await
            .map_err(|e| {
                crate::core::AnalyzerError::generic(format!(
                    "Failed to create temp directory: {}",
                    e
                ))
            })?;

        // Create backup of current executable
        let backup_path = self.create_backup(&current_exe).await?;

        // Create batch script for file replacement
        let batch_script_path = self
            .create_update_batch_script(&current_exe, new_binary_path, &backup_path)
            .await?;

        // Launch the update process and exit current process
        self.launch_update_process(&batch_script_path)?;

        // This point should never be reached as the process should exit
        tracing::info!("Update process launched, current process should exit");
        std::process::exit(0);
    }

    /// Create a backup of the current executable
    async fn create_backup(&self, current_exe: &Path) -> Result<PathBuf> {
        let backup_filename = format!(
            "{}.backup.{}",
            current_exe
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("installer-analyzer"),
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );

        let backup_path = self.temp_dir.join(backup_filename);

        tracing::info!("Creating backup: {}", backup_path.display());

        tokio::fs::copy(current_exe, &backup_path)
            .await
            .map_err(|e| {
                crate::core::AnalyzerError::generic(format!("Failed to create backup: {}", e))
            })?;

        Ok(backup_path)
    }

    /// Create a batch script for performing the file replacement
    async fn create_update_batch_script(
        &self,
        current_exe: &Path,
        new_binary: &Path,
        backup_path: &Path,
    ) -> Result<PathBuf> {
        let script_path = self.temp_dir.join("update.bat");

        // Create batch script content
        let script_content = format!(
            r#"@echo off
echo Starting installer-analyzer update process...

REM Wait for the main process to exit
timeout /t 2 /nobreak >nul

REM Attempt to replace the executable
echo Replacing executable...
copy /Y "{new_binary}" "{current_exe}"

if %ERRORLEVEL% EQU 0 (
    echo Update successful!
    echo Cleaning up temporary files...
    
    REM Clean up the downloaded file
    del "{new_binary}" 2>nul
    
    REM Clean up backup (optional, keep for safety)
    REM del "{backup_path}" 2>nul
    
    echo Restarting application...
    start "" "{current_exe}"
    
    echo Update completed successfully.
) else (
    echo Update failed! Attempting to restore backup...
    copy /Y "{backup_path}" "{current_exe}"
    
    if %ERRORLEVEL% EQU 0 (
        echo Backup restored successfully.
        start "" "{current_exe}"
    ) else (
        echo CRITICAL ERROR: Failed to restore backup!
        echo Please manually restore from: {backup_path}
        pause
    )
)

REM Clean up this script (self-delete)
del "%~f0" 2>nul
"#,
            new_binary = new_binary.display(),
            current_exe = current_exe.display(),
            backup_path = backup_path.display()
        );

        // Write the batch script
        tokio::fs::write(&script_path, script_content)
            .await
            .map_err(|e| {
                crate::core::AnalyzerError::generic(format!("Failed to create batch script: {}", e))
            })?;

        tracing::info!("Created update batch script: {}", script_path.display());
        Ok(script_path)
    }

    /// Launch the update process using Windows API
    fn launch_update_process(&self, batch_script_path: &Path) -> Result<()> {
        tracing::info!("Launching update process: {}", batch_script_path.display());

        // Use Windows-specific process creation for better control
        #[cfg(windows)]
        {
            self.launch_with_windows_api(batch_script_path)
        }

        #[cfg(not(windows))]
        {
            Err(crate::core::AnalyzerError::generic(
                "Windows updater can only be used on Windows platform",
            ))
        }
    }

    /// Launch process using Windows API for better control
    #[cfg(windows)]
    fn launch_with_windows_api(&self, batch_script_path: &Path) -> Result<()> {
        // Use the simpler Command approach for now
        // Windows API implementation can be added later if needed
        self.launch_with_command(batch_script_path)
    }

    /// Fallback method using std::process::Command
    #[allow(dead_code)]
    fn launch_with_command(&self, batch_script_path: &Path) -> Result<()> {
        let mut command = Command::new("cmd");
        command.arg("/c").arg(batch_script_path);

        // On Windows, try to detach the process
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x00000008); // DETACHED_PROCESS
        }

        match command.spawn() {
            Ok(_) => {
                tracing::info!("Update process launched successfully");
                Ok(())
            }
            Err(e) => Err(crate::core::AnalyzerError::generic(format!(
                "Failed to launch update process: {}",
                e
            ))),
        }
    }

    /// Clean up temporary files (called on error or cancellation)
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

    /// Get the temp directory path
    pub fn get_temp_dir(&self) -> &Path {
        &self.temp_dir
    }
}

impl Default for WindowsUpdater {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if the current process can be updated (not running from a protected location)
pub fn can_self_update() -> bool {
    match std::env::current_exe() {
        Ok(exe_path) => {
            // Check if we can write to the directory containing the executable
            if let Some(parent_dir) = exe_path.parent() {
                // Try to create a temporary file in the same directory
                let test_file = parent_dir.join("update_test.tmp");
                match std::fs::write(&test_file, b"test") {
                    Ok(_) => {
                        // Clean up test file
                        let _ = std::fs::remove_file(&test_file);
                        true
                    }
                    Err(_) => false,
                }
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// Get recommended update strategy based on current environment
pub fn get_update_strategy() -> UpdateStrategy {
    if is_admin() {
        UpdateStrategy::Direct
    } else if can_self_update() {
        UpdateStrategy::InPlace
    } else {
        UpdateStrategy::RequireElevation
    }
}

/// Update strategy recommendations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateStrategy {
    /// Direct update (running as admin)
    Direct,
    /// In-place update (can write to exe directory)
    InPlace,
    /// Requires elevation (protected location)
    RequireElevation,
}

impl std::fmt::Display for UpdateStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateStrategy::Direct => write!(f, "Direct update (administrator)"),
            UpdateStrategy::InPlace => write!(f, "In-place update"),
            UpdateStrategy::RequireElevation => write!(f, "Requires elevation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_strategy_display() {
        assert_eq!(
            UpdateStrategy::Direct.to_string(),
            "Direct update (administrator)"
        );
        assert_eq!(UpdateStrategy::InPlace.to_string(), "In-place update");
        assert_eq!(
            UpdateStrategy::RequireElevation.to_string(),
            "Requires elevation"
        );
    }

    #[test]
    fn test_windows_updater_creation() {
        let updater = WindowsUpdater::new();
        assert!(updater
            .get_temp_dir()
            .to_string_lossy()
            .contains("installer-analyzer-update"));
    }

    #[test]
    fn test_get_update_strategy() {
        let strategy = get_update_strategy();
        // Should return one of the valid strategies
        match strategy {
            UpdateStrategy::Direct | UpdateStrategy::InPlace | UpdateStrategy::RequireElevation => {
                // Test passes
            }
        }
    }

    #[test]
    fn test_can_self_update() {
        // This test just ensures the function doesn't panic
        let _can_update = can_self_update();
        // Result depends on the environment, so we don't assert a specific value
    }
}
