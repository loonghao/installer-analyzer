//! Archive format analyzer for compressed files (ZIP, 7z, etc.)

pub mod analyzer;
pub mod parser;

// Re-export main components
pub use analyzer::ArchiveAnalyzer;
pub use parser::{ArchiveParser, ArchiveFormat, ArchiveEntry};
