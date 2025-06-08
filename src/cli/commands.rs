//! CLI command implementations

use crate::analyzers::AnalyzerFactory;
use crate::core::{AnalysisResult, AnalyzerError, Result, SandboxConfig};
use crate::reporting::{ReportFormat, ReportGenerator, Reporter};
use crate::sandbox::{Sandbox, SandboxController};
use chrono::Utc;
use std::path::Path;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Handle the analyze command
pub async fn handle_analyze(
    input: &Path,
    output: Option<&Path>,
    format: &str,
    open_browser: bool,
) -> Result<()> {
    tracing::info!("Starting static analysis of: {}", input.display());

    // Create analyzer
    let analyzer = AnalyzerFactory::create_analyzer(input).await?;

    // Perform analysis
    let start_time = Instant::now();
    let (metadata, files, registry_ops) = analyzer.analyze(input).await?;
    let analysis_duration = start_time.elapsed();

    // Create analysis result
    let result = AnalysisResult {
        session_id: Uuid::new_v4(),
        metadata,
        files,
        registry_operations: registry_ops,
        file_operations: Vec::new(), // Static analysis doesn't capture file operations
        process_operations: Vec::new(),
        network_operations: Vec::new(),
        analyzed_at: Utc::now(),
        analysis_duration,
        dynamic_analysis: false,
    };

    // Generate and save report
    let report_generator = ReportGenerator::new();
    let report_format = parse_format(format)?;

    if let Some(output_path) = output {
        let is_html = matches!(report_format, ReportFormat::Html);
        report_generator
            .save_report(&result, report_format, output_path)
            .await?;
        println!(
            "Analysis complete. Report saved to: {}",
            output_path.display()
        );

        // Open browser if requested and format is HTML
        if open_browser && is_html {
            if let Err(e) = open_browser_to_file(output_path) {
                eprintln!("Warning: Failed to open browser: {}", e);
            }
        }
    } else {
        let report_content = report_generator
            .generate_report(&result, report_format)
            .await?;
        println!("{}", report_content);
    }

    Ok(())
}

/// Handle the sandbox command
pub async fn handle_sandbox(
    input: &Path,
    output: Option<&Path>,
    format: &str,
    timeout: u64,
    enable_network: bool,
    open_browser: bool,
) -> Result<()> {
    tracing::info!("Starting sandbox analysis of: {}", input.display());

    // Create sandbox configuration
    let config = SandboxConfig {
        enable_network,
        max_execution_time: Duration::from_secs(timeout),
        ..Default::default()
    };

    // Create sandbox controller
    let mut sandbox = SandboxController::with_config(config);

    // Perform sandbox analysis
    let result = sandbox.analyze_installer(input).await?;

    // Generate and save report
    let report_generator = ReportGenerator::new();
    let report_format = parse_format(format)?;

    if let Some(output_path) = output {
        let is_html = matches!(report_format, ReportFormat::Html);
        report_generator
            .save_report(&result, report_format, output_path)
            .await?;
        println!(
            "Sandbox analysis complete. Report saved to: {}",
            output_path.display()
        );

        // Open browser if requested and format is HTML
        if open_browser && is_html {
            if let Err(e) = open_browser_to_file(output_path) {
                eprintln!("Warning: Failed to open browser: {}", e);
            }
        }
    } else {
        let report_content = report_generator
            .generate_report(&result, report_format)
            .await?;
        println!("{}", report_content);
    }

    Ok(())
}

/// Handle the batch command
pub async fn handle_batch(
    input_dir: &Path,
    output_dir: &Path,
    format: &str,
    use_sandbox: bool,
) -> Result<()> {
    tracing::info!(
        "Starting batch analysis of directory: {}",
        input_dir.display()
    );

    // Create output directory if it doesn't exist
    tokio::fs::create_dir_all(output_dir).await?;

    // Find all installer files
    let mut entries = tokio::fs::read_dir(input_dir).await?;
    let mut processed = 0;
    let mut failed = 0;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Check if it's a supported installer format
        if !is_supported_file(&path) {
            continue;
        }

        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let output_file = output_dir.join(format!(
            "{}_report.{}",
            file_name,
            get_file_extension(format)
        ));

        println!("Processing: {}", path.display());

        let result = if use_sandbox {
            handle_sandbox(&path, Some(&output_file), format, 300, false, false).await
        } else {
            handle_analyze(&path, Some(&output_file), format, false).await
        };

        match result {
            Ok(_) => {
                processed += 1;
                println!("✓ Completed: {}", path.display());
            }
            Err(e) => {
                failed += 1;
                eprintln!("✗ Failed: {} - {}", path.display(), e);
            }
        }
    }

    println!("\nBatch processing complete:");
    println!("  Processed: {}", processed);
    println!("  Failed: {}", failed);

    Ok(())
}

/// Handle the info command
pub async fn handle_info() -> Result<()> {
    println!("Installer Analyzer - Supported Formats");
    println!("=====================================");
    println!();
    println!("Static Analysis (Fully Supported):");
    println!("  ✅ MSI (Microsoft Installer) - .msi files");
    println!("     • Metadata extraction (product, version, manufacturer)");
    println!("     • File list with sizes and attributes");
    println!("     • Registry operations analysis");
    println!();
    println!("  ✅ NSIS (Nullsoft Scriptable Install System) - .exe files");
    println!("     • Format detection via signature patterns");
    println!("     • Basic metadata extraction");
    println!("     • File structure analysis");
    println!();
    println!("  ✅ InnoSetup - .exe files");
    println!("     • Format detection via signature patterns");
    println!("     • Basic metadata extraction");
    println!("     • File structure analysis");
    println!();
    println!("Planned Support:");
    println!("  🔄 Python Wheel - .whl files");
    println!("     • ZIP-based archive extraction");
    println!("     • METADATA file parsing");
    println!("     • Dependency analysis");
    println!();
    println!("  🔄 7zip Archive Support - various formats");
    println!("     • Universal archive extraction fallback");
    println!("     • Support for .7z, .rar, .tar.gz, etc.");
    println!();
    println!("Dynamic Analysis (Sandbox):");
    println!("  ✅ File system monitoring");
    println!("  ✅ Registry monitoring");
    println!("  🔄 Process monitoring (planned)");
    println!("  🔄 Network monitoring (planned)");
    println!();
    println!("Report Formats:");
    println!("  • JSON - Machine-readable structured data");
    println!("  • HTML - Human-readable web format with file tree view");
    println!("  • Markdown - Documentation-friendly format");
    println!();
    println!("Usage Examples:");
    println!("  installer-analyzer analyze app.msi --format html");
    println!("  installer-analyzer analyze setup.exe --format json");
    println!("  installer-analyzer sandbox installer.exe --timeout 300");
    println!("  installer-analyzer batch ./installers/ ./reports/ --format html");

    Ok(())
}

/// Parse format string to ReportFormat enum
fn parse_format(format: &str) -> Result<ReportFormat> {
    match format.to_lowercase().as_str() {
        "json" => Ok(ReportFormat::Json),
        "html" => Ok(ReportFormat::Html),
        "markdown" | "md" => Ok(ReportFormat::Markdown),
        _ => Err(AnalyzerError::config_error(format!(
            "Unsupported format: {}",
            format
        ))),
    }
}

/// Get file extension for report format
fn get_file_extension(format: &str) -> &str {
    match format.to_lowercase().as_str() {
        "json" => "json",
        "html" => "html",
        "markdown" | "md" => "md",
        _ => "txt",
    }
}

/// Check if file is a supported installer format
fn is_supported_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        match ext.to_lowercase().as_str() {
            // Fully supported formats
            "msi" => true, // MSI files
            "exe" => true, // NSIS and InnoSetup files
            // Planned support
            "whl" => true, // Python Wheel files
            // Other potential formats
            "7z" | "rar" | "zip" | "tar" | "gz" | "bz2" => false, // Not yet supported
            _ => false,
        }
    } else {
        false
    }
}

/// Open browser to view the generated HTML report
fn open_browser_to_file(file_path: &Path) -> Result<()> {
    let absolute_path = file_path.canonicalize().map_err(|e| {
        AnalyzerError::file_not_found(format!("Failed to get absolute path: {}", e))
    })?;

    let url = format!("file://{}", absolute_path.display());

    open::that(&url)
        .map_err(|e| AnalyzerError::config_error(format!("Failed to open browser: {}", e)))?;

    println!("Opening report in browser: {}", url);
    Ok(())
}
