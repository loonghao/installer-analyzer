//! Complete MSI analyzer implementation

use crate::core::{Result, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use crate::analyzers::{InstallerAnalyzer, common};
use crate::analyzers::msi::database::MsiDatabase;
use crate::analyzers::msi::tables::MsiTables;
use async_trait::async_trait;
use std::path::Path;
use chrono::Utc;
use std::collections::HashMap;

/// MSI format analyzer
pub struct MsiAnalyzer;

impl MsiAnalyzer {
    /// Create a new MSI analyzer
    pub fn new() -> Self {
        Self
    }

    /// Check if file has MSI signature
    async fn has_msi_signature(file_path: &Path) -> Result<bool> {
        let header = common::read_file_header(file_path, 8).await?;
        
        // MSI files start with the OLE compound document signature
        // D0CF11E0A1B11AE1 (little-endian)
        if header.len() >= 8 {
            let signature = &header[0..8];
            let msi_signature = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
            Ok(signature == msi_signature)
        } else {
            Ok(false)
        }
    }

    /// Extract metadata from MSI properties
    async fn extract_msi_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        let file_size = common::get_file_size(file_path).await?;
        let file_hash = common::calculate_file_hash(file_path).await?;

        // Try to open MSI database and extract properties
        let (product_name, product_version, manufacturer, mut properties) = 
            match MsiDatabase::open(file_path) {
                Ok(db) => {
                    match MsiTables::query_properties(&db) {
                        Ok(props) => {
                            let mut prop_map = HashMap::new();
                            let mut product_name = None;
                            let mut product_version = None;
                            let mut manufacturer = None;

                            for prop in props {
                                match prop.property.as_str() {
                                    "ProductName" => product_name = Some(prop.value.clone()),
                                    "ProductVersion" => product_version = Some(prop.value.clone()),
                                    "Manufacturer" => manufacturer = Some(prop.value.clone()),
                                    _ => {}
                                }
                                prop_map.insert(prop.property, prop.value);
                            }

                            (product_name, product_version, manufacturer, prop_map)
                        }
                        Err(e) => {
                            tracing::warn!("Failed to query MSI properties: {}", e);
                            (None, None, None, HashMap::new())
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to open MSI database: {}", e);
                    (None, None, None, HashMap::new())
                }
            };

        // Add format information
        properties.insert("format_version".to_string(), "MSI".to_string());
        properties.insert("file_type".to_string(), "Windows Installer Package".to_string());

        Ok(InstallerMetadata {
            format: InstallerFormat::MSI,
            product_name,
            product_version,
            manufacturer,
            file_size,
            file_hash,
            created_at: Utc::now(),
            properties,
        })
    }

    /// Extract files from MSI database
    async fn extract_msi_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        let db = MsiDatabase::open(file_path)?;
        
        // Query File and Directory tables
        let files = MsiTables::query_files(&db)?;
        let directories = MsiTables::query_directories(&db)?;
        
        tracing::info!("Found {} files and {} directories in MSI", files.len(), directories.len());
        
        // Convert to our FileEntry format
        let file_entries = MsiTables::convert_to_file_entries(files, directories);
        
        Ok(file_entries)
    }

    /// Extract registry operations from MSI database
    async fn extract_msi_registry(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        let db = MsiDatabase::open(file_path)?;
        
        // Query Registry table
        let registry_entries = MsiTables::query_registry(&db)?;
        
        tracing::info!("Found {} registry entries in MSI", registry_entries.len());
        
        // Convert to our RegistryOperation format
        let operations = MsiTables::convert_to_registry_operations(registry_entries);
        
        Ok(operations)
    }
}

#[async_trait]
impl InstallerAnalyzer for MsiAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Check file extension first
        if let Some(ext) = file_path.extension() {
            if ext.to_str() == Some("msi") {
                // Validate it's actually an MSI file
                return Self::has_msi_signature(file_path).await;
            }
        }
        
        // If no .msi extension, check signature anyway
        Self::has_msi_signature(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::MSI
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_msi_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_msi_files(file_path).await
    }

    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_msi_registry(file_path).await
    }
}

impl Default for MsiAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
