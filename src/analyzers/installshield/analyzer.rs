//! InstallShield analyzer implementation

use super::parser::InstallShieldParser;
use crate::analyzers::{common, InstallerAnalyzer};
use crate::core::{FileEntry, InstallerFormat, InstallerMetadata, RegistryOperation, Result};
use async_trait::async_trait;
use chrono::Utc;
use std::path::Path;

/// InstallShield installer analyzer
pub struct InstallShieldAnalyzer {
    parser: InstallShieldParser,
}

impl InstallShieldAnalyzer {
    /// Create a new InstallShield analyzer
    pub fn new() -> Self {
        Self {
            parser: InstallShieldParser::new(),
        }
    }

    /// Check if file is an InstallShield installer
    async fn is_installshield_file(file_path: &Path) -> Result<bool> {
        InstallShieldParser::is_installshield_file(file_path).await
    }

    /// Extract metadata from InstallShield file
    async fn extract_installshield_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        let file_size = common::get_file_size(file_path).await?;
        let file_hash = common::calculate_file_hash(file_path).await?;

        // Extract InstallShield-specific properties
        let properties = self.parser.extract_properties(file_path).await?;

        // Extract InstallShield metadata for product info
        let installshield_metadata = self.parser.extract_metadata(file_path).await?;

        let product_name = installshield_metadata.product_name.or_else(|| {
            // Fallback to filename parsing
            file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        });

        let product_version = installshield_metadata.product_version;
        let manufacturer = installshield_metadata
            .company_name
            .or_else(|| Some("Unknown".to_string()));

        Ok(InstallerMetadata {
            format: InstallerFormat::InstallShield,
            product_name,
            product_version,
            manufacturer,
            file_size,
            file_hash,
            created_at: Utc::now(),
            properties,
        })
    }

    /// Extract files from InstallShield package
    async fn extract_installshield_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        tracing::info!(
            "Extracting files from InstallShield package: {}",
            file_path.display()
        );

        let files = self.parser.extract_files(file_path).await?;

        tracing::info!("Found {} files in InstallShield package", files.len());

        Ok(files)
    }

    /// Extract registry operations from InstallShield package
    async fn extract_installshield_registry(
        &self,
        _file_path: &Path,
    ) -> Result<Vec<RegistryOperation>> {
        // InstallShield packages can contain registry operations, but extracting them
        // requires deep analysis of the InstallShield format and potentially running
        // the installer in a sandbox environment.
        //
        // For static analysis, we provide common InstallShield registry patterns
        // that are typically created during installation.

        let mut operations = Vec::new();

        // Common InstallShield registry entries
        let common_registry_ops = [
            ("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[ProductCode]", "DisplayName"),
            ("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[ProductCode]", "DisplayVersion"),
            ("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[ProductCode]", "Publisher"),
            ("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[ProductCode]", "InstallLocation"),
            ("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[ProductCode]", "UninstallString"),
            ("HKEY_LOCAL_MACHINE\\SOFTWARE\\[Company]\\[ProductName]", "InstallPath"),
            ("HKEY_LOCAL_MACHINE\\SOFTWARE\\[Company]\\[ProductName]", "Version"),
        ];

        for (key_path, value_name) in &common_registry_ops {
            operations.push(RegistryOperation::SetValue {
                key_path: key_path.to_string(),
                value_name: value_name.to_string(),
                value_type: crate::core::RegistryValueType::String,
                value_data: crate::core::RegistryValue::String("[Placeholder]".to_string()),
                timestamp: Utc::now(),
            });
        }

        tracing::info!(
            "Generated {} common registry operations for InstallShield package",
            operations.len()
        );

        Ok(operations)
    }
}

#[async_trait]
impl InstallerAnalyzer for InstallShieldAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Validate file accessibility
        common::validate_file(file_path).await?;

        // Check if it's an InstallShield installer
        Self::is_installshield_file(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::InstallShield
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;

        self.extract_installshield_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;

        self.extract_installshield_files(file_path).await
    }

    async fn extract_registry_operations(
        &self,
        file_path: &Path,
    ) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;

        self.extract_installshield_registry(file_path).await
    }
}

impl Default for InstallShieldAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
