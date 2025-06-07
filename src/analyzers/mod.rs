//! Static analyzer implementations for various installer formats

use crate::core::{Result, InstallerFormat, InstallerMetadata, FileEntry, RegistryOperation};
use async_trait::async_trait;
use std::path::Path;

pub mod msi;
pub mod nsis;
pub mod inno;
pub mod archive;
pub mod wheel;
pub mod msix;
pub mod installshield;
pub mod wix;
pub mod squirrel;
pub mod common;

// Re-export analyzers
pub use msi::MsiAnalyzer;
pub use nsis::NsisAnalyzer;
pub use inno::InnoAnalyzer;
pub use archive::ArchiveAnalyzer;
pub use wheel::WheelAnalyzer;
pub use msix::MsixAnalyzer;
pub use installshield::InstallShieldAnalyzer;
pub use wix::WixAnalyzer;
pub use squirrel::SquirrelAnalyzer;

// Re-export common utilities
pub use common::{
    is_pe_file,
    search_file_content,
    detect_installer_format,
    read_file_content_range,
    calculate_file_hash,
    get_file_size,
    validate_file,
    detect_format_by_extension,
    read_file_header,
    is_archive_file,
    detect_archive_format,
};

/// Main trait for installer analyzers
#[async_trait]
pub trait InstallerAnalyzer: Send + Sync {
    /// Detect if this analyzer can handle the given file
    async fn can_analyze(&self, file_path: &Path) -> Result<bool>;

    /// Get the installer format this analyzer handles
    fn format(&self) -> InstallerFormat;

    /// Extract metadata from the installer
    async fn extract_metadata(&self, file_path: &Path) -> Result<InstallerMetadata>;

    /// Extract file list from the installer
    async fn extract_files(&self, file_path: &Path) -> Result<Vec<FileEntry>>;

    /// Extract registry operations from install scripts
    async fn extract_registry_operations(&self, file_path: &Path) -> Result<Vec<RegistryOperation>>;

    /// Perform complete analysis
    async fn analyze(&self, file_path: &Path) -> Result<(InstallerMetadata, Vec<FileEntry>, Vec<RegistryOperation>)> {
        let metadata = self.extract_metadata(file_path).await?;
        let files = self.extract_files(file_path).await?;
        let registry_ops = self.extract_registry_operations(file_path).await?;
        
        Ok((metadata, files, registry_ops))
    }
}

/// Factory for creating appropriate analyzers
pub struct AnalyzerFactory;

impl AnalyzerFactory {
    /// Create an analyzer for the given file
    pub async fn create_analyzer(file_path: &Path) -> Result<Box<dyn InstallerAnalyzer>> {
        tracing::debug!("Attempting to find analyzer for: {}", file_path.display());

        // Try WiX analyzer first (MSI variant with specific characteristics)
        let wix_analyzer = WixAnalyzer::new();
        if wix_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected WiX analyzer for: {}", file_path.display());
            return Ok(Box::new(wix_analyzer));
        }

        // Try MSI analyzer (general MSI format)
        let msi_analyzer = MsiAnalyzer::new();
        if msi_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected MSI analyzer for: {}", file_path.display());
            return Ok(Box::new(msi_analyzer));
        }

        // Try Python Wheel analyzer (specific file extension)
        let wheel_analyzer = WheelAnalyzer::new();
        if wheel_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected Python Wheel analyzer for: {}", file_path.display());
            return Ok(Box::new(wheel_analyzer));
        }

        // Try MSIX/AppX analyzer (specific file extension)
        let msix_analyzer = MsixAnalyzer::new();
        if msix_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected MSIX/AppX analyzer for: {}", file_path.display());
            return Ok(Box::new(msix_analyzer));
        }

        // Try InstallShield analyzer (PE-based detection)
        let installshield_analyzer = InstallShieldAnalyzer::new();
        if installshield_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected InstallShield analyzer for: {}", file_path.display());
            return Ok(Box::new(installshield_analyzer));
        }

        // Try Squirrel analyzer (NSIS variant for Electron apps)
        let squirrel_analyzer = SquirrelAnalyzer::new();
        if squirrel_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected Squirrel analyzer for: {}", file_path.display());
            return Ok(Box::new(squirrel_analyzer));
        }

        // Try NSIS analyzer (general NSIS format)
        let nsis_analyzer = NsisAnalyzer::new();
        if nsis_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected NSIS analyzer for: {}", file_path.display());
            return Ok(Box::new(nsis_analyzer));
        }

        // Try InnoSetup analyzer
        let inno_analyzer = InnoAnalyzer::new();
        if inno_analyzer.can_analyze(file_path).await? {
            tracing::info!("Selected InnoSetup analyzer for: {}", file_path.display());
            return Ok(Box::new(inno_analyzer));
        }

        tracing::warn!("No suitable analyzer found for: {}", file_path.display());
        Err(crate::core::AnalyzerError::unsupported_format(
            format!("No analyzer found for file: {}", file_path.display())
        ))
    }

    /// Get all available analyzers
    pub fn get_all_analyzers() -> Vec<Box<dyn InstallerAnalyzer>> {
        vec![
            Box::new(WixAnalyzer::new()),
            Box::new(MsiAnalyzer::new()),
            Box::new(WheelAnalyzer::new()),
            Box::new(MsixAnalyzer::new()),
            Box::new(InstallShieldAnalyzer::new()),
            Box::new(SquirrelAnalyzer::new()),
            Box::new(NsisAnalyzer::new()),
            Box::new(InnoAnalyzer::new()),
        ]
    }

    /// Get analyzer by format
    pub fn get_analyzer_by_format(format: InstallerFormat) -> Option<Box<dyn InstallerAnalyzer>> {
        match format {
            InstallerFormat::MSI => Some(Box::new(MsiAnalyzer::new())),
            InstallerFormat::WiX => Some(Box::new(WixAnalyzer::new())),
            InstallerFormat::PythonWheel => Some(Box::new(WheelAnalyzer::new())),
            InstallerFormat::MSIX => Some(Box::new(MsixAnalyzer::new())),
            InstallerFormat::InstallShield => Some(Box::new(InstallShieldAnalyzer::new())),
            InstallerFormat::NSIS => Some(Box::new(NsisAnalyzer::new())),
            InstallerFormat::Squirrel => Some(Box::new(SquirrelAnalyzer::new())),
            InstallerFormat::InnoSetup => Some(Box::new(InnoAnalyzer::new())),
            _ => None,
        }
    }

    /// Get supported formats
    pub fn get_supported_formats() -> Vec<InstallerFormat> {
        vec![
            InstallerFormat::MSI,
            InstallerFormat::WiX,
            InstallerFormat::PythonWheel,
            InstallerFormat::MSIX,
            InstallerFormat::InstallShield,
            InstallerFormat::NSIS,
            InstallerFormat::Squirrel,
            InstallerFormat::InnoSetup,
        ]
    }
}
