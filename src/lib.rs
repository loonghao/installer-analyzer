//! Installer Analyzer Library
//! 
//! A comprehensive tool for analyzing software installation packages and monitoring installation behavior.
//! Supports static analysis of various installer formats (MSI, NSIS, InnoSetup) and dynamic sandbox monitoring.

pub mod core;
pub mod analyzers;
pub mod sandbox;
pub mod monitoring;
pub mod reporting;
pub mod cli;
pub mod api;
pub mod utils;

// Re-export commonly used types
pub use core::{
    error::{Result, AnalyzerError},
    types::{InstallerFormat, InstallerMetadata, AnalysisResult},
};

// Re-export main analyzer trait
pub use analyzers::InstallerAnalyzer;

// Re-export sandbox functionality
pub use sandbox::SandboxController;

// Re-export reporting
pub use reporting::ReportGenerator;
