//! InnoSetup analyzer implementation

use crate::core::{Result, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use crate::analyzers::{InstallerAnalyzer, common};
use super::parser::InnoParser;
use async_trait::async_trait;
use std::path::Path;
use chrono::Utc;

/// InnoSetup installer analyzer
pub struct InnoAnalyzer {
    parser: InnoParser,
}

impl InnoAnalyzer {
    /// Create a new InnoSetup analyzer
    pub fn new() -> Self {
        Self {
            parser: InnoParser::new(),
        }
    }

    /// Check if file contains InnoSetup signatures
    async fn has_inno_signature(file_path: &Path) -> Result<bool> {
        // First check if it's a PE file
        if !common::is_pe_file(file_path).await? {
            return Ok(false);
        }

        // Search for InnoSetup-specific patterns
        let inno_patterns = [
            "Inno Setup Setup Data",
            "JR.Inno.Setup",
            "InnoSetupVersion",
            "Inno Setup",
            "Jordan Russell",  // InnoSetup creator
        ];

        let matches = common::search_file_content(file_path, &inno_patterns).await?;
        let has_inno = !matches.is_empty();

        if has_inno {
            tracing::info!("InnoSetup signatures found in {}: {:?}", file_path.display(), matches);
        }

        Ok(has_inno)
    }

    /// Extract metadata from InnoSetup installer
    async fn extract_inno_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
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
                    .map(|s| {
                        // Clean up common InnoSetup filename patterns
                        s.replace("Setup", "")
                         .replace("setup", "")
                         .replace("-x64", "")
                         .replace("-x86", "")
                         .replace("_", " ")
                         .trim()
                         .to_string()
                    })
                    .filter(|s| !s.is_empty())
            });

        let product_version = parser_metadata.get("ProductVersion").cloned()
            .or_else(|| parser_metadata.get("FileVersion").cloned());

        let manufacturer = parser_metadata.get("CompanyName").cloned()
            .or_else(|| Some("Unknown Publisher".to_string()));

        // Combine all properties
        let mut properties = parser_metadata;
        properties.insert("format_type".to_string(), "InnoSetup Installer".to_string());
        properties.insert("analyzer_version".to_string(), env!("CARGO_PKG_VERSION").to_string());

        Ok(InstallerMetadata {
            format: InstallerFormat::InnoSetup,
            product_name,
            product_version,
            manufacturer,
            file_size,
            file_hash,
            created_at: Utc::now(),
            properties,
        })
    }

    /// Extract files from InnoSetup installer
    async fn extract_inno_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        tracing::info!("Extracting files from InnoSetup installer: {}", file_path.display());
        
        let files = self.parser.extract_files(file_path)?;
        
        tracing::info!("Found {} files in InnoSetup installer", files.len());
        
        Ok(files)
    }

    /// Extract registry operations from InnoSetup installer
    async fn extract_inno_registry(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        tracing::info!("Extracting registry operations from InnoSetup installer: {}", file_path.display());
        
        let operations = self.parser.extract_registry_operations(file_path)?;
        
        tracing::info!("Found {} registry operations in InnoSetup installer", operations.len());
        
        Ok(operations)
    }
}

#[async_trait]
impl InstallerAnalyzer for InnoAnalyzer {
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

        // Check for InnoSetup signatures
        Self::has_inno_signature(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::InnoSetup
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_inno_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_inno_files(file_path).await
    }

    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_inno_registry(file_path).await
    }
}

impl Default for InnoAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
