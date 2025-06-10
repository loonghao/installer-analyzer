//! Template data structures for report generation

use crate::core::{AnalysisResult, FileEntry, RegistryOperation, RegistryValue};
use crate::utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Get the embedded HTML template
pub fn get_report_template() -> &'static str {
    // Use the built frontend template from dist/
    include_str!("../../frontend/dist/template.html")
}

/// Template data for HTML report generation
#[derive(Serialize, Deserialize)]
pub struct ReportTemplateData {
    pub metadata: MetadataData,
    pub summary: SummaryData,
    pub file_groups: FileGroupsData,
    pub file_tree: FileTreeData,
    pub file_tree_json: String,
    pub registry_operations: Vec<RegistryOperationData>,
    pub security: SecurityData,
    pub analyzed_at: String,
    pub analysis_type_class: String,
    pub analysis_type_icon: String,
    pub analysis_type_text: String,
    pub analysis_duration_formatted: String,
    pub session_id: String,
    pub version: String,
    pub chart_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataData {
    pub product_name: String,
    pub product_version: String,
    pub manufacturer: String,
    pub format: String,
    pub file_size_formatted: String,
    pub file_hash_short: String,
}

#[derive(Serialize, Deserialize)]
pub struct SummaryData {
    pub total_files: usize,
    pub registry_operations: usize,
    pub executables: usize,
    pub total_size_formatted: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileGroupsData {
    pub executables: FileGroupData,
    pub libraries: FileGroupData,
    pub resources: FileGroupData,
    pub others: FileGroupData,
}

#[derive(Serialize, Deserialize)]
pub struct FileGroupData {
    pub count: usize,
    pub files: Vec<FileItemData>,
    pub has_more: bool,
    pub remaining: usize,
}

#[derive(Serialize, Deserialize)]
pub struct FileItemData {
    pub name: String,
    pub size_formatted: String,
}

/// File tree node for hierarchical display
#[derive(Serialize, Deserialize, Clone)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub size_formatted: String,
    pub file_type: String,
    pub icon_class: String,
    pub children: Vec<FileTreeNode>,
    pub depth: usize,
}

/// File tree data for template
#[derive(Serialize, Deserialize)]
pub struct FileTreeData {
    pub nodes: Vec<FileTreeNode>,
    pub total_files: usize,
    pub total_directories: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RegistryOperationData {
    pub operation_class: String,
    pub operation_text: String,
    pub key_path: String,
    pub value_name: String,
    pub value_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct SecurityData {
    pub risk_level_class: String,
    pub risk_level_text: String,
    pub risk_icon: String,
    pub executable_count: usize,
    pub large_files: usize,
    pub total_size_formatted: String,
    pub registry_operations: usize,
    pub dynamic_analysis: String,
    pub file_modifications: usize,
}

impl ReportTemplateData {
    /// Create template data from analysis result
    pub fn from_analysis_result(result: &AnalysisResult) -> Self {
        let metadata = MetadataData {
            product_name: result
                .metadata
                .product_name
                .clone()
                .unwrap_or_else(|| "Unknown Package".to_string()),
            product_version: result
                .metadata
                .product_version
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            manufacturer: result
                .metadata
                .manufacturer
                .clone()
                .unwrap_or_else(|| "Unknown".to_string()),
            format: format!("{:?}", result.metadata.format),
            file_size_formatted: utils::format_file_size(result.metadata.file_size),
            file_hash_short: result.metadata.file_hash.chars().take(16).collect(),
        };

        let total_file_size: u64 = result.files.iter().map(|f| f.size).sum();
        let executable_count = result
            .files
            .iter()
            .filter(|f| f.attributes.executable)
            .count();

        let summary = SummaryData {
            total_files: result.files.len(),
            registry_operations: result.registry_operations.len(),
            executables: executable_count,
            total_size_formatted: utils::format_file_size(total_file_size),
        };

        let file_groups = Self::create_file_groups(&result.files);
        let file_tree = Self::build_file_tree(&result.files);
        let file_tree_json = serde_json::to_string(&file_tree).unwrap_or_else(|_| "{}".to_string());
        let registry_operations = Self::create_registry_operations(&result.registry_operations);
        let security = Self::create_security_data(result);
        let chart_data = Self::calculate_chart_data(&result.files);

        Self {
            metadata,
            summary,
            file_groups,
            file_tree,
            file_tree_json,
            registry_operations,
            security,
            analyzed_at: result
                .analyzed_at
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
            analysis_type_class: if result.dynamic_analysis {
                "bg-success".to_string()
            } else {
                "bg-info".to_string()
            },
            analysis_type_icon: if result.dynamic_analysis {
                "fa-play".to_string()
            } else {
                "fa-search".to_string()
            },
            analysis_type_text: if result.dynamic_analysis {
                "Dynamic Analysis".to_string()
            } else {
                "Static Analysis".to_string()
            },
            analysis_duration_formatted: utils::format_duration(result.analysis_duration),
            session_id: result.session_id.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            chart_data,
        }
    }

    /// Create file groups data
    fn create_file_groups(files: &[FileEntry]) -> FileGroupsData {
        let mut executables = Vec::new();
        let mut libraries = Vec::new();
        let mut resources = Vec::new();
        let mut others = Vec::new();

        for file in files {
            let path_str = file.path.to_string_lossy();
            let file_item = FileItemData {
                name: file
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                size_formatted: utils::format_file_size(file.size),
            };

            if path_str.ends_with(".exe") {
                executables.push(file_item);
            } else if path_str.ends_with(".dll") || path_str.ends_with(".so") {
                libraries.push(file_item);
            } else if path_str.ends_with(".pak")
                || path_str.ends_with(".dat")
                || path_str.ends_with(".ico")
            {
                resources.push(file_item);
            } else {
                others.push(file_item);
            }
        }

        FileGroupsData {
            executables: Self::create_file_group_data(executables),
            libraries: Self::create_file_group_data(libraries),
            resources: Self::create_file_group_data(resources),
            others: Self::create_file_group_data(others),
        }
    }

    /// Create file group data with pagination
    fn create_file_group_data(mut files: Vec<FileItemData>) -> FileGroupData {
        const MAX_DISPLAY: usize = 20;
        let total_count = files.len();
        let has_more = total_count > MAX_DISPLAY;
        let remaining = if has_more {
            total_count - MAX_DISPLAY
        } else {
            0
        };

        files.truncate(MAX_DISPLAY);

        FileGroupData {
            count: total_count,
            files,
            has_more,
            remaining,
        }
    }

    /// Create registry operations data
    fn create_registry_operations(operations: &[RegistryOperation]) -> Vec<RegistryOperationData> {
        operations
            .iter()
            .take(50)
            .map(|op| match op {
                RegistryOperation::CreateKey { key_path, .. } => RegistryOperationData {
                    operation_class: "bg-success".to_string(),
                    operation_text: "Create Key".to_string(),
                    key_path: key_path.clone(),
                    value_name: "-".to_string(),
                    value_data: "-".to_string(),
                },
                RegistryOperation::SetValue {
                    key_path,
                    value_name,
                    value_data,
                    ..
                } => {
                    let value_str = match value_data {
                        RegistryValue::String(s) => s.clone(),
                        RegistryValue::DWord(d) => format!("0x{:08x}", d),
                        RegistryValue::Binary(b) => format!("Binary ({} bytes)", b.len()),
                        _ => "Complex Value".to_string(),
                    };
                    RegistryOperationData {
                        operation_class: "bg-primary".to_string(),
                        operation_text: "Set Value".to_string(),
                        key_path: key_path.clone(),
                        value_name: value_name.clone(),
                        value_data: value_str,
                    }
                }
                RegistryOperation::DeleteKey { key_path, .. } => RegistryOperationData {
                    operation_class: "bg-danger".to_string(),
                    operation_text: "Delete Key".to_string(),
                    key_path: key_path.clone(),
                    value_name: "-".to_string(),
                    value_data: "-".to_string(),
                },
                RegistryOperation::DeleteValue {
                    key_path,
                    value_name,
                    ..
                } => RegistryOperationData {
                    operation_class: "bg-warning".to_string(),
                    operation_text: "Delete Value".to_string(),
                    key_path: key_path.clone(),
                    value_name: value_name.clone(),
                    value_data: "-".to_string(),
                },
            })
            .collect()
    }

    /// Create security analysis data
    fn create_security_data(result: &AnalysisResult) -> SecurityData {
        let executable_count = result
            .files
            .iter()
            .filter(|f| f.attributes.executable)
            .count();
        let total_size: u64 = result.files.iter().map(|f| f.size).sum();
        let large_files = result
            .files
            .iter()
            .filter(|f| f.size > 50 * 1024 * 1024)
            .count(); // > 50MB

        let (risk_level_text, risk_level_class, risk_icon) =
            if executable_count > 10 || large_files > 5 {
                ("High", "danger", "fa-exclamation-triangle")
            } else if executable_count > 5 || large_files > 2 {
                ("Medium", "warning", "fa-exclamation-circle")
            } else {
                ("Low", "success", "fa-check-circle")
            };

        SecurityData {
            risk_level_class: risk_level_class.to_string(),
            risk_level_text: risk_level_text.to_string(),
            risk_icon: risk_icon.to_string(),
            executable_count,
            large_files,
            total_size_formatted: utils::format_file_size(total_size),
            registry_operations: result.registry_operations.len(),
            dynamic_analysis: if result.dynamic_analysis {
                "Yes".to_string()
            } else {
                "No".to_string()
            },
            file_modifications: result.file_operations.len(),
        }
    }

    /// Calculate chart data for file size distribution
    fn calculate_chart_data(files: &[FileEntry]) -> String {
        let mut exe_size = 0u64;
        let mut dll_size = 0u64;
        let mut resource_size = 0u64;
        let mut doc_size = 0u64;
        let mut other_size = 0u64;

        for file in files {
            let path_str = file.path.to_string_lossy();
            if path_str.ends_with(".exe") {
                exe_size += file.size;
            } else if path_str.ends_with(".dll") || path_str.ends_with(".so") {
                dll_size += file.size;
            } else if path_str.ends_with(".pak")
                || path_str.ends_with(".dat")
                || path_str.ends_with(".ico")
            {
                resource_size += file.size;
            } else if path_str.ends_with(".html")
                || path_str.ends_with(".txt")
                || path_str.ends_with(".md")
            {
                doc_size += file.size;
            } else {
                other_size += file.size;
            }
        }

        format!(
            "[{}, {}, {}, {}, {}]",
            exe_size, dll_size, resource_size, doc_size, other_size
        )
    }

    /// Build file tree structure from flat file list
    fn build_file_tree(files: &[FileEntry]) -> FileTreeData {
        let mut root_nodes: HashMap<String, FileTreeNode> = HashMap::new();
        let mut total_files = 0;
        let mut total_directories = 0;

        for file in files {
            let path_str = file.path.to_string_lossy();
            let path_parts: Vec<&str> = path_str
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();

            if path_parts.is_empty() {
                continue;
            }

            total_files += 1;
            Self::insert_into_tree(
                &mut root_nodes,
                &path_parts,
                file,
                0,
                &mut total_directories,
            );
        }

        // Convert HashMap to sorted Vec
        let mut nodes: Vec<FileTreeNode> = root_nodes.into_values().collect();
        nodes.sort_by(|a, b| {
            // Directories first, then files, then alphabetical
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        // Sort children recursively
        Self::sort_tree_children(&mut nodes);

        FileTreeData {
            nodes,
            total_files,
            total_directories,
        }
    }

    /// Insert file into tree structure recursively
    fn insert_into_tree(
        nodes: &mut HashMap<String, FileTreeNode>,
        path_parts: &[&str],
        file: &FileEntry,
        depth: usize,
        total_directories: &mut usize,
    ) {
        if path_parts.is_empty() {
            return;
        }

        let current_part = path_parts[0];
        let is_last = path_parts.len() == 1;

        if !nodes.contains_key(current_part) {
            if !is_last {
                *total_directories += 1;
            }

            let node = FileTreeNode {
                name: current_part.to_string(),
                path: if is_last {
                    file.path.to_string_lossy().to_string()
                } else {
                    current_part.to_string()
                },
                is_directory: !is_last,
                size: if is_last { file.size } else { 0 },
                size_formatted: if is_last {
                    utils::format_file_size(file.size)
                } else {
                    "-".to_string()
                },
                file_type: if is_last {
                    Self::get_file_type(&file.path.to_string_lossy())
                } else {
                    "folder".to_string()
                },
                icon_class: if is_last {
                    Self::get_file_icon(&file.path.to_string_lossy())
                } else {
                    "fas fa-folder".to_string()
                },
                children: Vec::new(),
                depth,
            };
            nodes.insert(current_part.to_string(), node);
        }

        if !is_last {
            let node = nodes.get_mut(current_part).unwrap();
            Self::insert_into_tree_vec(
                &mut node.children,
                &path_parts[1..],
                file,
                depth + 1,
                total_directories,
            );
        }
    }

    /// Insert file into tree structure recursively (for Vec<FileTreeNode>)
    fn insert_into_tree_vec(
        nodes: &mut Vec<FileTreeNode>,
        path_parts: &[&str],
        file: &FileEntry,
        depth: usize,
        total_directories: &mut usize,
    ) {
        if path_parts.is_empty() {
            return;
        }

        let current_part = path_parts[0];
        let is_last = path_parts.len() == 1;

        // Find existing node or create new one
        let node_index = nodes.iter().position(|n| n.name == current_part);

        if node_index.is_none() {
            if !is_last {
                *total_directories += 1;
            }

            let node = FileTreeNode {
                name: current_part.to_string(),
                path: if is_last {
                    file.path.to_string_lossy().to_string()
                } else {
                    current_part.to_string()
                },
                is_directory: !is_last,
                size: if is_last { file.size } else { 0 },
                size_formatted: if is_last {
                    utils::format_file_size(file.size)
                } else {
                    "-".to_string()
                },
                file_type: if is_last {
                    Self::get_file_type(&file.path.to_string_lossy())
                } else {
                    "folder".to_string()
                },
                icon_class: if is_last {
                    Self::get_file_icon(&file.path.to_string_lossy())
                } else {
                    "fas fa-folder".to_string()
                },
                children: Vec::new(),
                depth,
            };
            nodes.push(node);
        }

        if !is_last {
            let node_index = nodes.iter().position(|n| n.name == current_part).unwrap();
            Self::insert_into_tree_vec(
                &mut nodes[node_index].children,
                &path_parts[1..],
                file,
                depth + 1,
                total_directories,
            );
        }
    }

    /// Sort tree children recursively
    fn sort_tree_children(nodes: &mut [FileTreeNode]) {
        for node in nodes.iter_mut() {
            node.children
                .sort_by(|a, b| match (a.is_directory, b.is_directory) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                });
            Self::sort_tree_children(&mut node.children);
        }
    }

    /// Get file type from extension
    fn get_file_type(path: &str) -> String {
        if let Some(ext) = std::path::Path::new(path).extension() {
            match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                "exe" => "executable".to_string(),
                "dll" | "so" | "dylib" => "library".to_string(),
                "txt" | "md" | "readme" => "document".to_string(),
                "html" | "htm" | "css" | "js" => "web".to_string(),
                "png" | "jpg" | "jpeg" | "gif" | "ico" | "bmp" => "image".to_string(),
                "zip" | "rar" | "7z" | "tar" | "gz" => "archive".to_string(),
                "xml" | "json" | "yaml" | "yml" | "toml" => "config".to_string(),
                "py" | "rs" | "cpp" | "c" | "h" | "java" | "cs" => "source".to_string(),
                _ => "file".to_string(),
            }
        } else {
            "file".to_string()
        }
    }

    /// Get file icon class
    fn get_file_icon(path: &str) -> String {
        if let Some(ext) = std::path::Path::new(path).extension() {
            match ext.to_str().unwrap_or("").to_lowercase().as_str() {
                "exe" => "fas fa-cog text-danger".to_string(),
                "dll" | "so" | "dylib" => "fas fa-book text-primary".to_string(),
                "txt" | "md" | "readme" => "fas fa-file-alt text-secondary".to_string(),
                "html" | "htm" | "css" | "js" => "fab fa-html5 text-warning".to_string(),
                "png" | "jpg" | "jpeg" | "gif" | "ico" | "bmp" => {
                    "fas fa-image text-success".to_string()
                }
                "zip" | "rar" | "7z" | "tar" | "gz" => "fas fa-file-archive text-info".to_string(),
                "xml" | "json" | "yaml" | "yml" | "toml" => {
                    "fas fa-cogs text-secondary".to_string()
                }
                "py" | "rs" | "cpp" | "c" | "h" | "java" | "cs" => {
                    "fas fa-code text-primary".to_string()
                }
                _ => "fas fa-file text-muted".to_string(),
            }
        } else {
            "fas fa-file text-muted".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{FileAttributes, FileEntry};
    use std::path::PathBuf;

    #[test]
    fn test_file_tree_building() {
        let files = vec![
            FileEntry {
                path: PathBuf::from("app/main.exe"),
                target_path: None,
                size: 1024,
                hash: None,
                attributes: FileAttributes {
                    readonly: false,
                    hidden: false,
                    system: false,
                    executable: true,
                },
                compression: None,
            },
            FileEntry {
                path: PathBuf::from("app/config/settings.ini"),
                target_path: None,
                size: 512,
                hash: None,
                attributes: FileAttributes {
                    readonly: false,
                    hidden: false,
                    system: false,
                    executable: false,
                },
                compression: None,
            },
            FileEntry {
                path: PathBuf::from("docs/readme.txt"),
                target_path: None,
                size: 256,
                hash: None,
                attributes: FileAttributes {
                    readonly: false,
                    hidden: false,
                    system: false,
                    executable: false,
                },
                compression: None,
            },
        ];

        let tree_data = ReportTemplateData::build_file_tree(&files);

        // Should have 2 root directories: app and docs
        assert_eq!(tree_data.nodes.len(), 2);
        assert_eq!(tree_data.total_files, 3);
        assert_eq!(tree_data.total_directories, 3); // app, config, docs

        // Check app directory
        let app_node = tree_data.nodes.iter().find(|n| n.name == "app").unwrap();
        assert!(app_node.is_directory);
        assert_eq!(app_node.children.len(), 2); // main.exe and config directory

        // Check config subdirectory
        let config_node = app_node
            .children
            .iter()
            .find(|n| n.name == "config")
            .unwrap();
        assert!(config_node.is_directory);
        assert_eq!(config_node.children.len(), 1); // settings.ini

        // Check file properties
        let main_exe = app_node
            .children
            .iter()
            .find(|n| n.name == "main.exe")
            .unwrap();
        assert!(!main_exe.is_directory);
        assert_eq!(main_exe.size, 1024);
        assert_eq!(main_exe.file_type, "executable");
        assert_eq!(main_exe.icon_class, "fas fa-cog text-danger");
    }

    #[test]
    fn test_file_tree_json_serialization() {
        let files = vec![FileEntry {
            path: PathBuf::from("test.txt"),
            target_path: None,
            size: 100,
            hash: None,
            attributes: FileAttributes {
                readonly: false,
                hidden: false,
                system: false,
                executable: false,
            },
            compression: None,
        }];

        let tree_data = ReportTemplateData::build_file_tree(&files);
        let json_result = serde_json::to_string(&tree_data);

        assert!(json_result.is_ok());
        let json_str = json_result.unwrap();
        assert!(json_str.contains("test.txt"));
        assert!(json_str.contains("\"total_files\":1"));
    }
}
