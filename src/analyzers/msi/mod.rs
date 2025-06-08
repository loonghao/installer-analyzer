//! MSI (Microsoft Installer) format analyzer

pub mod analyzer;
pub mod database;
pub mod tables;

// Re-export main analyzer
pub use analyzer::MsiAnalyzer;
