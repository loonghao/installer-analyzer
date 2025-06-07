//! MSIX/AppX data structure parser

use crate::core::{Result, AnalyzerError, FileEntry};
use crate::analyzers::archive::{ArchiveParser, ArchiveFormat};
use std::path::Path;
use std::collections::HashMap;
use std::io::Read;
use zip::ZipArchive;

/// MSIX/AppX dependency information
#[derive(Debug, Clone)]
pub struct AppxDependency {
    pub name: String,
    pub publisher: Option<String>,
    pub min_version: Option<String>,
    pub max_version_tested: Option<String>,
}

/// MSIX/AppX capability information
#[derive(Debug, Clone)]
pub struct AppxCapability {
    pub name: String,
    pub capability_type: String, // "Capability" or "DeviceCapability" or "RestrictedCapability"
}

/// MSIX/AppX manifest metadata
#[derive(Debug, Clone)]
pub struct AppxManifest {
    pub identity_name: String,
    pub identity_publisher: String,
    pub identity_version: String,
    pub identity_processor_architecture: Option<String>,
    pub display_name: String,
    pub publisher_display_name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub background_color: Option<String>,
    pub min_version: Option<String>,
    pub max_version_tested: Option<String>,
    pub dependencies: Vec<AppxDependency>,
    pub capabilities: Vec<AppxCapability>,
    pub applications: Vec<String>, // Application IDs
}

/// MSIX/AppX data parser
pub struct MsixParser {
    archive_parser: ArchiveParser,
}

impl MsixParser {
    /// Create a new MSIX parser
    pub fn new() -> Self {
        Self {
            archive_parser: ArchiveParser::new(),
        }
    }

    /// Check if file is a MSIX/AppX package
    pub async fn is_msix_file(file_path: &Path) -> Result<bool> {
        // Check file extension
        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_str().unwrap_or("");
            if !matches!(ext_str.to_lowercase().as_str(), "msix" | "appx") {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        // Check if it's a ZIP file
        let format = ArchiveParser::detect_format(file_path).await?;
        Ok(format == ArchiveFormat::Zip)
    }

    /// Extract AppxManifest.xml content from MSIX/AppX package
    fn extract_manifest_content(&self, file_path: &Path) -> Result<String> {
        let file = std::fs::File::open(file_path)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| AnalyzerError::generic(format!("Failed to open MSIX/AppX file: {}", e)))?;

        // Look for AppxManifest.xml in the root
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i)
                .map_err(|e| AnalyzerError::generic(format!("Failed to read zip entry {}: {}", i, e)))?;

            let file_name = zip_file.name();
            if file_name == "AppxManifest.xml" {
                let mut content = String::new();
                zip_file.read_to_string(&mut content)
                    .map_err(|e| AnalyzerError::generic(format!("Failed to read AppxManifest.xml: {}", e)))?;
                return Ok(content);
            }
        }

        Err(AnalyzerError::generic("AppxManifest.xml not found in MSIX/AppX package"))
    }

    /// Parse AppxManifest.xml content (simplified XML parsing)
    fn parse_manifest_content(&self, content: &str) -> Result<AppxManifest> {
        // This is a simplified XML parser for demonstration
        // In a production environment, you'd want to use a proper XML parser like quick-xml
        
        let mut manifest = AppxManifest {
            identity_name: String::new(),
            identity_publisher: String::new(),
            identity_version: String::new(),
            identity_processor_architecture: None,
            display_name: String::new(),
            publisher_display_name: String::new(),
            description: None,
            logo: None,
            background_color: None,
            min_version: None,
            max_version_tested: None,
            dependencies: Vec::new(),
            capabilities: Vec::new(),
            applications: Vec::new(),
        };

        // Extract Identity information
        if let Some(identity_start) = content.find("<Identity") {
            if let Some(identity_end) = content[identity_start..].find("/>") {
                let identity_section = &content[identity_start..identity_start + identity_end];
                
                manifest.identity_name = self.extract_xml_attribute(identity_section, "Name")
                    .unwrap_or_default();
                manifest.identity_publisher = self.extract_xml_attribute(identity_section, "Publisher")
                    .unwrap_or_default();
                manifest.identity_version = self.extract_xml_attribute(identity_section, "Version")
                    .unwrap_or_default();
                manifest.identity_processor_architecture = self.extract_xml_attribute(identity_section, "ProcessorArchitecture");
            }
        }

        // Extract Properties
        if let Some(props_start) = content.find("<Properties>") {
            if let Some(props_end) = content[props_start..].find("</Properties>") {
                let props_section = &content[props_start..props_start + props_end];
                
                manifest.display_name = self.extract_xml_element_content(props_section, "DisplayName")
                    .unwrap_or_default();
                manifest.publisher_display_name = self.extract_xml_element_content(props_section, "PublisherDisplayName")
                    .unwrap_or_default();
                manifest.description = self.extract_xml_element_content(props_section, "Description");
                manifest.logo = self.extract_xml_element_content(props_section, "Logo");
            }
        }

        // Extract Dependencies (simplified)
        let mut search_pos = 0;
        while let Some(dep_start) = content[search_pos..].find("<PackageDependency") {
            let abs_start = search_pos + dep_start;
            if let Some(dep_end) = content[abs_start..].find("/>") {
                let dep_section = &content[abs_start..abs_start + dep_end];
                
                if let Some(name) = self.extract_xml_attribute(dep_section, "Name") {
                    let dependency = AppxDependency {
                        name,
                        publisher: self.extract_xml_attribute(dep_section, "Publisher"),
                        min_version: self.extract_xml_attribute(dep_section, "MinVersion"),
                        max_version_tested: self.extract_xml_attribute(dep_section, "MaxVersionTested"),
                    };
                    manifest.dependencies.push(dependency);
                }
                
                search_pos = abs_start + dep_end;
            } else {
                break;
            }
        }

        // Extract Capabilities (simplified)
        let capability_patterns = ["<Capability", "<DeviceCapability", "<RestrictedCapability"];
        
        for pattern in &capability_patterns {
            search_pos = 0;
            while let Some(cap_start) = content[search_pos..].find(pattern) {
                let abs_start = search_pos + cap_start;
                if let Some(cap_end) = content[abs_start..].find("/>") {
                    let cap_section = &content[abs_start..abs_start + cap_end];
                    
                    if let Some(name) = self.extract_xml_attribute(cap_section, "Name") {
                        let capability = AppxCapability {
                            name,
                            capability_type: pattern.trim_start_matches('<').to_string(),
                        };
                        manifest.capabilities.push(capability);
                    }
                    
                    search_pos = abs_start + cap_end;
                } else {
                    break;
                }
            }
        }

        Ok(manifest)
    }

    /// Extract XML attribute value (simplified)
    fn extract_xml_attribute(&self, xml: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start) = xml.find(&pattern) {
            let value_start = start + pattern.len();
            if let Some(end) = xml[value_start..].find('"') {
                return Some(xml[value_start..value_start + end].to_string());
            }
        }
        None
    }

    /// Extract XML element content (simplified)
    fn extract_xml_element_content(&self, xml: &str, element_name: &str) -> Option<String> {
        let start_tag = format!("<{}>", element_name);
        let end_tag = format!("</{}>", element_name);
        
        if let Some(start) = xml.find(&start_tag) {
            let content_start = start + start_tag.len();
            if let Some(end) = xml[content_start..].find(&end_tag) {
                return Some(xml[content_start..content_start + end].trim().to_string());
            }
        }
        None
    }

    /// Extract manifest from MSIX/AppX file
    pub fn extract_manifest(&self, file_path: &Path) -> Result<AppxManifest> {
        let content = self.extract_manifest_content(file_path)?;
        self.parse_manifest_content(&content)
    }

    /// Extract files from MSIX/AppX using archive parser
    pub async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        self.archive_parser.extract_files(file_path).await
    }

    /// Extract MSIX-specific metadata as HashMap
    pub async fn extract_msix_properties(&self, file_path: &Path) -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();

        // Get basic archive properties
        let archive_props = self.archive_parser.extract_metadata(file_path).await?;
        properties.extend(archive_props);

        // Get MSIX-specific metadata
        match self.extract_manifest(file_path) {
            Ok(manifest) => {
                properties.insert("msix_identity_name".to_string(), manifest.identity_name);
                properties.insert("msix_identity_publisher".to_string(), manifest.identity_publisher);
                properties.insert("msix_identity_version".to_string(), manifest.identity_version);
                properties.insert("msix_display_name".to_string(), manifest.display_name);
                properties.insert("msix_publisher_display_name".to_string(), manifest.publisher_display_name);
                
                if let Some(arch) = manifest.identity_processor_architecture {
                    properties.insert("msix_processor_architecture".to_string(), arch);
                }
                
                if let Some(description) = manifest.description {
                    properties.insert("msix_description".to_string(), description);
                }
                
                if let Some(min_version) = manifest.min_version {
                    properties.insert("msix_min_version".to_string(), min_version);
                }
                
                properties.insert("msix_dependencies_count".to_string(), manifest.dependencies.len().to_string());
                properties.insert("msix_capabilities_count".to_string(), manifest.capabilities.len().to_string());
                properties.insert("msix_applications_count".to_string(), manifest.applications.len().to_string());
            }
            Err(e) => {
                tracing::warn!("Failed to extract MSIX manifest: {}", e);
                properties.insert("msix_manifest_error".to_string(), e.to_string());
            }
        }

        properties.insert("package_type".to_string(), "MSIX/AppX Package".to_string());
        
        Ok(properties)
    }
}

impl Default for MsixParser {
    fn default() -> Self {
        Self::new()
    }
}
