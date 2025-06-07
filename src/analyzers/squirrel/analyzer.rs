//! Squirrel analyzer implementation

use crate::core::{Result, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use crate::analyzers::{InstallerAnalyzer, NsisAnalyzer, common};
use async_trait::async_trait;
use std::path::Path;
use chrono::Utc;

/// Squirrel installer analyzer
/// 
/// Squirrel is used by Electron applications for auto-updating.
/// It typically uses NSIS as the underlying installer technology.
pub struct SquirrelAnalyzer {
    nsis_analyzer: NsisAnalyzer,
}

impl SquirrelAnalyzer {
    /// Create a new Squirrel analyzer
    pub fn new() -> Self {
        Self {
            nsis_analyzer: NsisAnalyzer::new(),
        }
    }

    /// Check if file is a Squirrel installer
    async fn is_squirrel_installer(file_path: &Path) -> Result<bool> {
        // First check if it's an NSIS file
        let nsis_analyzer = NsisAnalyzer::new();
        if !nsis_analyzer.can_analyze(file_path).await? {
            return Ok(false);
        }

        // Check for Squirrel-specific patterns
        let squirrel_patterns = [
            "Squirrel",
            "electron-builder",
            "electron-updater",
            "Update.exe",
            "SquirrelSetup",
            "app-update.yml",
            "latest.yml",
            "RELEASES",
            "nupkg",
            "Electron",
            "electron.exe",
            "resources\\app.asar",
            "resources/app.asar",
            "autoUpdater",
            "checkForUpdates",
            "quitAndInstall",
            "GitHub\\SquirrelTemp",
            "GitHub/SquirrelTemp",
        ];

        let matches = common::search_file_content(file_path, &squirrel_patterns).await?;
        Ok(!matches.is_empty())
    }

    /// Extract Squirrel-specific metadata
    async fn extract_squirrel_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Start with base NSIS metadata
        let mut metadata = self.nsis_analyzer.extract_metadata(file_path).await?;
        
        // Override format to Squirrel
        metadata.format = InstallerFormat::Squirrel;
        
        // Add Squirrel-specific properties
        let squirrel_properties = self.extract_squirrel_properties(file_path).await?;
        metadata.properties.extend(squirrel_properties);
        
        Ok(metadata)
    }

    /// Extract Squirrel-specific properties
    async fn extract_squirrel_properties(&self, file_path: &Path) -> Result<std::collections::HashMap<String, String>> {
        let mut properties = std::collections::HashMap::new();
        
        // Detect Electron version
        let electron_version = self.detect_electron_version(file_path).await?;
        if let Some(version) = electron_version {
            properties.insert("electron_version".to_string(), version);
        }
        
        // Detect Squirrel version/type
        let squirrel_type = self.detect_squirrel_type(file_path).await?;
        if let Some(sq_type) = squirrel_type {
            properties.insert("squirrel_type".to_string(), sq_type);
        }
        
        // Detect update mechanism
        let update_mechanism = self.detect_update_mechanism(file_path).await?;
        if let Some(mechanism) = update_mechanism {
            properties.insert("update_mechanism".to_string(), mechanism);
        }
        
        // Detect app framework
        let framework = self.detect_app_framework(file_path).await?;
        if let Some(fw) = framework {
            properties.insert("app_framework".to_string(), fw);
        }
        
        properties.insert("installer_type".to_string(), "Squirrel".to_string());
        properties.insert("base_technology".to_string(), "NSIS".to_string());
        properties.insert("app_type".to_string(), "Electron Application".to_string());
        
        Ok(properties)
    }

    /// Detect Electron version
    async fn detect_electron_version(&self, file_path: &Path) -> Result<Option<String>> {
        let version_patterns = [
            "Electron/",
            "electron-v",
            "electron@",
            "\"electron\":",
        ];

        // This is a simplified detection - in reality, you'd need to parse
        // the package.json or version info from the embedded resources
        for pattern in &version_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                // Try to extract version number (simplified)
                return Ok(Some("Unknown".to_string()));
            }
        }

        Ok(None)
    }

    /// Detect Squirrel type
    async fn detect_squirrel_type(&self, file_path: &Path) -> Result<Option<String>> {
        let type_patterns = [
            ("Squirrel.Windows", "Squirrel.Windows"),
            ("electron-builder", "electron-builder"),
            ("electron-updater", "electron-updater"),
            ("autoUpdater", "Electron autoUpdater"),
        ];

        for (pattern, squirrel_type) in &type_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(Some(squirrel_type.to_string()));
            }
        }

        Ok(Some("Generic Squirrel".to_string()))
    }

    /// Detect update mechanism
    async fn detect_update_mechanism(&self, file_path: &Path) -> Result<Option<String>> {
        let mechanism_patterns = [
            ("checkForUpdates", "Auto-update"),
            ("quitAndInstall", "Auto-update"),
            ("app-update.yml", "YAML-based updates"),
            ("latest.yml", "YAML-based updates"),
            ("RELEASES", "GitHub Releases"),
            ("nupkg", "NuGet packages"),
        ];

        for (pattern, mechanism) in &mechanism_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(Some(mechanism.to_string()));
            }
        }

        Ok(None)
    }

    /// Detect application framework
    async fn detect_app_framework(&self, file_path: &Path) -> Result<Option<String>> {
        let framework_patterns = [
            ("React", "React"),
            ("Vue", "Vue.js"),
            ("Angular", "Angular"),
            ("Svelte", "Svelte"),
            ("app.asar", "Electron"),
            ("node_modules", "Node.js"),
        ];

        for (pattern, framework) in &framework_patterns {
            let matches = common::search_file_content(file_path, &[pattern]).await?;
            if !matches.is_empty() {
                return Ok(Some(framework.to_string()));
            }
        }

        Ok(Some("Electron".to_string()))
    }

    /// Extract Squirrel-specific files
    async fn extract_squirrel_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Start with NSIS file extraction
        let mut files = self.nsis_analyzer.extract_files(file_path).await?;
        
        // Add common Squirrel/Electron files that might be present
        let squirrel_files = [
            ("Update.exe", 1024 * 1024, true),
            ("app.asar", 10 * 1024 * 1024, false),
            ("electron.exe", 100 * 1024 * 1024, true),
            ("resources/app.asar", 10 * 1024 * 1024, false),
            ("locales/en-US.pak", 1024 * 1024, false),
            ("version", 1024, false),
            ("LICENSE", 2048, false),
            ("LICENSES.chromium.html", 100 * 1024, false),
        ];

        for (filename, size, executable) in &squirrel_files {
            files.push(FileEntry {
                path: std::path::PathBuf::from(filename),
                target_path: Some(std::path::PathBuf::from(format!("C:\\Users\\[Username]\\AppData\\Local\\[AppName]\\{}", filename))),
                size: *size,
                hash: None,
                attributes: crate::core::FileAttributes {
                    readonly: false,
                    hidden: false,
                    system: false,
                    executable: *executable,
                },
                compression: Some("NSIS".to_string()),
            });
        }

        Ok(files)
    }

    /// Extract Squirrel-specific registry operations
    async fn extract_squirrel_registry(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Start with NSIS registry operations
        let mut operations = self.nsis_analyzer.extract_registry_operations(file_path).await?;
        
        // Add common Squirrel registry entries
        let squirrel_registry_ops = [
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[AppId]", "DisplayName"),
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[AppId]", "DisplayVersion"),
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[AppId]", "Publisher"),
            ("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\[AppId]", "UninstallString"),
            ("HKEY_CURRENT_USER\\Software\\[AppName]", "InstallLocation"),
            ("HKEY_CURRENT_USER\\Software\\[AppName]", "Version"),
            ("HKEY_CURRENT_USER\\Software\\Classes\\[AppName]", "URL Protocol"),
        ];

        for (key_path, value_name) in &squirrel_registry_ops {
            operations.push(RegistryOperation::SetValue {
                key_path: key_path.to_string(),
                value_name: value_name.to_string(),
                value_type: crate::core::RegistryValueType::String,
                value_data: crate::core::RegistryValue::String("[Squirrel Value]".to_string()),
                timestamp: Utc::now(),
            });
        }

        Ok(operations)
    }
}

#[async_trait]
impl InstallerAnalyzer for SquirrelAnalyzer {
    async fn can_analyze(&self, file_path: &Path) -> Result<bool> {
        // Validate file accessibility
        common::validate_file(file_path).await?;

        // Check if it's a Squirrel installer
        Self::is_squirrel_installer(file_path).await
    }

    fn format(&self) -> InstallerFormat {
        InstallerFormat::Squirrel
    }

    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_squirrel_metadata(file_path).await
    }

    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_squirrel_files(file_path).await
    }

    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>> {
        // Validate file first
        common::validate_file(file_path).await?;
        
        self.extract_squirrel_registry(file_path).await
    }
}

impl Default for SquirrelAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
