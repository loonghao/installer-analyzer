//! Tests for CLI output functionality

use installer_analyzer::cli::output::{CliOutput, Icons, init_console};
use std::time::Duration;

#[test]
fn test_icons_consistency() {
    // Test that all icons are defined and not empty
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
fn test_icons_platform_specific_content() {
    // Test that Windows gets text-based icons
    #[cfg(windows)]
    {
        assert!(Icons::SUCCESS.starts_with('[') && Icons::SUCCESS.ends_with(']'));
        assert!(Icons::ERROR.starts_with('[') && Icons::ERROR.ends_with(']'));
        assert!(Icons::WARNING.starts_with('[') && Icons::WARNING.ends_with(']'));
        assert!(Icons::INFO.starts_with('[') && Icons::INFO.ends_with(']'));
        assert!(Icons::FILE.starts_with('[') && Icons::FILE.ends_with(']'));
        assert!(Icons::FOLDER.starts_with('[') && Icons::FOLDER.ends_with(']'));
        assert!(Icons::BROWSER.starts_with('[') && Icons::BROWSER.ends_with(']'));
    }
    
    // Test that non-Windows gets emoji or special characters
    #[cfg(not(windows))]
    {
        // These should be emoji characters, which are multi-byte
        assert!(Icons::SUCCESS.len() > 1);
        assert!(Icons::ERROR.len() > 1);
        assert!(Icons::WARNING.len() > 1);
    }
}

#[test]
fn test_progress_bar_creation() {
    let pb = CliOutput::create_progress_bar(100, "Test progress");
    assert_eq!(pb.length(), Some(100));
    
    // Test that progress bar has the expected message
    // Note: We can't easily test the actual display without complex setup
    pb.finish();
}

#[test]
fn test_spinner_creation() {
    let spinner = CliOutput::create_spinner("Test spinner");
    assert_eq!(spinner.length(), None); // Spinners have indefinite length
    
    spinner.finish();
}

#[test]
fn test_progress_bar_finish_methods() {
    let pb = CliOutput::create_progress_bar(10, "Test");
    
    // Test success finish
    CliOutput::finish_progress_success(&pb, "Success message");
    
    let pb2 = CliOutput::create_progress_bar(10, "Test 2");
    
    // Test error finish
    CliOutput::finish_progress_error(&pb2, "Error message");
}

#[test]
fn test_output_methods_dont_panic() {
    // Test that all output methods can be called without panicking
    CliOutput::success("Test success message");
    CliOutput::error("Test error message");
    CliOutput::warning("Test warning message");
    CliOutput::info("Test info message");
    CliOutput::file_info("Test label", "/path/to/file");
    CliOutput::folder_info("Test label", "/path/to/folder");
    CliOutput::browser_info("Test browser message");
    CliOutput::section_header("Test Section");
    CliOutput::subsection_header("Test Subsection");
    CliOutput::startup_banner("1.0.0");
    CliOutput::format_detection("html", Some("json"));
    CliOutput::format_detection("json", None);
}

#[test]
fn test_analysis_summary() {
    // Test that analysis summary can be called without panicking
    CliOutput::analysis_summary(
        "HTML",
        "/path/to/report.html",
        Duration::from_secs(5),
        Some(10),
    );
    
    CliOutput::analysis_summary(
        "JSON",
        "/path/to/report.json",
        Duration::from_millis(1500),
        None,
    );
}

#[test]
fn test_batch_summary() {
    // Test batch summary with various scenarios
    CliOutput::batch_summary(5, 0, Duration::from_secs(30));
    CliOutput::batch_summary(3, 2, Duration::from_secs(45));
    CliOutput::batch_summary(0, 1, Duration::from_secs(10));
}

#[test]
fn test_init_console() {
    // Test that console initialization doesn't panic
    init_console();
    
    // Call it multiple times to ensure it's safe
    init_console();
    init_console();
}

#[test]
fn test_duration_formatting_in_summaries() {
    // Test various duration formats
    let durations = vec![
        Duration::from_millis(500),   // 0.5 seconds
        Duration::from_secs(1),       // 1 second
        Duration::from_secs(65),      // 1 minute 5 seconds
        Duration::from_secs(3661),    // 1 hour 1 minute 1 second
    ];
    
    for duration in durations {
        CliOutput::analysis_summary("JSON", "/test/path", duration, Some(1));
        CliOutput::batch_summary(1, 0, duration);
    }
}

#[test]
fn test_file_count_scenarios() {
    // Test analysis summary with different file counts
    let file_counts = vec![None, Some(0), Some(1), Some(100), Some(1000)];
    
    for count in file_counts {
        CliOutput::analysis_summary(
            "HTML",
            "/test/report.html",
            Duration::from_secs(1),
            count,
        );
    }
}

#[test]
fn test_batch_processing_scenarios() {
    // Test various batch processing scenarios
    let scenarios = vec![
        (0, 0),   // No files processed
        (1, 0),   // One success, no failures
        (0, 1),   // No successes, one failure
        (5, 2),   // Mixed results
        (100, 0), // Large batch, all successful
        (50, 50), // Large batch, half failed
    ];
    
    for (processed, failed) in scenarios {
        CliOutput::batch_summary(processed, failed, Duration::from_secs(10));
    }
}

#[test]
fn test_format_detection_scenarios() {
    // Test format detection with various combinations
    let scenarios = vec![
        ("json", None),
        ("html", Some("html")),
        ("markdown", Some("json")), // Mismatch scenario
        ("json", Some("markdown")), // Another mismatch
    ];
    
    for (detected, explicit) in scenarios {
        CliOutput::format_detection(detected, explicit);
    }
}

#[test]
fn test_section_headers_with_various_lengths() {
    // Test headers with different lengths
    let headers = vec![
        "Short",
        "Medium Length Header",
        "Very Long Header That Spans Multiple Words And Contains Lots Of Text",
        "",  // Empty header
        "Header with 数字 and émojis! 🚀",
    ];
    
    for header in headers {
        CliOutput::section_header(header);
        CliOutput::subsection_header(header);
    }
}

#[test]
fn test_path_handling_in_output() {
    // Test various path formats
    let paths = vec![
        "/unix/style/path",
        "C:\\Windows\\Style\\Path",
        "relative/path",
        "file.txt",
        "",
        "path with spaces/file name.ext",
        "path/with/unicode/文件名.txt",
    ];
    
    for path in paths {
        CliOutput::file_info("Test file", path);
        CliOutput::folder_info("Test folder", path);
    }
}
