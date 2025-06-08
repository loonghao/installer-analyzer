//! Python Wheel analyzer implementation

use super::parser::WheelParser;
use crate::analyzers::{common, InstallerAnalyzer};
use crate::core::{FileEntry, InstallerFormat, InstallerMetadata, RegistryOperation, Result};
use async_trait::async_trait;
use chrono::Utc;
use std::path::Path;

/// Python Wheel installer analyzer
pub struct WheelAnalyzer {
    parser: WheelParser,
}

impl WheelAnalyzer {
    /// Create a new wheel analyzer
    pub fn new() -> Self {
        Self {
            parser: WheelParser::new(),
        }
    }

    /// Check if file is a Python wheel
    async fn is_wheel_file(file_path: &Path) -> Result<bool> {
        WheelParser::is_wheel_file(file_path).await
    }

    /// Extract metadata from wheel file
    async fn extract_wheel_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        let file_size = common::get_file_size(file_path).await?;
        let file_hash = common::calculate_file_hash(file_path).await?;

        // Extract wheel-specific properties
        let properties = self.parser.extract_wheel_properties(file_path).await?;

        // Extract wheel metadata for product info
        let (product_name, product_version, manufacturer) =
            match self.parser.extract_metadata(file_path) {
                Ok(metadata) => {
                    let product_name = Some(metadata.name.clone());
                    let product_version = Some(metadata.version.clone());
                    let manufacturer = metadata
                        .author
                        .or(metadata.maintainer)
                        .or_else(|| Some("Unknown".to_string()));
                    (product_name, product_version, manufacturer)
                }
                Err(_) => {
                    // Fallback to filename parsing
                    let product_name = file_path.file_stem().and_then(|s| s.to_str()).map(|s| {
                        // Extract package name from wheel filename (name-version-python-abi-platform.whl)
                        s.split('-').next().unwrap_or(s).to_string()
                    });
                    (product_name, None, Some("Unknown".to_string()))
                }
            };

        Ok(InstallerMetadata {
            format: InstallerFormat::PythonWheel,
            product_name,
            product_version,
            manufacturer,
            file_size,
            file_hash,
            created_at: Utc::now(),
            properties,
        })
    }

    /// Extract files from wheel
    async fn extract_wheel_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        tracing::info!(
            "Extracting files from Python wheel: {}",
            file_path.display()
        );

        let files = self.parser.extract_files(file_path).await?;

        tracing::info!("Found {} files in Python wheel", files.len());

        Ok(files)
    }

    /// Extract registry operations (Python wheels don't have registry operations)
    async fn extract_wheel_registry(&self, _file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Python wheels don't contain Windows registry operations
        // They are installed via pip and don't modify the registry directly
        Ok(Vec::new())
    }
}

#[async_trait]
impl InstallerAnalyzer for WheelAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Validate file accessibility
        common::validate_file(file_path).await?;

        // Check if it's a Python wheel file
        Self::is_wheel_file(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::PythonWheel
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;

        self.extract_wheel_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;

        self.extract_wheel_files(file_path).await
    }

    async fn extract_registry_operations(
        &self,
        file_path: &Path,
    ) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;

        self.extract_wheel_registry(file_path).await
    }
}

impl Default for WheelAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
