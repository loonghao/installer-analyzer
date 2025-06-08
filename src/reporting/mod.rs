//! Report generation functionality

use crate::core::{AnalysisResult, Result};
use std::path::Path;

pub mod generator;
pub mod templates;

// Re-export main types
pub use generator::ReportGenerator;

/// Report format options
#[derive(Debug, Clone)]
pub enum ReportFormat {
    Json,
    Html,
    Markdown,
}

/// Trait for report generators
#[allow(async_fn_in_trait)]
pub trait Reporter {
    /// Generate report in specified format
    async fn generate_report(
        &self,
        result: &AnalysisResult,
        format: ReportFormat,
    ) -> Result<String>;

    /// Save report to file
    async fn save_report(
        &self,
        result: &AnalysisResult,
        format: ReportFormat,
        output_path: &Path,
    ) -> Result<()>;
}
