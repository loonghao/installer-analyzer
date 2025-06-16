//! Installer Analyzer Library
//!
//! A comprehensive tool for analyzing software installation packages and monitoring installation behavior.
//! Supports static analysis of various installer formats (MSI, NSIS, InnoSetup) and dynamic sandbox monitoring.

pub mod analyzers;
pub mod api;
pub mod cli;
pub mod core;
pub mod monitoring;
pub mod reporting;
pub mod sandbox;
pub mod updater;
pub mod utils;

// Re-export commonly used types
pub use core::{
    error::{AnalyzerError, Result},
    types::{AnalysisResult, InstallerFormat, InstallerMetadata},
};

// Re-export main analyzer trait
pub use analyzers::InstallerAnalyzer;

// Re-export sandbox functionality
pub use sandbox::SandboxController;

// Re-export reporting
pub use reporting::ReportGenerator;

// Re-export updater functionality
pub use updater::{UpdateConfig, UpdateInfo, Updater};
