//! InnoSetup installer format analyzer

pub mod analyzer;
pub mod parser;

// Re-export main analyzer
pub use analyzer::InnoAnalyzer;
