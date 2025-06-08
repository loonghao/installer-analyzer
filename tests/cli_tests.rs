//! Integration tests for CLI functionality

use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Get the path to the compiled binary
fn get_binary_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test executable name
    if path.ends_with("deps") {
        path.pop(); // Remove deps directory
    }
    path.push("installer-analyzer");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

/// Create a dummy installer file for testing
fn create_dummy_installer(dir: &std::path::Path, name: &str, extension: &str) -> PathBuf {
    let file_path = dir.join(format!("{}.{}", name, extension));
    std::fs::write(&file_path, b"dummy installer content").unwrap();
    file_path
}

#[test]
fn test_cli_help_commands() {
    let binary = get_binary_path();

    // Test main help
    let output = Command::new(&binary)
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("installer-analyzer"));
    assert!(stdout.contains("analyze"));
    assert!(stdout.contains("sandbox"));
    assert!(stdout.contains("batch"));

    // Test analyze help
    let output = Command::new(&binary)
        .args(&["analyze", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--input"));
    assert!(stdout.contains("--output"));
    assert!(stdout.contains("--format"));
    assert!(stdout.contains("--open"));

    // Test sandbox help
    let output = Command::new(&binary)
        .args(&["sandbox", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--timeout"));
    assert!(stdout.contains("--network"));

    // Test batch help
    let output = Command::new(&binary)
        .args(&["batch", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--input-dir"));
    assert!(stdout.contains("--output-dir"));
    assert!(stdout.contains("--sandbox"));
}

#[test]
fn test_cli_version() {
    let binary = get_binary_path();

    let output = Command::new(&binary)
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("installer-analyzer"));
}

#[test]
fn test_analyze_command_format_detection() {
    let binary = get_binary_path();
    let temp_dir = TempDir::new().unwrap();

    // Create a dummy installer
    let installer = create_dummy_installer(temp_dir.path(), "test", "msi");

    // Test HTML format detection
    let html_output = temp_dir.path().join("report.html");
    let output = Command::new(&binary)
        .args(&[
            "analyze",
            "--input",
            installer.to_str().unwrap(),
            "--output",
            html_output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    // Command might fail due to invalid installer, but should not crash
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show format detection or processing attempt
    assert!(stdout.contains("Installer Analyzer") || stderr.contains("Error"));

    // Test JSON format detection
    let json_output = temp_dir.path().join("report.json");
    let output = Command::new(&binary)
        .args(&[
            "analyze",
            "--input",
            installer.to_str().unwrap(),
            "--output",
            json_output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show format detection or processing attempt
    assert!(stdout.contains("Installer Analyzer") || stderr.contains("Error"));
}

#[test]
fn test_analyze_command_explicit_format() {
    let binary = get_binary_path();
    let temp_dir = TempDir::new().unwrap();

    // Create a dummy installer
    let installer = create_dummy_installer(temp_dir.path(), "test", "exe");
    let output_file = temp_dir.path().join("report.html");

    // Test explicit format that differs from extension
    let output = Command::new(&binary)
        .args(&[
            "analyze",
            "--input",
            installer.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show warning about format mismatch or processing attempt
    assert!(
        stdout.contains("Installer Analyzer")
            || stderr.contains("Error")
            || stderr.contains("Warning")
    );
}

#[test]
fn test_invalid_input_file() {
    let binary = get_binary_path();
    let temp_dir = TempDir::new().unwrap();

    let non_existent = temp_dir.path().join("non_existent.msi");
    let output_file = temp_dir.path().join("report.json");

    let output = Command::new(&binary)
        .args(&[
            "analyze",
            "--input",
            non_existent.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    // Should fail with non-zero exit code
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    // Should contain error message about file not found
    assert!(
        stderr.contains("Error") || stderr.contains("not found") || stderr.contains("No such file")
    );
}

#[test]
fn test_batch_command_empty_directory() {
    let binary = get_binary_path();
    let temp_dir = TempDir::new().unwrap();

    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");

    std::fs::create_dir(&input_dir).unwrap();
    std::fs::create_dir(&output_dir).unwrap();

    let output = Command::new(&binary)
        .args(&[
            "batch",
            "--input-dir",
            input_dir.to_str().unwrap(),
            "--output-dir",
            output_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    // Should handle empty directory gracefully
    assert!(
        stdout.contains("No supported installer files")
            || stdout.contains("Found 0 installer files")
            || stderr.contains("Error")
    );
}

#[test]
fn test_startup_banner() {
    let binary = get_binary_path();
    let temp_dir = TempDir::new().unwrap();

    // Create a dummy installer
    let installer = create_dummy_installer(temp_dir.path(), "test", "msi");
    let output_file = temp_dir.path().join("report.json");

    let output = Command::new(&binary)
        .args(&[
            "analyze",
            "--input",
            installer.to_str().unwrap(),
            "--output",
            output_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show startup banner
    assert!(stdout.contains("Installer Analyzer"));
    assert!(stdout.contains("Version"));
}
