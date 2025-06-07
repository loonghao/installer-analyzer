//! MSI (Microsoft Installer) format analyzer

pub mod database;
pub mod tables;
pub mod analyzer;

// Re-export main analyzer
pub use analyzer::MsiAnalyzer;
