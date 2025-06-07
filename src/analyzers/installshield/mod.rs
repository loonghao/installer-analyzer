//! InstallShield format analyzer for enterprise installation packages

pub mod analyzer;
pub mod parser;

// Re-export main components
pub use analyzer::InstallShieldAnalyzer;
pub use parser::{InstallShieldParser, InstallShieldVersion, InstallShieldMetadata};
