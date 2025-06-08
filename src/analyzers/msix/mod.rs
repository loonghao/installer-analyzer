//! MSIX/AppX format analyzer for modern Windows applications

pub mod analyzer;
pub mod parser;

// Re-export main components
pub use analyzer::MsixAnalyzer;
pub use parser::{AppxCapability, AppxDependency, AppxManifest, MsixParser};
