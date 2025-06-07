//! Archive analyzer implementation

use crate::core::{Result, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use crate::analyzers::{InstallerAnalyzer, common};
use super::parser::{ArchiveParser, ArchiveFormat};
use async_trait::async_trait;
use std::path::Path;
use chrono::Utc;

/// Archive installer analyzer
pub struct ArchiveAnalyzer {
    parser: ArchiveParser,
}

impl ArchiveAnalyzer {
    /// Create a new archive analyzer
    pub fn new() -> Self {
        Self {
            parser: ArchiveParser::new(),
        }
    }

    /// Check if file is a supported archive format
    async fn is_supported_archive(file_path: &Path) -> Result<bool> {
        ArchiveParser::is_archive_file(file_path).await
    }

    /// Extract metadata from archive
    async fn extract_archive_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        let file_size = common::get_file_size(file_path).await?;
        let file_hash = common::calculate_file_hash(file_path).await?;

        // Extract metadata using parser
        let parser_metadata = self.parser.extract_metadata(file_path).await?;

        // Build metadata structure
        let product_name = file_path.file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        let manufacturer = Some("Unknown".to_string());

        // Determine format based on archive type
        let format = match ArchiveParser::detect_format(file_path).await? {
            ArchiveFormat::Zip => InstallerFormat::Unknown, // Will be overridden by specific analyzers
            ArchiveFormat::SevenZ => InstallerFormat::Unknown,
            ArchiveFormat::Unknown => InstallerFormat::Unknown,
        };

        // Combine all properties
        let mut properties = parser_metadata;
        properties.insert("analyzer_type".to_string(), "Archive".to_string());
        properties.insert("analyzer_version".to_string(), env!("CARGO_PKG_VERSION").to_string());

        Ok(InstallerMetadata {
            format,
            product_name,
            product_version: None,
            manufacturer,
            file_size,
            file_hash,
            created_at: Utc::now(),
            properties,
        })
    }

    /// Extract files from archive
    async fn extract_archive_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        tracing::info!("Extracting files from archive: {}", file_path.display());
        
        let files = self.parser.extract_files(file_path).await?;
        
        tracing::info!("Found {} files in archive", files.len());
        
        Ok(files)
    }

    /// Extract registry operations (archives typically don't have registry operations)
    async fn extract_archive_registry(&self, _file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Archives themselves don't contain registry operations
        // This would be handled by specific format analyzers that use archives
        Ok(Vec::new())
    }
}

#[async_trait]
impl InstallerAnalyzer for ArchiveAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Validate file accessibility
        common::validate_file(file_path).await?;

        // Check if it's a supported archive format
        Self::is_supported_archive(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        // Archive analyzer is a base analyzer, specific formats will override this
        InstallerFormat::Unknown
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_archive_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_archive_files(file_path).await
    }

    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_archive_registry(file_path).await
    }
}

impl Default for ArchiveAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
