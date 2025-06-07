//! MSIX/AppX analyzer implementation

use crate::core::{Result, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use crate::analyzers::{InstallerAnalyzer, common};
use super::parser::MsixParser;
use async_trait::async_trait;
use std::path::Path;
use chrono::Utc;

/// MSIX/AppX installer analyzer
pub struct MsixAnalyzer {
    parser: MsixParser,
}

impl MsixAnalyzer {
    /// Create a new MSIX analyzer
    pub fn new() -> Self {
        Self {
            parser: MsixParser::new(),
        }
    }

    /// Check if file is a MSIX/AppX package
    async fn is_msix_file(file_path: &Path) -> Result<bool> {
        MsixParser::is_msix_file(file_path).await
    }

    /// Extract metadata from MSIX/AppX file
    async fn extract_msix_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        let file_size = common::get_file_size(file_path).await?;
        let file_hash = common::calculate_file_hash(file_path).await?;

        // Extract MSIX-specific properties
        let properties = self.parser.extract_msix_properties(file_path).await?;

        // Extract manifest metadata for product info
        let (product_name, product_version, manufacturer) = match self.parser.extract_manifest(file_path) {
            Ok(manifest) => {
                let product_name = Some(manifest.display_name.clone());
                let product_version = Some(manifest.identity_version.clone());
                let manufacturer = Some(manifest.publisher_display_name.clone());
                (product_name, product_version, manufacturer)
            }
            Err(_) => {
                // Fallback to filename parsing
                let product_name = file_path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string());
                (product_name, None, Some("Unknown".to_string()))
            }
        };

        Ok(InstallerMetadata {
            format: InstallerFormat::MSIX,
            product_name,
            product_version,
            manufacturer,
            file_size,
            file_hash,
            created_at: Utc::now(),
            properties,
        })
    }

    /// Extract files from MSIX/AppX package
    async fn extract_msix_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        tracing::info!("Extracting files from MSIX/AppX package: {}", file_path.display());
        
        let files = self.parser.extract_files(file_path).await?;
        
        tracing::info!("Found {} files in MSIX/AppX package", files.len());
        
        Ok(files)
    }

    /// Extract registry operations (MSIX packages use a different deployment model)
    async fn extract_msix_registry(&self, _file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // MSIX packages use a containerized deployment model and don't directly modify
        // the system registry like traditional installers. They use a virtual registry
        // and package-specific registry hives that are managed by the Windows App Model.
        // 
        // For static analysis, we can't extract registry operations since they're
        // handled by the Windows deployment infrastructure at runtime.
        Ok(Vec::new())
    }
}

#[async_trait]
impl InstallerAnalyzer for MsixAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Validate file accessibility
        common::validate_file(file_path).await?;

        // Check if it's a MSIX/AppX package
        Self::is_msix_file(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::MSIX
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_msix_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_msix_files(file_path).await
    }

    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_msix_registry(file_path).await
    }
}

impl Default for MsixAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
