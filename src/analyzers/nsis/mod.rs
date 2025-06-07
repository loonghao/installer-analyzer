//! NSIS (Nullsoft Scriptable Install System) format analyzer

pub mod analyzer;
pub mod parser;

// Re-export main analyzer
pub use analyzer::NsisAnalyzer;
