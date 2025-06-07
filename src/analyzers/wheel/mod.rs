//! Python Wheel (.whl) format analyzer

pub mod analyzer;
pub mod parser;

// Re-export main components
pub use analyzer::WheelAnalyzer;
pub use parser::{WheelParser, WheelMetadata, WheelDependency};
