//! NSIS analyzer implementation

use crate::core::{Result, AnalyzerError, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use crate::analyzers::{InstallerAnalyzer, common};
use super::parser::NsisParser;
use async_trait::async_trait;
use std::path::Path;
use std::collections::HashMap;
use chrono::Utc;

/// NSIS installer analyzer
pub struct NsisAnalyzer {
    parser: NsisParser,
}

impl NsisAnalyzer {
    /// Create a new NSIS analyzer
    pub fn new() -> Self {
        Self {
            parser: NsisParser::new(),
        }
    }

    /// Check if file contains NSIS signatures
    async fn has_nsis_signature(file_path: &Path) -> Result<bool> {
        // First check if it's a PE file
        if !common::is_pe_file(file_path).await? {
            return Ok(false);
        }

        // Search for NSIS-specific patterns
        let nsis_patterns = [
            "Nullsoft.NSIS.exehead",
            "NullsoftInst", 
            "NSIS Error",
            "Nullsoft Install System",
        ];

        let matches = common::search_file_content(file_path, &nsis_patterns).await?;
        let has_nsis = !matches.is_empty();

        if has_nsis {
            tracing::info!("NSIS signatures found in {}: {:?}", file_path.display(), matches);
        }

        Ok(has_nsis)
    }

    /// Extract metadata from NSIS installer
    async fn extract_nsis_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        let file_size = common::get_file_size(file_path).await?;
        let file_hash = common::calculate_file_hash(file_path).await?;

        // Extract metadata using parser
        let parser_metadata = self.parser.extract_metadata(file_path)?;

        // Build metadata structure
        let product_name = parser_metadata.get("ProductName").cloned()
            .or_else(|| {
                // Try to extract from filename
                file_path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            });

        let product_version = parser_metadata.get("ProductVersion").cloned()
            .or_else(|| parser_metadata.get("FileVersion").cloned());

        let manufacturer = parser_metadata.get("CompanyName").cloned()
            .or_else(|| Some("Unknown Publisher".to_string()));

        // Combine all properties
        let mut properties = parser_metadata;
        properties.insert("format_type".to_string(), "NSIS Installer".to_string());
        properties.insert("analyzer_version".to_string(), env!("CARGO_PKG_VERSION").to_string());

        Ok(InstallerMetadata {
            format: InstallerFormat::NSIS,
            product_name,
            product_version,
            manufacturer,
            file_size,
            file_hash,
            created_at: Utc::now(),
            properties,
        })
    }

    /// Extract files from NSIS installer
    async fn extract_nsis_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        tracing::info!("Extracting files from NSIS installer: {}", file_path.display());
        
        let files = self.parser.extract_files(file_path)?;
        
        tracing::info!("Found {} files in NSIS installer", files.len());
        
        Ok(files)
    }

    /// Extract registry operations from NSIS installer
    async fn extract_nsis_registry(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        tracing::info!("Extracting registry operations from NSIS installer: {}", file_path.display());
        
        let operations = self.parser.extract_registry_operations(file_path)?;
        
        tracing::info!("Found {} registry operations in NSIS installer", operations.len());
        
        Ok(operations)
    }
}

#[async_trait]
impl InstallerAnalyzer for NsisAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Check file extension first for quick filtering
        if let Some(ext) = file_path.extension() {
            if ext.to_str() != Some("exe") {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        // Validate file accessibility
        common::validate_file(file_path).await?;

        // Check for NSIS signatures
        Self::has_nsis_signature(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::NSIS
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_nsis_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_nsis_files(file_path).await
    }

    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_nsis_registry(file_path).await
    }
}

impl Default for NsisAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
