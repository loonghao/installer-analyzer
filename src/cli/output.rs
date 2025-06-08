//! Cross-platform CLI output utilities

use colored::*;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::time::Duration;

/// Cross-platform emojis that work on Windows
pub struct Icons;

impl Icons {
    pub const SUCCESS: &'static str = if cfg!(windows) { "[OK]" } else { "✅" };
    pub const ERROR: &'static str = if cfg!(windows) { "[ERR]" } else { "❌" };
    pub const WARNING: &'static str = if cfg!(windows) { "[WARN]" } else { "⚠️" };
    pub const INFO: &'static str = if cfg!(windows) { "[INFO]" } else { "ℹ️" };
    pub const FILE: &'static str = if cfg!(windows) { "[FILE]" } else { "📄" };
    pub const FOLDER: &'static str = if cfg!(windows) { "[DIR]" } else { "📁" };
    pub const BROWSER: &'static str = if cfg!(windows) { "[WEB]" } else { "🌐" };
    pub const ROCKET: &'static str = if cfg!(windows) { "[START]" } else { "🚀" };
    pub const GEAR: &'static str = if cfg!(windows) { "[PROC]" } else { "⚙️" };
    pub const MAGNIFYING_GLASS: &'static str = if cfg!(windows) { "[SCAN]" } else { "🔍" };
    pub const PACKAGE: &'static str = if cfg!(windows) { "[PKG]" } else { "📦" };
    pub const SHIELD: &'static str = if cfg!(windows) { "[SEC]" } else { "🛡️" };
}

/// Enhanced CLI output with colors and progress
pub struct CliOutput;

impl CliOutput {
    /// Print a success message
    pub fn success(message: &str) {
        println!("{} {}", Icons::SUCCESS.green().bold(), message.green());
    }

    /// Print an error message
    pub fn error(message: &str) {
        eprintln!("{} {}", Icons::ERROR.red().bold(), message.red());
    }

    /// Print a warning message
    pub fn warning(message: &str) {
        println!("{} {}", Icons::WARNING.yellow().bold(), message.yellow());
    }

    /// Print an info message
    pub fn info(message: &str) {
        println!("{} {}", Icons::INFO.blue().bold(), message);
    }

    /// Print a file-related message
    pub fn file_info(label: &str, path: &str) {
        println!("{} {}: {}", Icons::FILE.cyan().bold(), label.cyan(), path);
    }

    /// Print a folder-related message
    pub fn folder_info(label: &str, path: &str) {
        println!("{} {}: {}", Icons::FOLDER.cyan().bold(), label.cyan(), path);
    }

    /// Print a browser-related message
    pub fn browser_info(message: &str) {
        println!("{} {}", Icons::BROWSER.blue().bold(), message);
    }

    /// Print a section header
    pub fn section_header(title: &str) {
        println!();
        println!("{}", title.bold().underline());
        println!("{}", "=".repeat(title.len()).dimmed());
    }

    /// Print a subsection header
    pub fn subsection_header(title: &str) {
        println!();
        println!("{}", title.bold());
        println!("{}", "-".repeat(title.len()).dimmed());
    }

    /// Create a progress bar for file processing
    pub fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {msg}",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(120));
        pb
    }

    /// Create a spinner for indeterminate progress
    pub fn create_spinner(message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"]),
        );
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(120));
        pb
    }

    /// Finish a progress bar with success
    pub fn finish_progress_success(pb: &ProgressBar, message: &str) {
        pb.finish_with_message(format!("{} {}", Icons::SUCCESS, message));
    }

    /// Finish a progress bar with error
    pub fn finish_progress_error(pb: &ProgressBar, message: &str) {
        pb.finish_with_message(format!("{} {}", Icons::ERROR, message));
    }

    /// Print analysis summary
    pub fn analysis_summary(
        format: &str,
        output_path: &str,
        duration: Duration,
        file_count: Option<usize>,
    ) {
        Self::section_header("Analysis Complete");
        Self::file_info("Report format", format);
        Self::folder_info("Report saved to", output_path);
        Self::info(&format!(
            "Analysis duration: {:.2}s",
            duration.as_secs_f64()
        ));

        if let Some(count) = file_count {
            Self::info(&format!("Files analyzed: {}", count));
        }
    }

    /// Print batch processing summary
    pub fn batch_summary(processed: usize, failed: usize, total_duration: Duration) {
        Self::section_header("Batch Processing Complete");
        Self::success(&format!("Successfully processed: {}", processed));

        if failed > 0 {
            Self::error(&format!("Failed to process: {}", failed));
        }

        Self::info(&format!(
            "Total duration: {:.2}s",
            total_duration.as_secs_f64()
        ));

        if processed > 0 {
            Self::info(&format!(
                "Average time per file: {:.2}s",
                total_duration.as_secs_f64() / processed as f64
            ));
        }
    }

    /// Print startup banner
    pub fn startup_banner(version: &str) {
        println!();
        println!("{}", "Installer Analyzer".bold().cyan());
        println!("{}", format!("Version {}", version).dimmed());
        println!(
            "{}",
            "A comprehensive tool for analyzing software installation packages".dimmed()
        );
        println!();
    }

    /// Print format detection info
    pub fn format_detection(detected: &str, explicit: Option<&str>) {
        if let Some(explicit_format) = explicit {
            if explicit_format != detected {
                Self::warning(&format!(
                    "Format mismatch: detected '{}' from file extension, using explicit '{}'",
                    detected, explicit_format
                ));
            }
        } else {
            Self::info(&format!("Auto-detected format: {}", detected));
        }
    }
}

/// Initialize console for better Windows compatibility
pub fn init_console() {
    // Enable ANSI colors on Windows
    #[cfg(windows)]
    {
        let _ = console::set_colors_enabled(true);
        let _ = console::set_colors_enabled_stderr(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icons_windows_fallback() {
        // Test that icons are properly defined for both platforms
        assert!(!Icons::SUCCESS.is_empty());
        assert!(!Icons::ERROR.is_empty());
        assert!(!Icons::WARNING.is_empty());
        assert!(!Icons::INFO.is_empty());
        assert!(!Icons::FILE.is_empty());
        assert!(!Icons::FOLDER.is_empty());
        assert!(!Icons::BROWSER.is_empty());
        assert!(!Icons::ROCKET.is_empty());
        assert!(!Icons::GEAR.is_empty());
        assert!(!Icons::MAGNIFYING_GLASS.is_empty());
        assert!(!Icons::PACKAGE.is_empty());
        assert!(!Icons::SHIELD.is_empty());
    }

    #[test]
    fn test_icons_platform_specific() {
        #[cfg(windows)]
        {
            assert_eq!(Icons::SUCCESS, "[OK]");
            assert_eq!(Icons::ERROR, "[ERR]");
            assert_eq!(Icons::WARNING, "[WARN]");
        }

        #[cfg(not(windows))]
        {
            assert_eq!(Icons::SUCCESS, "✅");
            assert_eq!(Icons::ERROR, "❌");
            assert_eq!(Icons::WARNING, "⚠️");
        }
    }

    #[test]
    fn test_progress_bar_creation() {
        let pb = CliOutput::create_progress_bar(100, "Test message");
        assert_eq!(pb.length(), Some(100));

        let spinner = CliOutput::create_spinner("Test spinner");
        assert_eq!(spinner.length(), None); // Spinners have no defined length
    }

    #[test]
    fn test_init_console() {
        // Should not panic
        init_console();
    }
}
