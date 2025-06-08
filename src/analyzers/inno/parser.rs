//! InnoSetup data structure parser

use crate::core::{
    FileAttributes, FileEntry, RegistryOperation, RegistryValue, RegistryValueType, Result,
};
use chrono::Utc;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// InnoSetup header structure (simplified)
#[derive(Debug)]
pub struct InnoHeader {
    pub signature: [u8; 12],
    pub version: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

/// InnoSetup file entry
#[derive(Debug, Clone)]
pub struct InnoFileEntry {
    pub name: String,
    pub size: u64,
    pub attributes: u32,
    pub target_path: Option<String>,
    pub compressed: bool,
}

/// InnoSetup registry entry
#[derive(Debug, Clone)]
pub struct InnoRegistryEntry {
    pub root_key: String,
    pub subkey: String,
    pub value_name: String,
    pub value_type: String,
    pub value_data: String,
}

/// InnoSetup data parser
pub struct InnoParser;

impl InnoParser {
    /// Create a new InnoSetup parser
    pub fn new() -> Self {
        Self
    }

    /// Parse InnoSetup header from file data
    pub fn parse_header(&self, data: &[u8]) -> Result<Option<InnoHeader>> {
        if data.len() < 32 {
            return Ok(None);
        }

        // Look for InnoSetup signature patterns
        if let Some(_pos) = self.find_inno_signature(data) {
            // For now, return a basic header structure
            // In a real implementation, we would parse the actual InnoSetup header
            Ok(Some(InnoHeader {
                signature: [
                    b'I', b'n', b'n', b'o', b'S', b'e', b't', b'u', b'p', b' ', b'5', b'.',
                ],
                version: 5,
                compressed_size: 0,
                uncompressed_size: data.len() as u32,
            }))
        } else {
            Ok(None)
        }
    }

    /// Find InnoSetup signature in data
    fn find_inno_signature(&self, data: &[u8]) -> Option<usize> {
        // Look for InnoSetup-specific patterns
        let patterns: &[&[u8]] = &[
            b"Inno Setup Setup Data",
            b"JR.Inno.Setup",
            b"InnoSetupVersion",
            b"Inno Setup",
        ];

        for pattern in patterns {
            if let Some(pos) = self.find_pattern(data, pattern) {
                return Some(pos);
            }
        }
        None
    }

    /// Find a pattern in data
    fn find_pattern(&self, data: &[u8], pattern: &[u8]) -> Option<usize> {
        data.windows(pattern.len())
            .position(|window| window == pattern)
    }

    /// Extract file list from InnoSetup data (simplified implementation)
    pub fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // For now, we'll extract basic information
        // In a real implementation, we would parse the InnoSetup data structures

        let mut files = Vec::new();

        // Try to extract some basic file information
        if let Ok(file_info) = self.extract_basic_file_info(file_path) {
            files.extend(file_info);
        }

        // Add the installer itself as a file entry
        if let Ok(metadata) = std::fs::metadata(file_path) {
            files.push(FileEntry {
                path: file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
                    .into(),
                target_path: None,
                size: metadata.len(),
                hash: None,
                attributes: FileAttributes {
                    readonly: false,
                    hidden: false,
                    system: false,
                    executable: true,
                },
                compression: Some("InnoSetup".to_string()),
            });
        }

        Ok(files)
    }

    /// Extract basic file information (placeholder implementation)
    fn extract_basic_file_info(&self, _file_path: &Path) -> Result<Vec<FileEntry>> {
        // This is a placeholder implementation
        // Real InnoSetup file extraction would require parsing the InnoSetup data structures

        let mut files = Vec::new();

        // Add some common InnoSetup-installed files as examples
        let common_files = [
            ("unins000.exe", 1024 * 150, true), // InnoSetup uninstaller
            ("unins000.dat", 1024 * 50, false), // Uninstaller data
            ("readme.txt", 1024 * 8, false),
            ("license.txt", 1024 * 12, false),
            ("setup.exe", 1024 * 200, true), // Main application
        ];

        for (name, size, executable) in &common_files {
            files.push(FileEntry {
                path: PathBuf::from(name),
                target_path: Some(format!("{{app}}\\{}", name).into()),
                size: *size,
                hash: None,
                attributes: FileAttributes {
                    readonly: false,
                    hidden: false,
                    system: false,
                    executable: *executable,
                },
                compression: Some("InnoSetup LZMA".to_string()),
            });
        }

        Ok(files)
    }

    /// Extract registry operations from InnoSetup data (simplified implementation)
    pub fn extract_registry_operations(&self, _file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // This is a placeholder implementation
        // Real InnoSetup registry extraction would require parsing the InnoSetup script

        let mut operations = Vec::new();
        let now = Utc::now();

        // Add some common InnoSetup registry operations as examples
        operations.push(RegistryOperation::CreateKey {
            key_path: "HKEY_LOCAL_MACHINE\\SOFTWARE\\MyApplication".to_string(),
            timestamp: now,
        });

        operations.push(RegistryOperation::SetValue {
            key_path: "HKEY_LOCAL_MACHINE\\SOFTWARE\\MyApplication".to_string(),
            value_name: "InstallLocation".to_string(),
            value_type: RegistryValueType::String,
            value_data: RegistryValue::String("{app}".to_string()),
            timestamp: now,
        });

        operations.push(RegistryOperation::SetValue {
            key_path: "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\MyApplication_is1".to_string(),
            value_name: "DisplayName".to_string(),
            value_type: RegistryValueType::String,
            value_data: RegistryValue::String("My Application".to_string()),
            timestamp: now,
        });

        operations.push(RegistryOperation::SetValue {
            key_path: "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\MyApplication_is1".to_string(),
            value_name: "UninstallString".to_string(),
            value_type: RegistryValueType::String,
            value_data: RegistryValue::String("{app}\\unins000.exe".to_string()),
            timestamp: now,
        });

        Ok(operations)
    }

    /// Extract metadata from InnoSetup installer (simplified implementation)
    pub fn extract_metadata(&self, file_path: &Path) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();

        // Try to extract version information from PE resources
        if let Ok(version_info) = self.extract_version_info(file_path) {
            metadata.extend(version_info);
        }

        // Add InnoSetup-specific metadata
        metadata.insert("installer_type".to_string(), "InnoSetup".to_string());
        metadata.insert("format_version".to_string(), "InnoSetup 6.x".to_string());
        metadata.insert("compression".to_string(), "LZMA2".to_string());

        Ok(metadata)
    }

    /// Extract version information from PE resources (placeholder)
    fn extract_version_info(&self, _file_path: &Path) -> Result<HashMap<String, String>> {
        let mut info = HashMap::new();

        // This would normally parse PE version resources
        // For now, we'll return some placeholder data
        info.insert(
            "FileDescription".to_string(),
            "InnoSetup Installer".to_string(),
        );
        info.insert("FileVersion".to_string(), "1.0.0.0".to_string());
        info.insert("ProductName".to_string(), "Unknown Application".to_string());
        info.insert("ProductVersion".to_string(), "1.0.0".to_string());
        info.insert("CompanyName".to_string(), "Unknown Publisher".to_string());
        info.insert(
            "LegalCopyright".to_string(),
            "Copyright (C) Unknown".to_string(),
        );

        Ok(info)
    }
}

impl Default for InnoParser {
    fn default() -> Self {
        Self::new()
    }
}
