use crate::core::error::Result;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

/// Enhanced metadata extractor that combines multiple sources
pub struct MetadataExtractor;

impl MetadataExtractor {
    /// Extract metadata from multiple sources with fallback strategy
    pub async fn extract_enhanced_metadata(
        file_path: &Path,
        existing_metadata: Option<HashMap<String, String>>,
    ) -> Result<EnhancedMetadata> {
        let mut metadata = EnhancedMetadata::default();

        // 1. Use existing metadata if available
        if let Some(existing) = existing_metadata {
            metadata.apply_existing_metadata(existing);
        }

        // 2. Extract from filename if metadata is still incomplete
        if metadata.is_incomplete() {
            metadata.apply_filename_metadata(file_path);
        }

        // 3. Try to extract from PE version info (for Windows executables)
        if metadata.is_incomplete() && Self::is_pe_file(file_path).await? {
            if let Ok(pe_metadata) = Self::extract_pe_version_info(file_path).await {
                metadata.apply_pe_metadata(pe_metadata);
            }
        }

        // 4. Apply smart defaults
        metadata.apply_smart_defaults();

        Ok(metadata)
    }

    /// Check if file is a PE (Portable Executable) file
    async fn is_pe_file(file_path: &Path) -> Result<bool> {
        let header = super::read_file_header(file_path, 64).await?;

        // Check for DOS header "MZ"
        if header.len() >= 2 && &header[0..2] == b"MZ" {
            // Check for PE signature at offset specified in DOS header
            if header.len() >= 64 {
                let pe_offset =
                    u32::from_le_bytes([header[60], header[61], header[62], header[63]]) as usize;
                if pe_offset < header.len() - 4 {
                    return Ok(&header[pe_offset..pe_offset + 4] == b"PE\0\0");
                }
            }
        }
        Ok(false)
    }

    /// Extract version information from PE file resources
    async fn extract_pe_version_info(file_path: &Path) -> Result<HashMap<String, String>> {
        let mut info = HashMap::new();

        // Read a larger portion of the file to find version info
        let content = super::read_file_content_range(file_path, 0, 2 * 1024 * 1024).await?; // 2MB

        // Convert to string, handling potential encoding issues
        let content_str = String::from_utf8_lossy(&content);

        // Look for common version info patterns
        Self::extract_version_patterns(&content_str, &mut info);

        Ok(info)
    }

    /// Extract version information using pattern matching
    fn extract_version_patterns(content: &str, info: &mut HashMap<String, String>) {
        // Common version info fields
        let patterns = [
            ("ProductName", r"ProductName[^\x00]*?\x00([^\x00]+)"),
            ("ProductVersion", r"ProductVersion[^\x00]*?\x00([^\x00]+)"),
            ("FileVersion", r"FileVersion[^\x00]*?\x00([^\x00]+)"),
            ("CompanyName", r"CompanyName[^\x00]*?\x00([^\x00]+)"),
            ("FileDescription", r"FileDescription[^\x00]*?\x00([^\x00]+)"),
            ("InternalName", r"InternalName[^\x00]*?\x00([^\x00]+)"),
            (
                "OriginalFilename",
                r"OriginalFilename[^\x00]*?\x00([^\x00]+)",
            ),
            ("LegalCopyright", r"LegalCopyright[^\x00]*?\x00([^\x00]+)"),
        ];

        for (key, pattern) in &patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(content) {
                    if let Some(value) = captures.get(1) {
                        let value_str = value.as_str().trim();
                        if !value_str.is_empty() && value_str.len() < 256 {
                            info.insert(key.to_string(), value_str.to_string());
                        }
                    }
                }
            }
        }

        // Also try simpler patterns for version numbers
        if !info.contains_key("ProductVersion") && !info.contains_key("FileVersion") {
            let version_patterns = [
                r"\b(\d+\.\d+\.\d+\.\d+)\b",
                r"\b(\d+\.\d+\.\d+)\b",
                r"\bv(\d+\.\d+)\b",
                r"\bversion\s+(\d+\.\d+)\b",
            ];

            for pattern in &version_patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if let Some(captures) = regex.captures(content) {
                        if let Some(version) = captures.get(1) {
                            info.insert(
                                "ExtractedVersion".to_string(),
                                version.as_str().to_string(),
                            );
                            break;
                        }
                    }
                }
            }
        }
    }
}

/// Enhanced metadata structure
#[derive(Debug, Default)]
pub struct EnhancedMetadata {
    pub product_name: Option<String>,
    pub product_version: Option<String>,
    pub manufacturer: Option<String>,
    pub file_description: Option<String>,
    pub internal_name: Option<String>,
    pub original_filename: Option<String>,
    pub legal_copyright: Option<String>,
    pub confidence_score: f32,
}

impl EnhancedMetadata {
    /// Apply existing metadata from analyzers
    pub fn apply_existing_metadata(&mut self, metadata: HashMap<String, String>) {
        if let Some(name) = metadata.get("ProductName") {
            if name != "Unknown Application" && name != "Unknown" {
                self.product_name = Some(name.clone());
                self.confidence_score += 0.4;
            }
        }

        if let Some(version) = metadata
            .get("ProductVersion")
            .or_else(|| metadata.get("FileVersion"))
        {
            if version != "1.0.0" && version != "1.0.0.0" {
                self.product_version = Some(version.clone());
                self.confidence_score += 0.3;
            }
        }

        if let Some(company) = metadata.get("CompanyName") {
            if company != "Unknown Publisher" && company != "Unknown" {
                self.manufacturer = Some(company.clone());
                self.confidence_score += 0.2;
            }
        }

        // Apply other fields
        self.file_description = metadata.get("FileDescription").cloned();
        self.internal_name = metadata.get("InternalName").cloned();
        self.original_filename = metadata.get("OriginalFilename").cloned();
        self.legal_copyright = metadata.get("LegalCopyright").cloned();
    }

    /// Apply metadata extracted from PE version info
    pub fn apply_pe_metadata(&mut self, pe_info: HashMap<String, String>) {
        if self.product_name.is_none() {
            if let Some(name) = pe_info
                .get("ProductName")
                .or_else(|| pe_info.get("FileDescription"))
            {
                if !name.contains("Installer") && !name.contains("Setup") {
                    self.product_name = Some(name.clone());
                    self.confidence_score += 0.3;
                }
            }
        }

        if self.product_version.is_none() {
            if let Some(version) = pe_info
                .get("ProductVersion")
                .or_else(|| pe_info.get("FileVersion"))
                .or_else(|| pe_info.get("ExtractedVersion"))
            {
                self.product_version = Some(version.clone());
                self.confidence_score += 0.2;
            }
        }

        if self.manufacturer.is_none() {
            if let Some(company) = pe_info.get("CompanyName") {
                self.manufacturer = Some(company.clone());
                self.confidence_score += 0.2;
            }
        }

        // Apply other PE fields
        if self.file_description.is_none() {
            self.file_description = pe_info.get("FileDescription").cloned();
        }
        if self.internal_name.is_none() {
            self.internal_name = pe_info.get("InternalName").cloned();
        }
        if self.original_filename.is_none() {
            self.original_filename = pe_info.get("OriginalFilename").cloned();
        }
        if self.legal_copyright.is_none() {
            self.legal_copyright = pe_info.get("LegalCopyright").cloned();
        }
    }

    /// Apply metadata extracted from filename
    pub fn apply_filename_metadata(&mut self, file_path: &Path) {
        if let Some(filename) = file_path.file_stem().and_then(|s| s.to_str()) {
            let parsed = FilenameParser::parse(filename);

            if self.product_name.is_none() && parsed.product_name.is_some() {
                self.product_name = parsed.product_name;
                self.confidence_score += 0.2;
            }

            if self.product_version.is_none() && parsed.version.is_some() {
                self.product_version = parsed.version;
                self.confidence_score += 0.1;
            }

            if self.manufacturer.is_none() && parsed.company.is_some() {
                self.manufacturer = parsed.company;
                self.confidence_score += 0.1;
            }
        }
    }

    /// Apply smart defaults when metadata is missing
    pub fn apply_smart_defaults(&mut self) {
        if self.product_name.is_none() {
            self.product_name = Some("Unknown Application".to_string());
        }

        if self.product_version.is_none() {
            self.product_version = Some("Unknown".to_string());
        }

        if self.manufacturer.is_none() {
            self.manufacturer = Some("Unknown Publisher".to_string());
        }
    }

    /// Check if metadata is incomplete
    pub fn is_incomplete(&self) -> bool {
        self.product_name.is_none()
            || self.product_version.is_none()
            || self.manufacturer.is_none()
            || self.confidence_score < 0.5
    }
}

/// Filename parser for extracting metadata from filenames
pub struct FilenameParser;

impl FilenameParser {
    /// Parse filename to extract product information
    pub fn parse(filename: &str) -> ParsedFilename {
        let mut result = ParsedFilename::default();

        // Remove file extension first
        let name_without_ext = filename
            .strip_suffix(".exe")
            .unwrap_or(filename)
            .strip_suffix(".msi")
            .unwrap_or(filename)
            .strip_suffix(".zip")
            .unwrap_or(filename)
            .strip_suffix(".whl")
            .unwrap_or(filename);

        // Try to extract version using regex (before cleaning)
        let version_patterns = [
            r"(\d+\.\d+\.\d+\.\d+)", // 1.2.3.4
            r"(\d+\.\d+\.\d+)",      // 1.2.3
            r"v(\d+\.\d+\.\d+)",     // v1.2.3
            r"v(\d+\.\d+)",          // v1.2
            r"(\d+\.\d+)",           // 1.2 (last resort)
        ];

        let mut version_found = false;
        let mut product_name_base = name_without_ext.to_string();

        for pattern in &version_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(name_without_ext) {
                    if let Some(version_match) = captures.get(1) {
                        result.version = Some(version_match.as_str().to_string());
                        // Remove version from product name
                        product_name_base = regex.replace(name_without_ext, "").to_string();
                        version_found = true;
                        break;
                    }
                }
            }
        }

        // Clean up common patterns from product name
        let cleaned = product_name_base
            .replace("Setup", "")
            .replace("setup", "")
            .replace("Installer", "")
            .replace("installer", "")
            .replace("-portable", "")
            .replace("-x64", "")
            .replace("-x86", "")
            .replace("_x64", "")
            .replace("_x86", "")
            .replace("-win32", "")
            .replace("-win64", "")
            .replace("_", " ")
            .replace("-", " ");

        // Clean up multiple spaces and trim, but preserve dots in reasonable places
        let final_name = cleaned
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join(" ")
            .trim()
            .to_string();

        if !final_name.is_empty() {
            result.product_name = Some(final_name);
        } else if !version_found {
            // If no version found and name is empty after cleaning, use original
            result.product_name = Some(name_without_ext.to_string());
        }

        // Try to detect architecture
        if filename.contains("x64") || filename.contains("64") {
            result.architecture = Some("x64".to_string());
        } else if filename.contains("x86") || filename.contains("32") {
            result.architecture = Some("x86".to_string());
        }

        result
    }
}

/// Parsed filename information
#[derive(Debug, Default)]
pub struct ParsedFilename {
    pub product_name: Option<String>,
    pub version: Option<String>,
    pub company: Option<String>,
    pub architecture: Option<String>,
}
