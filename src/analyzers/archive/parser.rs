//! Archive data structure parser

use crate::core::{Result, AnalyzerError, FileEntry, FileAttributes};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use zip::ZipArchive;

/// Supported archive formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    /// ZIP format (including JAR, WAR, etc.)
    Zip,
    /// 7-Zip format
    SevenZ,
    /// Unknown or unsupported archive format
    Unknown,
}

/// Archive entry information
#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub name: String,
    pub size: u64,
    pub compressed_size: u64,
    pub is_directory: bool,
    pub compression_method: Option<String>,
}

/// Archive data parser
pub struct ArchiveParser;

impl ArchiveParser {
    /// Create a new archive parser
    pub fn new() -> Self {
        Self
    }

    /// Detect archive format from file header
    pub async fn detect_format(file_path: &Path) -> Result<ArchiveFormat> {
        let header = crate::analyzers::common::read_file_header(file_path, 8).await?;
        
        if header.len() >= 4 {
            // ZIP signature: PK (0x504B)
            if header[0] == 0x50 && header[1] == 0x4B {
                return Ok(ArchiveFormat::Zip);
            }
            
            // 7z signature: 7z¼¯' (0x377ABCAF271C)
            if header.len() >= 6 
                && header[0] == 0x37 && header[1] == 0x7A 
                && header[2] == 0xBC && header[3] == 0xAF 
                && header[4] == 0x27 && header[5] == 0x1C {
                return Ok(ArchiveFormat::SevenZ);
            }
        }
        
        Ok(ArchiveFormat::Unknown)
    }

    /// Check if file is a supported archive format
    pub async fn is_archive_file(file_path: &Path) -> Result<bool> {
        let format = Self::detect_format(file_path).await?;
        Ok(format != ArchiveFormat::Unknown)
    }

    /// Extract file list from ZIP archive
    pub fn extract_zip_files(&self, file_path: &Path) -> Result<Vec<ArchiveEntry>> {
        let file = std::fs::File::open(file_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| AnalyzerError::generic(format!("Failed to open ZIP archive: {}", e)))?;

        let mut entries = Vec::new();

        for i in 0..archive.len() {
            let zip_file = archive.by_index(i)
                .map_err(|e| AnalyzerError::generic(format!("Failed to read ZIP entry {}: {}", i, e)))?;

            let entry = ArchiveEntry {
                name: zip_file.name().to_string(),
                size: zip_file.size(),
                compressed_size: zip_file.compressed_size(),
                is_directory: zip_file.is_dir(),
                compression_method: Some(format!("{:?}", zip_file.compression())),
            };

            entries.push(entry);
        }

        Ok(entries)
    }

    /// Extract file list from 7z archive
    pub fn extract_7z_files(&self, file_path: &Path) -> Result<Vec<ArchiveEntry>> {
        // For now, we'll provide a simplified implementation
        // Real 7z parsing is complex and would require more detailed integration
        let file_size = std::fs::metadata(file_path)?.len();

        // Return a basic entry representing the 7z file itself
        let entries = vec![ArchiveEntry {
            name: file_path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            size: file_size,
            compressed_size: file_size,
            is_directory: false,
            compression_method: Some("7Z-LZMA".to_string()),
        }];

        Ok(entries)
    }

    /// Extract file list from archive (unified interface)
    pub async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        let format = Self::detect_format(file_path).await?;
        
        let archive_entries = match format {
            ArchiveFormat::Zip => self.extract_zip_files(file_path)?,
            ArchiveFormat::SevenZ => self.extract_7z_files(file_path)?,
            ArchiveFormat::Unknown => {
                return Err(AnalyzerError::unsupported_format(
                    format!("Unsupported archive format: {}", file_path.display())
                ));
            }
        };

        // Convert ArchiveEntry to FileEntry
        let mut file_entries = Vec::new();
        for entry in archive_entries {
            if !entry.is_directory {
                file_entries.push(FileEntry {
                    path: PathBuf::from(&entry.name),
                    target_path: Some(PathBuf::from(&entry.name)),
                    size: entry.size,
                    hash: None,
                    attributes: FileAttributes {
                        readonly: false,
                        hidden: false,
                        system: false,
                        executable: entry.name.ends_with(".exe") || entry.name.ends_with(".dll"),
                    },
                    compression: entry.compression_method,
                });
            }
        }

        Ok(file_entries)
    }

    /// Extract metadata from archive
    pub async fn extract_metadata(&self, file_path: &Path) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();
        
        let format = Self::detect_format(file_path).await?;
        metadata.insert("archive_format".to_string(), format!("{:?}", format));
        
        let file_size = crate::analyzers::common::get_file_size(file_path).await?;
        metadata.insert("file_size".to_string(), file_size.to_string());
        
        // Get entry count
        let entries = self.extract_files(file_path).await?;
        metadata.insert("entry_count".to_string(), entries.len().to_string());
        
        // Calculate total uncompressed size
        let total_size: u64 = entries.iter().map(|e| e.size).sum();
        metadata.insert("total_uncompressed_size".to_string(), total_size.to_string());
        
        // Calculate compression ratio
        if total_size > 0 {
            let compression_ratio = (file_size as f64 / total_size as f64) * 100.0;
            metadata.insert("compression_ratio".to_string(), format!("{:.2}%", compression_ratio));
        }

        Ok(metadata)
    }
}

impl Default for ArchiveParser {
    fn default() -> Self {
        Self::new()
    }
}
