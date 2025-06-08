//! Python Wheel data structure parser

use crate::analyzers::archive::{ArchiveFormat, ArchiveParser};
use crate::core::{AnalyzerError, FileEntry, Result};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

/// Python Wheel dependency information
#[derive(Debug, Clone)]
pub struct WheelDependency {
    pub name: String,
    pub version_spec: Option<String>,
    pub extras: Vec<String>,
    pub environment_marker: Option<String>,
}

/// Python Wheel metadata
#[derive(Debug, Clone)]
pub struct WheelMetadata {
    pub name: String,
    pub version: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub maintainer: Option<String>,
    pub maintainer_email: Option<String>,
    pub license: Option<String>,
    pub home_page: Option<String>,
    pub download_url: Option<String>,
    pub platform: Vec<String>,
    pub classifier: Vec<String>,
    pub requires_dist: Vec<WheelDependency>,
    pub requires_python: Option<String>,
    pub provides_extra: Vec<String>,
}

/// Python Wheel data parser
pub struct WheelParser {
    archive_parser: ArchiveParser,
}

impl WheelParser {
    /// Create a new wheel parser
    pub fn new() -> Self {
        Self {
            archive_parser: ArchiveParser::new(),
        }
    }

    /// Check if file is a Python wheel
    pub async fn is_wheel_file(file_path: &Path) -> Result<bool> {
        // Check file extension
        if let Some(ext) = file_path.extension() {
            if ext.to_str() != Some("whl") {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        // Check if it's a ZIP file
        let format = ArchiveParser::detect_format(file_path).await?;
        Ok(format == ArchiveFormat::Zip)
    }

    /// Extract METADATA file content from wheel
    fn extract_metadata_content(&self, file_path: &Path) -> Result<String> {
        let file = std::fs::File::open(file_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| AnalyzerError::generic(format!("Failed to open wheel file: {}", e)))?;

        // Look for METADATA file in .dist-info directory
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).map_err(|e| {
                AnalyzerError::generic(format!("Failed to read zip entry {}: {}", i, e))
            })?;

            let file_name = zip_file.name();
            if file_name.ends_with(".dist-info/METADATA") {
                let mut content = String::new();
                zip_file.read_to_string(&mut content).map_err(|e| {
                    AnalyzerError::generic(format!("Failed to read METADATA file: {}", e))
                })?;
                return Ok(content);
            }
        }

        Err(AnalyzerError::generic("METADATA file not found in wheel"))
    }

    /// Parse METADATA file content
    fn parse_metadata_content(&self, content: &str) -> Result<WheelMetadata> {
        let mut metadata = WheelMetadata {
            name: String::new(),
            version: String::new(),
            summary: None,
            description: None,
            author: None,
            author_email: None,
            maintainer: None,
            maintainer_email: None,
            license: None,
            home_page: None,
            download_url: None,
            platform: Vec::new(),
            classifier: Vec::new(),
            requires_dist: Vec::new(),
            requires_python: None,
            provides_extra: Vec::new(),
        };

        let mut in_description = false;
        let mut description_lines = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // Handle description section (after empty line)
            if in_description {
                description_lines.push(line.to_string());
                continue;
            }

            // Check for start of description
            if line.is_empty() && !metadata.name.is_empty() {
                in_description = true;
                continue;
            }

            // Parse metadata fields
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "Name" => metadata.name = value.to_string(),
                    "Version" => metadata.version = value.to_string(),
                    "Summary" => metadata.summary = Some(value.to_string()),
                    "Author" => metadata.author = Some(value.to_string()),
                    "Author-email" => metadata.author_email = Some(value.to_string()),
                    "Maintainer" => metadata.maintainer = Some(value.to_string()),
                    "Maintainer-email" => metadata.maintainer_email = Some(value.to_string()),
                    "License" => metadata.license = Some(value.to_string()),
                    "Home-page" => metadata.home_page = Some(value.to_string()),
                    "Download-URL" => metadata.download_url = Some(value.to_string()),
                    "Platform" => metadata.platform.push(value.to_string()),
                    "Classifier" => metadata.classifier.push(value.to_string()),
                    "Requires-Python" => metadata.requires_python = Some(value.to_string()),
                    "Provides-Extra" => metadata.provides_extra.push(value.to_string()),
                    "Requires-Dist" => {
                        if let Ok(dep) = self.parse_dependency(value) {
                            metadata.requires_dist.push(dep);
                        }
                    }
                    _ => {} // Ignore unknown fields
                }
            }
        }

        // Set description if we found any
        if !description_lines.is_empty() {
            metadata.description = Some(description_lines.join("\n"));
        }

        Ok(metadata)
    }

    /// Parse a dependency specification
    fn parse_dependency(&self, spec: &str) -> Result<WheelDependency> {
        // Simple dependency parsing - real implementation would be more complex
        let spec = spec.trim();

        // Split on semicolon for environment markers
        let (dep_part, env_marker) = if let Some(pos) = spec.find(';') {
            (spec[..pos].trim(), Some(spec[pos + 1..].trim().to_string()))
        } else {
            (spec, None)
        };

        // Extract package name and version spec
        let (name, version_spec) = if let Some(pos) = dep_part.find(|c: char| ">=<=!~".contains(c))
        {
            (
                dep_part[..pos].trim().to_string(),
                Some(dep_part[pos..].trim().to_string()),
            )
        } else {
            (dep_part.to_string(), None)
        };

        Ok(WheelDependency {
            name,
            version_spec,
            extras: Vec::new(), // TODO: Parse extras
            environment_marker: env_marker,
        })
    }

    /// Extract metadata from wheel file
    pub fn extract_metadata(&self, file_path: &Path) -> Result<WheelMetadata> {
        let content = self.extract_metadata_content(file_path)?;
        self.parse_metadata_content(&content)
    }

    /// Extract files from wheel using archive parser
    pub async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        self.archive_parser.extract_files(file_path).await
    }

    /// Extract wheel-specific metadata as HashMap
    pub async fn extract_wheel_properties(
        &self,
        file_path: &Path,
    ) -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();

        // Get basic archive properties
        let archive_props = self.archive_parser.extract_metadata(file_path).await?;
        properties.extend(archive_props);

        // Get wheel-specific metadata
        match self.extract_metadata(file_path) {
            Ok(metadata) => {
                properties.insert("wheel_name".to_string(), metadata.name);
                properties.insert("wheel_version".to_string(), metadata.version);

                if let Some(summary) = metadata.summary {
                    properties.insert("wheel_summary".to_string(), summary);
                }

                if let Some(author) = metadata.author {
                    properties.insert("wheel_author".to_string(), author);
                }

                if let Some(license) = metadata.license {
                    properties.insert("wheel_license".to_string(), license);
                }

                if let Some(requires_python) = metadata.requires_python {
                    properties.insert("wheel_requires_python".to_string(), requires_python);
                }

                properties.insert(
                    "wheel_dependencies_count".to_string(),
                    metadata.requires_dist.len().to_string(),
                );
                properties.insert(
                    "wheel_classifiers_count".to_string(),
                    metadata.classifier.len().to_string(),
                );
            }
            Err(e) => {
                tracing::warn!("Failed to extract wheel metadata: {}", e);
                properties.insert("wheel_metadata_error".to_string(), e.to_string());
            }
        }

        properties.insert("package_type".to_string(), "Python Wheel".to_string());

        Ok(properties)
    }
}

impl Default for WheelParser {
    fn default() -> Self {
        Self::new()
    }
}
