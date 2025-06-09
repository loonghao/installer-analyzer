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
        // Build directory hierarchy mapping
        let dir_hierarchy = Self::build_directory_hierarchy(&directories);

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

            // Build full path by resolving directory hierarchy
            // For now, use the deepest directory path available (simplified approach)
            let full_path = Self::resolve_file_path(&file.component, &display_name, &dir_hierarchy);

            let path = PathBuf::from(&full_path);
            let target_path = Some(PathBuf::from(format!("TARGETDIR\\{}", full_path)));

            let attributes = FileAttributes {
                readonly: file.attributes.is_some_and(|a| a & 1 != 0),
                hidden: file.attributes.is_some_and(|a| a & 2 != 0),
                system: file.attributes.is_some_and(|a| a & 4 != 0),
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

    /// Build directory hierarchy mapping from MSI Directory table
    fn build_directory_hierarchy(directories: &[DirectoryEntry]) -> HashMap<String, String> {
        let mut dir_map: HashMap<String, String> = HashMap::new();
        let mut parent_map: HashMap<String, String> = HashMap::new();

        // First pass: collect directory names and parent relationships
        for dir in directories {
            // Parse directory name (may contain | separator for short|long names)
            let dir_name = if dir.default_dir.contains('|') {
                dir.default_dir
                    .split('|')
                    .nth(1)
                    .unwrap_or(&dir.default_dir)
                    .to_string()
            } else {
                dir.default_dir.clone()
            };

            dir_map.insert(dir.directory.clone(), dir_name);
            if let Some(parent) = &dir.directory_parent {
                parent_map.insert(dir.directory.clone(), parent.clone());
            }
        }

        // Second pass: resolve full paths
        let mut resolved_paths: HashMap<String, String> = HashMap::new();
        for dir_id in dir_map.keys() {
            let full_path = Self::resolve_directory_path(dir_id, &dir_map, &parent_map);
            resolved_paths.insert(dir_id.clone(), full_path);
        }

        resolved_paths
    }

    /// Resolve full directory path by walking up the parent hierarchy
    fn resolve_directory_path(
        dir_id: &str,
        dir_map: &HashMap<String, String>,
        parent_map: &HashMap<String, String>,
    ) -> String {
        let mut path_parts = Vec::new();
        let mut current_id = dir_id;

        // Walk up the directory hierarchy
        loop {
            if let Some(dir_name) = dir_map.get(current_id) {
                // Skip special directories like TARGETDIR
                if !dir_name.is_empty() && dir_name != "TARGETDIR" && dir_name != "." {
                    path_parts.push(dir_name.clone());
                }
            }

            if let Some(parent_id) = parent_map.get(current_id) {
                current_id = parent_id;
            } else {
                break;
            }
        }

        // Reverse to get correct order (root to leaf)
        path_parts.reverse();
        path_parts.join("\\")
    }

    /// Resolve file path within directory structure
    fn resolve_file_path(
        _component: &str,
        filename: &str,
        dir_hierarchy: &HashMap<String, String>,
    ) -> String {
        // For now, use a simplified approach
        // In a full implementation, we would need to query the Component table
        // to map components to directories

        // Try to find the deepest (most specific) directory path
        let deepest_path = dir_hierarchy
            .values()
            .filter(|path| !path.is_empty())
            .max_by_key(|path| path.matches('\\').count());

        if let Some(dir_path) = deepest_path {
            format!("{}\\{}", dir_path, filename)
        } else {
            filename.to_string()
        }
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
        if let Some(hex_str) = value_str.strip_prefix("#x") {
            // Binary data
            if let Ok(bytes) = hex::decode(hex_str) {
                return (RegistryValueType::Binary, RegistryValue::Binary(bytes));
            }
        } else if let Some(stripped) = value_str.strip_prefix("#") {
            // DWORD value
            if let Ok(dword) = stripped.parse::<u32>() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_hierarchy_building() {
        let directories = vec![
            DirectoryEntry {
                directory: "TARGETDIR".to_string(),
                directory_parent: None,
                default_dir: "SourceDir".to_string(),
            },
            DirectoryEntry {
                directory: "ProgramFilesFolder".to_string(),
                directory_parent: Some("TARGETDIR".to_string()),
                default_dir: "PFiles".to_string(),
            },
            DirectoryEntry {
                directory: "INSTALLDIR".to_string(),
                directory_parent: Some("ProgramFilesFolder".to_string()),
                default_dir: "MyApp".to_string(),
            },
        ];

        let hierarchy = MsiTables::build_directory_hierarchy(&directories);

        // TARGETDIR should contain "SourceDir" (not skipped because it's not empty, TARGETDIR, or ".")
        assert_eq!(hierarchy.get("TARGETDIR").unwrap(), "SourceDir");

        // ProgramFilesFolder should be "SourceDir\\PFiles"
        assert_eq!(
            hierarchy.get("ProgramFilesFolder").unwrap(),
            "SourceDir\\PFiles"
        );

        // INSTALLDIR should be "SourceDir\\PFiles\\MyApp"
        assert_eq!(
            hierarchy.get("INSTALLDIR").unwrap(),
            "SourceDir\\PFiles\\MyApp"
        );
    }

    #[test]
    fn test_file_path_resolution() {
        let mut dir_hierarchy = HashMap::new();
        dir_hierarchy.insert("TARGETDIR".to_string(), "".to_string());
        dir_hierarchy.insert("INSTALLDIR".to_string(), "Program Files\\MyApp".to_string());

        let file_path = MsiTables::resolve_file_path("Component1", "myapp.exe", &dir_hierarchy);

        // Should use the first non-empty directory path
        assert_eq!(file_path, "Program Files\\MyApp\\myapp.exe");
    }

    #[test]
    fn test_convert_to_file_entries_with_hierarchy() {
        let files = vec![FileTableEntry {
            file: "File1".to_string(),
            component: "Component1".to_string(),
            filename: "app.exe".to_string(),
            file_size: Some(1024),
            version: None,
            language: None,
            attributes: Some(0),
            sequence: Some(1),
        }];

        let directories = vec![
            DirectoryEntry {
                directory: "TARGETDIR".to_string(),
                directory_parent: None,
                default_dir: "SourceDir".to_string(),
            },
            DirectoryEntry {
                directory: "INSTALLDIR".to_string(),
                directory_parent: Some("TARGETDIR".to_string()),
                default_dir: "MyApp".to_string(),
            },
        ];

        let file_entries = MsiTables::convert_to_file_entries(files, directories);

        assert_eq!(file_entries.len(), 1);
        let entry = &file_entries[0];

        // Should have hierarchical path (SourceDir\\MyApp\\app.exe)
        assert_eq!(entry.path.to_string_lossy(), "SourceDir\\MyApp\\app.exe");
        assert_eq!(entry.size, 1024);
        assert!(entry.attributes.executable);
    }
}
