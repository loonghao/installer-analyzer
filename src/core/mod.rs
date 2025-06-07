//! Core module containing fundamental types and traits

pub mod error;
pub mod types;

// Re-export commonly used items
pub use error::{AnalyzerError, Result};
pub use types::*;
