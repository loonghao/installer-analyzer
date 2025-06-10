//! Report generator implementation using frontend templates

use crate::core::{AnalysisResult, AnalyzerError, Result};
use crate::reporting::templates::get_report_template;
use crate::reporting::{ReportFormat, Reporter};
use serde_json;
use std::path::Path;

/// Main report generator
pub struct ReportGenerator {}

impl ReportGenerator {
    pub fn new() -> Self {
        Self {}
    }

    /// Create unified analysis data structure (used by both HTML and JSON reports)
    fn create_unified_analysis_data(&self, result: &AnalysisResult) -> Result<serde_json::Value> {
        // Extract original filename - unified logic
        let original_filename = if let Some(source_path) = &result.source_file_path {
            source_path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Unknown File".to_string())
        } else {
            result
                .metadata
                .properties
                .get("OriginalFilename")
                .or_else(|| result.metadata.properties.get("original_filename"))
                .cloned()
                .unwrap_or_else(|| {
                    result
                        .metadata
                        .product_name
                        .as_deref()
                        .unwrap_or("Unknown Package")
                        .to_string()
                })
        };

        // Create unified data structure
        let analysis_data = serde_json::json!({
            "session_id": result.session_id,
            "analyzed_at": result.analyzed_at,
            "analysis_duration": result.analysis_duration.as_secs_f64(),
            "dynamic_analysis": result.dynamic_analysis,
            "metadata": {
                "original_filename": original_filename,
                "filename": result.metadata.product_name.as_deref().unwrap_or("Unknown Package"),
                "file_size": result.metadata.file_size,
                "file_hash": result.metadata.file_hash,
                "format": format!("{:?}", result.metadata.format),
                "version": result.metadata.product_version.as_deref().unwrap_or("N/A"),
                "publisher": result.metadata.manufacturer.as_deref().unwrap_or("N/A"),
                "description": result.metadata.properties.get("FileDescription")
                    .or_else(|| result.metadata.properties.get("description"))
                    .cloned()
                    .unwrap_or_else(|| "N/A".to_string()),
                "properties": result.metadata.properties
            },
            "files": self.create_hierarchical_file_list(&result.files),
            "registry_operations": result.registry_operations.iter().map(|op| {
                match op {
                    crate::core::RegistryOperation::CreateKey { key_path, .. } => {
                        serde_json::json!({
                            "operation": "CREATE",
                            "key": key_path,
                            "value": null
                        })
                    },
                    crate::core::RegistryOperation::SetValue { key_path, value_name, value_data, .. } => {
                        let value_str = match value_data {
                            crate::core::RegistryValue::String(s) => s.clone(),
                            crate::core::RegistryValue::DWord(d) => format!("0x{:08x}", d),
                            crate::core::RegistryValue::Binary(b) => format!("Binary ({} bytes)", b.len()),
                            _ => "Complex Value".to_string(),
                        };
                        serde_json::json!({
                            "operation": "SET",
                            "key": format!("{}\\{}", key_path, value_name),
                            "value": value_str
                        })
                    },
                    crate::core::RegistryOperation::DeleteKey { key_path, .. } => {
                        serde_json::json!({
                            "operation": "DELETE",
                            "key": key_path,
                            "value": null
                        })
                    },
                    crate::core::RegistryOperation::DeleteValue { key_path, value_name, .. } => {
                        serde_json::json!({
                            "operation": "DELETE_VALUE",
                            "key": format!("{}\\{}", key_path, value_name),
                            "value": null
                        })
                    }
                }
            }).collect::<Vec<_>>(),
            "file_operations": result.file_operations,
            "process_operations": result.process_operations,
            "network_operations": result.network_operations,
            "summary": {
                "total_files": result.files.len(),
                "executable_files": result.files.iter().filter(|f| f.attributes.executable).count(),
                "registry_operations": result.registry_operations.len(),
                "file_operations": result.file_operations.len(),
                "process_operations": result.process_operations.len(),
                "network_operations": result.network_operations.len()
            }
        });

        Ok(analysis_data)
    }

    /// Generate JSON report using unified data structure
    async fn generate_json_report(&self, result: &AnalysisResult) -> Result<String> {
        let analysis_data = self.create_unified_analysis_data(result)?;

        // For JSON output, we can either return the raw analysis data or wrap it
        // Let's return the unified data structure directly for consistency
        serde_json::to_string_pretty(&analysis_data).map_err(AnalyzerError::SerializationError)
    }

    /// Generate modern HTML report using frontend template with data injection
    async fn generate_html_report(&self, result: &AnalysisResult) -> Result<String> {
        // Get the base HTML template
        let template_html = get_report_template();

        // Use unified analysis data structure
        let analysis_data = self.create_unified_analysis_data(result)?;

        // Inject the data into the HTML template
        let data_script = format!(
            "<script>window.ANALYSIS_DATA = {};</script>",
            serde_json::to_string(&analysis_data).map_err(AnalyzerError::SerializationError)?
        );

        // Insert the data script before the closing </head> tag
        let html_with_data = template_html.replace("</head>", &format!("{}\n</head>", data_script));

        Ok(html_with_data)
    }

    /// Get file type for frontend display
    fn get_file_type(&self, path: &str, is_executable: bool) -> &'static str {
        if is_executable {
            return "executable";
        }

        if let Some(ext) = path.split('.').next_back() {
            match ext.to_lowercase().as_str() {
                "dll" | "so" | "dylib" => "library",
                "txt" | "md" | "readme" => "text",
                "ini" | "cfg" | "conf" => "config",
                "sys" => "driver",
                _ => "file",
            }
        } else {
            "file"
        }
    }

    /// Get file icon class for frontend display
    fn get_file_icon(&self, path: &str, is_directory: bool) -> &'static str {
        if is_directory {
            return "fas fa-folder";
        }

        if let Some(ext) = path.split('.').next_back() {
            match ext.to_lowercase().as_str() {
                "exe" | "msi" | "dmg" => "fas fa-cog",
                "dll" | "so" | "dylib" => "fas fa-puzzle-piece",
                "txt" | "md" | "readme" => "fas fa-file-alt",
                "pdf" => "fas fa-file-pdf",
                "jpg" | "jpeg" | "png" | "gif" => "fas fa-file-image",
                "mp3" | "wav" | "ogg" => "fas fa-file-audio",
                "mp4" | "avi" | "mov" => "fas fa-file-video",
                "zip" | "rar" | "7z" => "fas fa-file-archive",
                "js" | "ts" | "py" | "java" | "cpp" => "fas fa-file-code",
                "sys" => "fas fa-microchip",
                _ => "fas fa-file",
            }
        } else {
            "fas fa-file"
        }
    }

    /// Create hierarchical file list with proper directory structure for frontend
    fn create_hierarchical_file_list(&self, files: &[crate::core::FileEntry]) -> serde_json::Value {
        use std::collections::HashMap;

        let mut all_files = Vec::new();
        let mut directories = HashMap::new();

        // First, collect all unique directory paths
        for file in files {
            let path_str = file.path.to_string_lossy();
            let path_parts: Vec<&str> = path_str.split('/').collect();

            // Create directory entries for each level
            for i in 1..path_parts.len() {
                let dir_path = path_parts[0..i].join("/");
                if !directories.contains_key(&dir_path) {
                    directories.insert(dir_path.clone(), true);
                }
            }
        }

        // Add directory entries
        for dir_path in directories.keys() {
            all_files.push(serde_json::json!({
                "path": dir_path,
                "size": 0,
                "type": "folder",
                "is_directory": true,
                "icon_class": "fas fa-folder",
                "attributes": {
                    "readonly": false,
                    "hidden": false,
                    "system": false,
                    "executable": false
                },
                "hash": null,
                "target_path": null,
                "compression": null
            }));
        }

        // Add file entries
        for file in files {
            let path_str = file.path.to_string_lossy();
            let is_directory = file.path.is_dir();

            all_files.push(serde_json::json!({
                "path": path_str,
                "size": file.size,
                "type": self.get_file_type(&path_str, file.attributes.executable),
                "is_directory": is_directory,
                "icon_class": self.get_file_icon(&path_str, is_directory),
                "attributes": {
                    "readonly": file.attributes.readonly,
                    "hidden": file.attributes.hidden,
                    "system": file.attributes.system,
                    "executable": file.attributes.executable
                },
                "hash": file.hash,
                "target_path": file.target_path.as_ref().map(|p| p.to_string_lossy().to_string()),
                "compression": file.compression
            }));
        }

        serde_json::Value::Array(all_files)
    }

    /// Generate Markdown report
    async fn generate_markdown_report(&self, result: &AnalysisResult) -> Result<String> {
        let markdown = format!(
            r#"# Installer Analysis Report

**Session ID:** {}
**Analyzed at:** {}
**Analysis Duration:** {}
**Dynamic Analysis:** {}

## Installer Metadata

- **Format:** {:?}
- **Product Name:** {}
- **Product Version:** {}
- **Manufacturer:** {}
- **File Size:** {}
- **File Hash:** {}

## Summary

- **Files:** {} entries
- **Registry Operations:** {} operations
- **File Operations:** {} operations
- **Process Operations:** {} operations
- **Network Operations:** {} operations

## Security Analysis

- **Risk Level:** {}
- **Executable Files:** {}
- **Large Files (>50MB):** {}

## File Analysis

### Top 10 Largest Files

{}

### Executable Files

{}

## Registry Operations

{}

---

*Report generated by Installer Analyzer v{} at {}*
"#,
            result.session_id,
            result.analyzed_at.format("%Y-%m-%d %H:%M:%S UTC"),
            crate::utils::format_duration(result.analysis_duration),
            if result.dynamic_analysis { "Yes" } else { "No" },
            result.metadata.format,
            result.metadata.product_name.as_deref().unwrap_or("Unknown"),
            result
                .metadata
                .product_version
                .as_deref()
                .unwrap_or("Unknown"),
            result.metadata.manufacturer.as_deref().unwrap_or("Unknown"),
            crate::utils::format_file_size(result.metadata.file_size),
            &result.metadata.file_hash[..16],
            result.files.len(),
            result.registry_operations.len(),
            result.file_operations.len(),
            result.process_operations.len(),
            result.network_operations.len(),
            self.calculate_risk_level(result),
            result
                .files
                .iter()
                .filter(|f| f.attributes.executable)
                .count(),
            result
                .files
                .iter()
                .filter(|f| f.size > 50 * 1024 * 1024)
                .count(),
            self.generate_top_files_markdown(&result.files),
            self.generate_executable_files_markdown(&result.files),
            self.generate_registry_operations_markdown(&result.registry_operations),
            env!("CARGO_PKG_VERSION"),
            result.analyzed_at.format("%Y-%m-%d %H:%M:%S UTC")
        );

        Ok(markdown)
    }

    /// Calculate risk level for CI/CD
    fn calculate_risk_level(&self, result: &AnalysisResult) -> String {
        let executable_count = result
            .files
            .iter()
            .filter(|f| f.attributes.executable)
            .count();
        let large_files = result
            .files
            .iter()
            .filter(|f| f.size > 50 * 1024 * 1024)
            .count();

        if executable_count > 10 || large_files > 5 {
            "high".to_string()
        } else if executable_count > 5 || large_files > 2 {
            "medium".to_string()
        } else {
            "low".to_string()
        }
    }

    /// Generate top files section for markdown
    fn generate_top_files_markdown(&self, files: &[crate::core::FileEntry]) -> String {
        let mut sorted_files: Vec<_> = files.iter().collect();
        sorted_files.sort_by(|a, b| b.size.cmp(&a.size));

        let mut markdown = String::new();
        for (i, file) in sorted_files.iter().take(10).enumerate() {
            markdown.push_str(&format!(
                "{}. **{}** - {}\n",
                i + 1,
                file.path.file_name().unwrap_or_default().to_string_lossy(),
                crate::utils::format_file_size(file.size)
            ));
        }

        if markdown.is_empty() {
            markdown.push_str("No files found.\n");
        }

        markdown
    }

    /// Generate executable files section for markdown
    fn generate_executable_files_markdown(&self, files: &[crate::core::FileEntry]) -> String {
        let executable_files: Vec<_> = files.iter().filter(|f| f.attributes.executable).collect();

        let mut markdown = String::new();
        for file in executable_files.iter().take(20) {
            markdown.push_str(&format!(
                "- **{}** - {}\n",
                file.path.file_name().unwrap_or_default().to_string_lossy(),
                crate::utils::format_file_size(file.size)
            ));
        }

        if markdown.is_empty() {
            markdown.push_str("No executable files found.\n");
        }

        markdown
    }

    /// Generate registry operations section for markdown
    fn generate_registry_operations_markdown(
        &self,
        operations: &[crate::core::RegistryOperation],
    ) -> String {
        let mut markdown = String::new();

        for (i, op) in operations.iter().take(20).enumerate() {
            match op {
                crate::core::RegistryOperation::CreateKey { key_path, .. } => {
                    markdown.push_str(&format!("{}. **Create Key:** `{}`\n", i + 1, key_path));
                }
                crate::core::RegistryOperation::SetValue {
                    key_path,
                    value_name,
                    ..
                } => {
                    markdown.push_str(&format!(
                        "{}. **Set Value:** `{}\\{}`\n",
                        i + 1,
                        key_path,
                        value_name
                    ));
                }
                crate::core::RegistryOperation::DeleteKey { key_path, .. } => {
                    markdown.push_str(&format!("{}. **Delete Key:** `{}`\n", i + 1, key_path));
                }
                crate::core::RegistryOperation::DeleteValue {
                    key_path,
                    value_name,
                    ..
                } => {
                    markdown.push_str(&format!(
                        "{}. **Delete Value:** `{}\\{}`\n",
                        i + 1,
                        key_path,
                        value_name
                    ));
                }
            }
        }

        if markdown.is_empty() {
            markdown.push_str("No registry operations found.\n");
        }

        markdown
    }
}

impl Reporter for ReportGenerator {
    async fn generate_report(
        &self,
        result: &AnalysisResult,
        format: ReportFormat,
    ) -> Result<String> {
        match format {
            ReportFormat::Json => self.generate_json_report(result).await,
            ReportFormat::Html => self.generate_html_report(result).await,
            ReportFormat::Markdown => self.generate_markdown_report(result).await,
        }
    }

    async fn save_report(
        &self,
        result: &AnalysisResult,
        format: ReportFormat,
        output_path: &Path,
    ) -> Result<()> {
        let content = self.generate_report(result, format).await?;
        tokio::fs::write(output_path, content).await?;
        tracing::info!("Report saved to: {}", output_path.display());
        Ok(())
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}
