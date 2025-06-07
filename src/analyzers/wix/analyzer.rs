//! WiX Toolset analyzer implementation

use crate::core::{Result, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use crate::analyzers::{InstallerAnalyzer, MsiAnalyzer, common};
use async_trait::async_trait;
use std::path::Path;

/// WiX Toolset installer analyzer
/// 
/// WiX generates MSI files with specific characteristics that can be detected
/// to distinguish them from other MSI generators.
pub struct WixAnalyzer {
    msi_analyzer: MsiAnalyzer,
}

impl WixAnalyzer {
    /// Create a new WiX analyzer
    pub fn new() -> Self {
        Self {
            msi_analyzer: MsiAnalyzer::new(),
        }
    }

    /// Check if file is a WiX-generated MSI
    async fn is_wix_msi(file_path: &Path) -> Result<bool> {
        // First check if it's an MSI file
        let msi_analyzer = MsiAnalyzer::new();
        if !msi_analyzer.can_analyze(file_path).await? {
            return Ok(false);
        }

        // Check for WiX-specific patterns
        let wix_patterns = [
            "WiX Toolset",
            "Windows Installer XML",
            "WixToolset",
            "Microsoft.Tools.WindowsInstallerXml",
            "WiX v3",
            "WiX v4",
            "WiX v5",
            "wix.exe",
            "candle.exe",
            "light.exe",
            "WixUI",
            "WixUIExtension",
            "WixUtilExtension",
            "WixNetFxExtension",
            "WixFirewallExtension",
        ];

        let matches = common::search_file_content(file_path, &wix_patterns).await?;
        Ok(!matches.is_empty())
    }

    /// Extract WiX-specific metadata
    async fn extract_wix_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Start with base MSI metadata
        let mut metadata = self.msi_analyzer.extract_metadata(file_path).await?;
        
        // Override format to WiX
        metadata.format = InstallerFormat::WiX;
        
        // Add WiX-specific properties
        let wix_properties = self.extract_wix_properties(file_path).await?;
        metadata.properties.extend(wix_properties);
        
        Ok(metadata)
    }

    /// Extract WiX-specific properties
    async fn extract_wix_properties(&self, file_path: &Path) -> Result<std::collections::HashMap<String, String>> {
        let mut properties = std::collections::HashMap::new();
        
        // Detect WiX version
        let wix_version = self.detect_wix_version(file_path).await?;
        if let Some(version) = wix_version {
            properties.insert("wix_version".to_string(), version);
        }
        
        // Detect WiX extensions
        let extensions = self.detect_wix_extensions(file_path).await?;
        if !extensions.is_empty() {
            properties.insert("wix_extensions".to_string(), extensions.join(", "));
            properties.insert("wix_extensions_count".to_string(), extensions.len().to_string());
        }
        
        // Detect WiX UI
        let ui_type = self.detect_wix_ui(file_path).await?;
        if let Some(ui) = ui_type {
            properties.insert("wix_ui_type".to_string(), ui);
        }
        
        properties.insert("generator_tool".to_string(), "WiX Toolset".to_string());
        properties.insert("msi_variant".to_string(), "WiX-generated".to_string());
        
        Ok(properties)
    }

    /// Detect WiX version
    async fn detect_wix_version(&self, file_path: &Path) -> Result<Option<String>> {
        let version_patterns = [
            ("WiX v5", "5.x"),
            ("WiX v4", "4.x"),
            ("WiX v3", "3.x"),
            ("WiX Toolset v5", "5.x"),
            ("WiX Toolset v4", "4.x"),
            ("WiX Toolset v3", "3.x"),
        ];

        for (pattern, version) in &version_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(Some(version.to_string()));
            }
        }

        // Generic WiX detection
        let generic_patterns = ["WiX Toolset", "Windows Installer XML"];
        let matches = common::search_file_content(file_path, &generic_patterns).await?;
        if !matches.is_empty() {
            return Ok(Some("Unknown".to_string()));
        }

        Ok(None)
    }

    /// Detect WiX extensions
    async fn detect_wix_extensions(&self, file_path: &Path) -> Result<Vec<String>> {
        let extension_patterns = [
            ("WixUIExtension", "UI Extension"),
            ("WixUtilExtension", "Util Extension"),
            ("WixNetFxExtension", ".NET Framework Extension"),
            ("WixFirewallExtension", "Firewall Extension"),
            ("WixIIsExtension", "IIS Extension"),
            ("WixSqlExtension", "SQL Extension"),
            ("WixVSExtension", "Visual Studio Extension"),
            ("WixGamingExtension", "Gaming Extension"),
            ("WixHttpExtension", "HTTP Extension"),
        ];

        let mut extensions = Vec::new();
        
        for (pattern, extension_name) in &extension_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                extensions.push(extension_name.to_string());
            }
        }

        Ok(extensions)
    }

    /// Detect WiX UI type
    async fn detect_wix_ui(&self, file_path: &Path) -> Result<Option<String>> {
        let ui_patterns = [
            ("WixUI_Advanced", "Advanced UI"),
            ("WixUI_FeatureTree", "Feature Tree UI"),
            ("WixUI_InstallDir", "Install Directory UI"),
            ("WixUI_Minimal", "Minimal UI"),
            ("WixUI_Mondo", "Mondo UI"),
            ("WixUI", "Custom UI"),
        ];

        for (pattern, ui_type) in &ui_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(Some(ui_type.to_string()));
            }
        }

        Ok(None)
    }
}

#[async_trait]
impl InstallerAnalyzer for WixAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Validate file accessibility
        common::validate_file(file_path).await?;

        // Check if it's a WiX-generated MSI
        Self::is_wix_msi(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::WiX
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_wix_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        // Delegate to MSI analyzer for file extraction
        self.msi_analyzer.extract_files(file_path).await
    }

    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        // Delegate to MSI analyzer for registry operations
        self.msi_analyzer.extract_registry_operations(file_path).await
    }
}

impl Default for WixAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
