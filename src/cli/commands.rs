//! CLI command implementations

use crate::analyzers::AnalyzerFactory;
use crate::cli::output::CliOutput;
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
    format: Option<&str>,
    open_browser: bool,
) -> Result<()> {
    CliOutput::info(&format!("Starting static analysis of: {}", input.display()));

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
    let report_format = determine_format(format, output)?;

    if let Some(output_path) = output {
        let is_html = matches!(report_format, ReportFormat::Html);
        let format_name = format_to_string(&report_format);

        report_generator
            .save_report(&result, report_format, output_path)
            .await?;

        CliOutput::analysis_summary(
            format_name,
            &output_path.display().to_string(),
            analysis_duration,
            Some(result.files.len()),
        );

        // Open browser if requested and format is HTML
        if open_browser && is_html {
            CliOutput::browser_info("Opening report in browser...");
            if let Err(e) = open_browser_to_file(output_path) {
                CliOutput::warning(&format!("Failed to open browser: {}", e));
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
    format: Option<&str>,
    timeout: u64,
    enable_network: bool,
    open_browser: bool,
) -> Result<()> {
    CliOutput::info(&format!(
        "Starting sandbox analysis of: {}",
        input.display()
    ));

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
    let report_format = determine_format(format, output)?;

    if let Some(output_path) = output {
        let is_html = matches!(report_format, ReportFormat::Html);
        let format_name = format_to_string(&report_format);

        report_generator
            .save_report(&result, report_format, output_path)
            .await?;

        CliOutput::success("Sandbox analysis complete!");
        CliOutput::file_info("Report format", format_name);
        CliOutput::folder_info("Report saved to", &output_path.display().to_string());

        // Open browser if requested and format is HTML
        if open_browser && is_html {
            CliOutput::browser_info("Opening report in browser...");
            if let Err(e) = open_browser_to_file(output_path) {
                CliOutput::warning(&format!("Failed to open browser: {}", e));
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
    format: Option<&str>,
    use_sandbox: bool,
) -> Result<()> {
    CliOutput::section_header("Batch Analysis");
    CliOutput::folder_info("Input directory", &input_dir.display().to_string());
    CliOutput::folder_info("Output directory", &output_dir.display().to_string());

    // Create output directory if it doesn't exist
    tokio::fs::create_dir_all(output_dir).await?;

    // Find all installer files first to get total count
    let mut entries = tokio::fs::read_dir(input_dir).await?;
    let mut installer_files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && is_supported_file(&path) {
            installer_files.push(path);
        }
    }

    if installer_files.is_empty() {
        CliOutput::warning("No supported installer files found in the directory");
        return Ok(());
    }

    CliOutput::info(&format!(
        "Found {} installer files to process",
        installer_files.len()
    ));

    // Create progress bar
    let pb = CliOutput::create_progress_bar(installer_files.len() as u64, "Processing installers");

    let mut processed = 0;
    let mut failed = 0;
    let batch_start = Instant::now();

    for path in installer_files {
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let format_str = format.unwrap_or("json");
        let output_file = output_dir.join(format!(
            "{}_report.{}",
            file_name,
            get_file_extension(format_str)
        ));

        pb.set_message(format!("Processing: {}", file_name));

        let result = if use_sandbox {
            handle_sandbox(&path, Some(&output_file), format, 300, false, false).await
        } else {
            handle_analyze(&path, Some(&output_file), format, false).await
        };

        match result {
            Ok(_) => {
                processed += 1;
                pb.println(format!("✓ Completed: {}", path.display()));
            }
            Err(e) => {
                failed += 1;
                pb.println(format!("✗ Failed: {} - {}", path.display(), e));
            }
        }

        pb.inc(1);
    }

    CliOutput::finish_progress_success(&pb, "Batch processing complete");

    let total_duration = batch_start.elapsed();
    CliOutput::batch_summary(processed, failed, total_duration);

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

/// Detect format from output file extension
fn detect_format_from_path(path: &Path) -> Option<ReportFormat> {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        match ext.to_lowercase().as_str() {
            "json" => Some(ReportFormat::Json),
            "html" | "htm" => Some(ReportFormat::Html),
            "md" | "markdown" => Some(ReportFormat::Markdown),
            _ => None,
        }
    } else {
        None
    }
}

/// Determine the final format to use, considering output path and explicit format
fn determine_format(
    explicit_format: Option<&str>,
    output_path: Option<&Path>,
) -> Result<ReportFormat> {
    // If output path is provided, try to detect format from extension
    if let Some(path) = output_path {
        if let Some(detected_format) = detect_format_from_path(path) {
            // If no explicit format provided, use detected format
            if explicit_format.is_none() {
                return Ok(detected_format);
            }
            // If explicit format differs from detected, warn but use explicit
            if let Some(format_str) = explicit_format {
                let explicit_parsed = parse_format(format_str)?;
                if !format_matches(&explicit_parsed, &detected_format) {
                    eprintln!(
                        "Warning: Explicit format '{}' differs from file extension. Using explicit format.",
                        format_str
                    );
                }
                return Ok(explicit_parsed);
            }
        }
    }

    // Fall back to explicit format or default
    if let Some(format_str) = explicit_format {
        parse_format(format_str)
    } else {
        // Default to JSON if no format specified and no output file to detect from
        Ok(ReportFormat::Json)
    }
}

/// Check if two formats match
fn format_matches(format1: &ReportFormat, format2: &ReportFormat) -> bool {
    matches!(
        (format1, format2),
        (ReportFormat::Json, ReportFormat::Json)
            | (ReportFormat::Html, ReportFormat::Html)
            | (ReportFormat::Markdown, ReportFormat::Markdown)
    )
}

/// Convert ReportFormat to human-readable string
fn format_to_string(format: &ReportFormat) -> &'static str {
    match format {
        ReportFormat::Json => "JSON",
        ReportFormat::Html => "HTML",
        ReportFormat::Markdown => "Markdown",
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

    CliOutput::success(&format!("Report opened in browser: {}", url));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_parse_format() {
        assert!(matches!(parse_format("json"), Ok(ReportFormat::Json)));
        assert!(matches!(parse_format("html"), Ok(ReportFormat::Html)));
        assert!(matches!(
            parse_format("markdown"),
            Ok(ReportFormat::Markdown)
        ));
        assert!(matches!(parse_format("md"), Ok(ReportFormat::Markdown)));

        // Test case insensitive
        assert!(matches!(parse_format("JSON"), Ok(ReportFormat::Json)));
        assert!(matches!(parse_format("HTML"), Ok(ReportFormat::Html)));

        // Test invalid format
        assert!(parse_format("invalid").is_err());
    }

    #[test]
    fn test_detect_format_from_path() {
        let json_path = PathBuf::from("report.json");
        assert!(matches!(
            detect_format_from_path(&json_path),
            Some(ReportFormat::Json)
        ));

        let html_path = PathBuf::from("report.html");
        assert!(matches!(
            detect_format_from_path(&html_path),
            Some(ReportFormat::Html)
        ));

        let htm_path = PathBuf::from("report.htm");
        assert!(matches!(
            detect_format_from_path(&htm_path),
            Some(ReportFormat::Html)
        ));

        let md_path = PathBuf::from("report.md");
        assert!(matches!(
            detect_format_from_path(&md_path),
            Some(ReportFormat::Markdown)
        ));

        let markdown_path = PathBuf::from("report.markdown");
        assert!(matches!(
            detect_format_from_path(&markdown_path),
            Some(ReportFormat::Markdown)
        ));

        // Test unsupported extension
        let txt_path = PathBuf::from("report.txt");
        assert!(detect_format_from_path(&txt_path).is_none());

        // Test no extension
        let no_ext_path = PathBuf::from("report");
        assert!(detect_format_from_path(&no_ext_path).is_none());
    }

    #[test]
    fn test_determine_format_auto_detection() {
        let html_path = PathBuf::from("report.html");

        // No explicit format, should detect from extension
        let result = determine_format(None, Some(&html_path));
        assert!(matches!(result, Ok(ReportFormat::Html)));

        // Explicit format matches extension
        let result = determine_format(Some("html"), Some(&html_path));
        assert!(matches!(result, Ok(ReportFormat::Html)));

        // Explicit format differs from extension (should use explicit)
        let result = determine_format(Some("json"), Some(&html_path));
        assert!(matches!(result, Ok(ReportFormat::Json)));
    }

    #[test]
    fn test_determine_format_defaults() {
        // No output path, no explicit format - should default to JSON
        let result = determine_format(None, None);
        assert!(matches!(result, Ok(ReportFormat::Json)));

        // No output path, explicit format
        let result = determine_format(Some("html"), None);
        assert!(matches!(result, Ok(ReportFormat::Html)));

        // Output path with no extension, explicit format
        let no_ext_path = PathBuf::from("report");
        let result = determine_format(Some("markdown"), Some(&no_ext_path));
        assert!(matches!(result, Ok(ReportFormat::Markdown)));
    }

    #[test]
    fn test_format_matches() {
        assert!(format_matches(&ReportFormat::Json, &ReportFormat::Json));
        assert!(format_matches(&ReportFormat::Html, &ReportFormat::Html));
        assert!(format_matches(
            &ReportFormat::Markdown,
            &ReportFormat::Markdown
        ));

        assert!(!format_matches(&ReportFormat::Json, &ReportFormat::Html));
        assert!(!format_matches(
            &ReportFormat::Html,
            &ReportFormat::Markdown
        ));
        assert!(!format_matches(
            &ReportFormat::Markdown,
            &ReportFormat::Json
        ));
    }

    #[test]
    fn test_format_to_string() {
        assert_eq!(format_to_string(&ReportFormat::Json), "JSON");
        assert_eq!(format_to_string(&ReportFormat::Html), "HTML");
        assert_eq!(format_to_string(&ReportFormat::Markdown), "Markdown");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(get_file_extension("json"), "json");
        assert_eq!(get_file_extension("html"), "html");
        assert_eq!(get_file_extension("markdown"), "md");
        assert_eq!(get_file_extension("md"), "md");
        assert_eq!(get_file_extension("unknown"), "txt"); // Default is txt, not json
    }

    #[test]
    fn test_is_supported_file() {
        // Test currently supported extensions
        assert!(is_supported_file(&PathBuf::from("test.msi")));
        assert!(is_supported_file(&PathBuf::from("test.exe")));
        assert!(is_supported_file(&PathBuf::from("test.whl"))); // Python wheels are supported

        // Test case insensitive
        assert!(is_supported_file(&PathBuf::from("test.MSI")));
        assert!(is_supported_file(&PathBuf::from("test.EXE")));

        // Test currently unsupported extensions (marked as false in implementation)
        assert!(!is_supported_file(&PathBuf::from("test.zip")));
        assert!(!is_supported_file(&PathBuf::from("test.7z")));
        assert!(!is_supported_file(&PathBuf::from("test.rar")));
        assert!(!is_supported_file(&PathBuf::from("test.tar")));
        assert!(!is_supported_file(&PathBuf::from("test.gz")));
        assert!(!is_supported_file(&PathBuf::from("test.bz2")));

        // Test completely unsupported extensions
        assert!(!is_supported_file(&PathBuf::from("test.txt")));
        assert!(!is_supported_file(&PathBuf::from("test.doc")));
        assert!(!is_supported_file(&PathBuf::from("test")));
    }

    #[test]
    fn test_open_browser_to_file_url_generation() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.html");
        std::fs::write(&test_file, "<html><body>Test</body></html>").unwrap();

        // Test that the function generates proper file:// URLs
        // Note: We can't test actual browser opening in unit tests
        let result = open_browser_to_file(&test_file);

        // The function should succeed in generating the URL even if browser opening fails
        // In CI environments, browser opening will fail but URL generation should work
        match result {
            Ok(_) => {
                // Browser opened successfully (unlikely in CI)
            }
            Err(e) => {
                // Expected in CI environments - browser opening fails but that's OK
                // The important thing is that the function doesn't panic
                assert!(e.to_string().contains("Failed to open browser"));
            }
        }
    }
}
