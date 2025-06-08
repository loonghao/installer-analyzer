//! MSI database table structures and queries

use crate::analyzers::msi::database::MsiDatabase;
use crate::core::{
    FileAttributes, FileEntry, RegistryOperation, RegistryValue, RegistryValueType, Result,
};
use chrono::Utc;
use std::collections::HashMap;
use std::path::PathBuf;

/// MSI Property table entry
#[derive(Debug, Clone)]
pub struct PropertyEntry {
    pub property: String,
    pub value: String,
}

/// MSI File table entry
#[derive(Debug, Clone)]
pub struct FileTableEntry {
    pub file: String,
    pub component: String,
    pub filename: String,
    pub file_size: Option<i32>,
    pub version: Option<String>,
    pub language: Option<String>,
    pub attributes: Option<i32>,
    pub sequence: Option<i32>,
}

/// MSI Directory table entry
#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub directory: String,
    pub directory_parent: Option<String>,
    pub default_dir: String,
}

/// MSI Registry table entry
#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub registry: String,
    pub root: i32,
    pub key: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub component: String,
}

/// MSI table queries and parsers
pub struct MsiTables;

impl MsiTables {
    /// Query the Property table
    pub fn query_properties(db: &MsiDatabase) -> Result<Vec<PropertyEntry>> {
        let view = db.execute_query("SELECT `Property`, `Value` FROM `Property`")?;
        let records = view.collect_records()?;

        let mut properties = Vec::new();
        for record in records {
            let property = record.get_string(1)?;
            let value = record.get_string(2)?;

            properties.push(PropertyEntry { property, value });
        }

        Ok(properties)
    }

    /// Query the File table
    pub fn query_files(db: &MsiDatabase) -> Result<Vec<FileTableEntry>> {
        let query = "SELECT `File`, `Component_`, `FileName`, `FileSize`, `Version`, `Language`, `Attributes`, `Sequence` FROM `File`";
        let view = db.execute_query(query)?;
        let records = view.collect_records()?;

        let mut files = Vec::new();
        for record in records {
            let file = record.get_string(1)?;
            let component = record.get_string(2)?;
            let filename = record.get_string(3)?;

            let file_size = if record.is_null(4) {
                None
            } else {
                Some(record.get_integer(4)?)
            };
            let version = if record.is_null(5) {
                None
            } else {
                Some(record.get_string(5)?)
            };
            let language = if record.is_null(6) {
                None
            } else {
                Some(record.get_string(6)?)
            };
            let attributes = if record.is_null(7) {
                None
            } else {
                Some(record.get_integer(7)?)
            };
            let sequence = if record.is_null(8) {
                None
            } else {
                Some(record.get_integer(8)?)
            };

            files.push(FileTableEntry {
                file,
                component,
                filename,
                file_size,
                version,
                language,
                attributes,
                sequence,
            });
        }

        Ok(files)
    }

    /// Query the Directory table
    pub fn query_directories(db: &MsiDatabase) -> Result<Vec<DirectoryEntry>> {
        let query = "SELECT `Directory`, `Directory_Parent`, `DefaultDir` FROM `Directory`";
        let view = db.execute_query(query)?;
        let records = view.collect_records()?;

        let mut directories = Vec::new();
        for record in records {
            let directory = record.get_string(1)?;
            let directory_parent = if record.is_null(2) {
                None
            } else {
                Some(record.get_string(2)?)
            };
            let default_dir = record.get_string(3)?;

            directories.push(DirectoryEntry {
                directory,
                directory_parent,
                default_dir,
            });
        }

        Ok(directories)
    }

    /// Query the Registry table
    pub fn query_registry(db: &MsiDatabase) -> Result<Vec<RegistryEntry>> {
        let query =
            "SELECT `Registry`, `Root`, `Key`, `Name`, `Value`, `Component_` FROM `Registry`";
        let view = db.execute_query(query)?;
        let records = view.collect_records()?;

        let mut registry_entries = Vec::new();
        for record in records {
            let registry = record.get_string(1)?;
            let root = record.get_integer(2)?;
            let key = record.get_string(3)?;
            let name = if record.is_null(4) {
                None
            } else {
                Some(record.get_string(4)?)
            };
            let value = if record.is_null(5) {
                None
            } else {
                Some(record.get_string(5)?)
            };
            let component = record.get_string(6)?;

            registry_entries.push(RegistryEntry {
                registry,
                root,
                key,
                name,
                value,
                component,
            });
        }

        Ok(registry_entries)
    }

    /// Convert MSI file entries to our FileEntry format
    pub fn convert_to_file_entries(
        files: Vec<FileTableEntry>,
        directories: Vec<DirectoryEntry>,
    ) -> Vec<FileEntry> {
        // Build directory path mapping
        let mut dir_map: HashMap<String, String> = HashMap::new();

        // First pass: collect all directories
        for dir in &directories {
            dir_map.insert(dir.directory.clone(), dir.default_dir.clone());
        }

        // Second pass: resolve full paths (simplified)
        let mut file_entries = Vec::new();
        for file in files {
            // Parse filename (may contain | separator for short|long names)
            let display_name = if file.filename.contains('|') {
                file.filename
                    .split('|')
                    .nth(1)
                    .unwrap_or(&file.filename)
                    .to_string()
            } else {
                file.filename.clone()
            };

            let path = PathBuf::from(&display_name);
            let target_path = Some(PathBuf::from(format!("TARGETDIR\\{}", display_name)));

            let attributes = FileAttributes {
                readonly: file.attributes.map_or(false, |a| a & 1 != 0),
                hidden: file.attributes.map_or(false, |a| a & 2 != 0),
                system: file.attributes.map_or(false, |a| a & 4 != 0),
                executable: display_name.ends_with(".exe") || display_name.ends_with(".dll"),
            };

            file_entries.push(FileEntry {
                path,
                target_path,
                size: file.file_size.unwrap_or(0) as u64,
                hash: None, // Would need to extract from CAB to calculate
                attributes,
                compression: Some("CAB".to_string()),
            });
        }

        file_entries
    }

    /// Convert MSI registry entries to our RegistryOperation format
    pub fn convert_to_registry_operations(entries: Vec<RegistryEntry>) -> Vec<RegistryOperation> {
        let mut operations = Vec::new();

        for entry in entries {
            let key_path = Self::format_registry_key(entry.root, &entry.key);

            if let Some(name) = entry.name {
                if let Some(value_str) = entry.value {
                    // Determine value type and parse value
                    let (value_type, value_data) = Self::parse_registry_value(&value_str);

                    operations.push(RegistryOperation::SetValue {
                        key_path,
                        value_name: name,
                        value_type,
                        value_data,
                        timestamp: Utc::now(),
                    });
                }
            } else {
                // Key creation without value
                operations.push(RegistryOperation::CreateKey {
                    key_path,
                    timestamp: Utc::now(),
                });
            }
        }

        operations
    }

    /// Format registry key path from root and key
    fn format_registry_key(root: i32, key: &str) -> String {
        let root_name = match root {
            -2147483648 => "HKEY_CLASSES_ROOT",     // HKCR
            -2147483647 => "HKEY_CURRENT_USER",     // HKCU
            -2147483646 => "HKEY_LOCAL_MACHINE",    // HKLM
            -2147483645 => "HKEY_USERS",            // HKU
            -2147483644 => "HKEY_PERFORMANCE_DATA", // HKPD
            -2147483643 => "HKEY_CURRENT_CONFIG",   // HKCC
            -2147483642 => "HKEY_DYN_DATA",         // HKDD
            _ => "UNKNOWN_ROOT",
        };

        format!("{}\\{}", root_name, key)
    }

    /// Parse registry value string and determine type
    fn parse_registry_value(value_str: &str) -> (RegistryValueType, RegistryValue) {
        // MSI registry values can have prefixes indicating type
        if value_str.starts_with("#x") {
            // Binary data
            let hex_str = &value_str[2..];
            if let Ok(bytes) = hex::decode(hex_str) {
                return (RegistryValueType::Binary, RegistryValue::Binary(bytes));
            }
        } else if value_str.starts_with("#") {
            // DWORD value
            if let Ok(dword) = value_str[1..].parse::<u32>() {
                return (RegistryValueType::DWord, RegistryValue::DWord(dword));
            }
        }

        // Default to string
        (
            RegistryValueType::String,
            RegistryValue::String(value_str.to_string()),
        )
    }
}
