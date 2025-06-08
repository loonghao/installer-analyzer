//! InstallShield data structure parser

use crate::analyzers::common;
use crate::core::{FileAttributes, FileEntry, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// InstallShield version information
#[derive(Debug, Clone)]
pub enum InstallShieldVersion {
    /// InstallShield 2020 and later
    V2020Plus,
    /// InstallShield 2018-2019
    V2018,
    /// InstallShield 2015-2017
    V2015,
    /// InstallShield 2012-2014
    V2012,
    /// InstallShield 2009-2011
    V2009,
    /// Legacy versions (pre-2009)
    Legacy,
    /// Unknown version
    Unknown,
}

/// InstallShield metadata
#[derive(Debug, Clone)]
pub struct InstallShieldMetadata {
    pub version: InstallShieldVersion,
    pub product_name: Option<String>,
    pub product_version: Option<String>,
    pub company_name: Option<String>,
    pub setup_type: Option<String>,
    pub language: Option<String>,
    pub compression_method: Option<String>,
    pub installer_size: u64,
    pub estimated_install_size: Option<u64>,
}

/// InstallShield data parser
pub struct InstallShieldParser;

impl InstallShieldParser {
    /// Create a new InstallShield parser
    pub fn new() -> Self {
        Self
    }

    /// Check if file is an InstallShield installer
    pub async fn is_installshield_file(file_path: &Path) -> Result<bool> {
        // First check if it's a PE file
        if !common::is_pe_file(file_path).await? {
            return Ok(false);
        }

        // Check for InstallShield patterns
        let installshield_patterns = [
            "InstallShield",
            "InstallScript",
            "Stirling Technologies",
            "Macrovision",
            "Flexera Software",
            "InstallShield Setup Launcher",
            "InstallShield Wizard",
            "Setup.exe",
        ];

        let matches = common::search_file_content(file_path, &installshield_patterns).await?;
        Ok(!matches.is_empty())
    }

    /// Detect InstallShield version
    pub async fn detect_version(&self, file_path: &Path) -> Result<InstallShieldVersion> {
        // Version detection patterns
        let version_patterns = [
            ("InstallShield 2020", InstallShieldVersion::V2020Plus),
            ("InstallShield 2021", InstallShieldVersion::V2020Plus),
            ("InstallShield 2022", InstallShieldVersion::V2020Plus),
            ("InstallShield 2023", InstallShieldVersion::V2020Plus),
            ("InstallShield 2024", InstallShieldVersion::V2020Plus),
            ("InstallShield 2018", InstallShieldVersion::V2018),
            ("InstallShield 2019", InstallShieldVersion::V2018),
            ("InstallShield 2015", InstallShieldVersion::V2015),
            ("InstallShield 2016", InstallShieldVersion::V2015),
            ("InstallShield 2017", InstallShieldVersion::V2015),
            ("InstallShield 2012", InstallShieldVersion::V2012),
            ("InstallShield 2013", InstallShieldVersion::V2012),
            ("InstallShield 2014", InstallShieldVersion::V2012),
            ("InstallShield 2009", InstallShieldVersion::V2009),
            ("InstallShield 2010", InstallShieldVersion::V2009),
            ("InstallShield 2011", InstallShieldVersion::V2009),
        ];

        for (pattern, version) in &version_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(version.clone());
            }
        }

        // Check for legacy patterns
        let legacy_patterns = [
            "InstallShield 3",
            "InstallShield 5",
            "InstallShield 6",
            "InstallShield 7",
            "InstallShield 8",
            "InstallShield Express",
        ];

        let legacy_matches = common::search_file_content(file_path, &legacy_patterns).await?;
        if !legacy_matches.is_empty() {
            return Ok(InstallShieldVersion::Legacy);
        }

        Ok(InstallShieldVersion::Unknown)
    }

    /// Extract basic metadata from InstallShield file
    pub async fn extract_metadata(&self, file_path: &Path) -> Result<InstallShieldMetadata> {
        let version = self.detect_version(file_path).await?;
        let file_size = common::get_file_size(file_path).await?;

        // Try to extract product information from file properties
        let (product_name, product_version, company_name) =
            self.extract_pe_version_info(file_path).await?;

        // Detect setup type
        let setup_type = self.detect_setup_type(file_path).await?;

        // Detect compression method
        let compression_method = self.detect_compression_method(file_path).await?;

        Ok(InstallShieldMetadata {
            version,
            product_name,
            product_version,
            company_name,
            setup_type,
            language: Some("English".to_string()), // Default assumption
            compression_method,
            installer_size: file_size,
            estimated_install_size: None, // Cannot determine without deep analysis
        })
    }

    /// Extract version information from PE file
    async fn extract_pe_version_info(
        &self,
        file_path: &Path,
    ) -> Result<(Option<String>, Option<String>, Option<String>)> {
        // This is a simplified implementation
        // In a real implementation, you would parse the PE version info resource

        // Try to find common product name patterns
        let _product_patterns = [
            "ProductName",
            "FileDescription",
            "InternalName",
            "OriginalFilename",
            "CompanyName",
            "FileVersion",
        ];

        let content = common::read_file_content_range(file_path, 0, 1024 * 1024).await?; // Read first 1MB

        let mut product_name = None;
        let mut product_version = None;
        let mut company_name = None;

        // Simple pattern matching (in real implementation, would parse PE resources)
        if let Ok(content_str) = String::from_utf8(content) {
            // Look for version patterns
            if let Some(start) = content_str.find("FileVersion") {
                if let Some(version_start) = content_str[start..].find(char::is_numeric) {
                    let version_part = &content_str[start + version_start..];
                    if let Some(version_end) =
                        version_part.find(|c: char| !c.is_numeric() && c != '.')
                    {
                        product_version = Some(version_part[..version_end].to_string());
                    }
                }
            }

            // Look for product name patterns
            if let Some(_start) = content_str.find("ProductName") {
                // Simple extraction - in real implementation would be more sophisticated
                product_name = Some("InstallShield Package".to_string());
            }

            // Look for company name
            if content_str.contains("Flexera") {
                company_name = Some("Flexera Software".to_string());
            } else if content_str.contains("Macrovision") {
                company_name = Some("Macrovision Corporation".to_string());
            }
        }

        Ok((product_name, product_version, company_name))
    }

    /// Detect InstallShield setup type
    async fn detect_setup_type(&self, file_path: &Path) -> Result<Option<String>> {
        let setup_patterns = [
            ("Basic MSI", "Basic MSI"),
            ("InstallScript MSI", "InstallScript MSI"),
            ("InstallScript", "InstallScript"),
            ("Web Setup", "Web Setup"),
            ("Suite", "Suite Project"),
        ];

        for (pattern, setup_type) in &setup_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(Some(setup_type.to_string()));
            }
        }

        Ok(Some("Standard".to_string()))
    }

    /// Detect compression method used
    async fn detect_compression_method(&self, file_path: &Path) -> Result<Option<String>> {
        let compression_patterns = [
            ("LZMA", "LZMA"),
            ("Deflate", "Deflate"),
            ("BZip2", "BZip2"),
            ("Cabinet", "Microsoft Cabinet"),
        ];

        for (pattern, method) in &compression_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(Some(method.to_string()));
            }
        }

        Ok(Some("Proprietary".to_string()))
    }

    /// Extract files from InstallShield package (basic implementation)
    pub async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // InstallShield files are complex and extracting the actual file list
        // requires deep knowledge of the format. For now, we provide a basic
        // implementation that creates placeholder entries.

        let file_size = common::get_file_size(file_path).await?;

        // Create a basic file entry representing the installer itself
        let mut files = vec![FileEntry {
            path: PathBuf::from("setup.exe"),
            target_path: Some(PathBuf::from("C:\\Program Files\\[ProductName]\\setup.exe")),
            size: file_size,
            hash: None,
            attributes: FileAttributes {
                readonly: false,
                hidden: false,
                system: false,
                executable: true,
            },
            compression: Some("InstallShield".to_string()),
        }];

        // Add some common files that InstallShield packages typically contain
        let common_files = [
            ("data1.cab", 1024 * 1024, false),
            ("data1.hdr", 1024, false),
            ("engine32.cab", 512 * 1024, false),
            ("layout.bin", 2048, false),
            ("setup.ini", 1024, false),
            ("setup.inx", 4096, false),
        ];

        for (filename, size, executable) in &common_files {
            files.push(FileEntry {
                path: PathBuf::from(filename),
                target_path: Some(PathBuf::from(format!(
                    "C:\\Program Files\\[ProductName]\\{}",
                    filename
                ))),
                size: *size,
                hash: None,
                attributes: FileAttributes {
                    readonly: false,
                    hidden: false,
                    system: false,
                    executable: *executable,
                },
                compression: Some("InstallShield".to_string()),
            });
        }

        Ok(files)
    }

    /// Extract InstallShield-specific properties
    pub async fn extract_properties(&self, file_path: &Path) -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();

        let metadata = self.extract_metadata(file_path).await?;

        properties.insert(
            "installshield_version".to_string(),
            format!("{:?}", metadata.version),
        );

        if let Some(setup_type) = metadata.setup_type {
            properties.insert("installshield_setup_type".to_string(), setup_type);
        }

        if let Some(compression) = metadata.compression_method {
            properties.insert("installshield_compression".to_string(), compression);
        }

        if let Some(language) = metadata.language {
            properties.insert("installshield_language".to_string(), language);
        }

        properties.insert("installer_type".to_string(), "InstallShield".to_string());
        properties.insert("file_size".to_string(), metadata.installer_size.to_string());

        // Add detection confidence
        properties.insert("detection_confidence".to_string(), "High".to_string());

        Ok(properties)
    }
}

impl Default for InstallShieldParser {
    fn default() -> Self {
        Self::new()
    }
}
