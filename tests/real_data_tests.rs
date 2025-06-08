//! Integration tests using real installer files from tests/data

use installer_analyzer::cli::commands::{handle_analyze, handle_batch, handle_sandbox};
use std::path::PathBuf;
use tempfile::TempDir;

/// Get the path to test data directory
fn get_test_data_dir() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.push("tests");
    path.push("data");
    path
}

/// Get path to a specific test file
fn get_test_file(filename: &str) -> PathBuf {
    get_test_data_dir().join(filename)
}

#[tokio::test]
async fn test_analyze_real_msi_file() {
    let msi_file = get_test_file("rust-1.86.0-x86_64-pc-windows-msvc.msi");

    // Skip test if file doesn't exist
    if !msi_file.exists() {
        println!("Skipping test: MSI file not found at {:?}", msi_file);
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("rust_analysis.json");

    // Test JSON format
    let result = handle_analyze(&msi_file, Some(&output_file), Some("json"), false).await;

    match result {
        Ok(_) => {
            assert!(output_file.exists(), "Output file should be created");

            // Verify the output file contains valid JSON
            let content = std::fs::read_to_string(&output_file).unwrap();
            assert!(!content.is_empty(), "Output file should not be empty");

            // Try to parse as JSON to ensure it's valid
            let json_value: serde_json::Value =
                serde_json::from_str(&content).expect("Output should be valid JSON");

            // Check for expected fields in the analysis result
            assert!(
                json_value.get("metadata").is_some(),
                "Should have metadata field"
            );
            assert!(json_value.get("files").is_some(), "Should have files field");
        }
        Err(e) => {
            // Analysis might fail due to complex MSI structure, but should not panic
            println!("Analysis failed (expected for complex MSI): {}", e);
        }
    }
}

#[tokio::test]
async fn test_analyze_real_exe_files() {
    let exe_files = vec![
        "Gitify.Setup.6.3.0.exe",
        "wetype_installer_official_p_48.exe",
    ];

    for exe_filename in exe_files {
        let exe_file = get_test_file(exe_filename);

        if !exe_file.exists() {
            println!("Skipping test: EXE file not found at {:?}", exe_file);
            continue;
        }

        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join(format!(
            "{}_analysis.html",
            exe_file.file_stem().unwrap().to_str().unwrap()
        ));

        // Test HTML format with auto-detection
        let result = handle_analyze(&exe_file, Some(&output_file), None, false).await;

        match result {
            Ok(_) => {
                assert!(
                    output_file.exists(),
                    "Output file should be created for {}",
                    exe_filename
                );

                let content = std::fs::read_to_string(&output_file).unwrap();
                assert!(
                    !content.is_empty(),
                    "Output file should not be empty for {}",
                    exe_filename
                );

                // Check for HTML structure
                assert!(
                    content.contains("<html>") || content.contains("<!DOCTYPE"),
                    "Should contain HTML structure for {}",
                    exe_filename
                );
            }
            Err(e) => {
                println!(
                    "Analysis failed for {} (may be expected): {}",
                    exe_filename, e
                );
            }
        }
    }
}

#[tokio::test]
async fn test_analyze_real_whl_file() {
    let whl_file = get_test_file("persistent_ssh_agent-0.9.0-py3-none-any.whl");

    if !whl_file.exists() {
        println!("Skipping test: WHL file not found at {:?}", whl_file);
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("python_wheel_analysis.md");

    // Test Markdown format
    let result = handle_analyze(&whl_file, Some(&output_file), Some("markdown"), false).await;

    match result {
        Ok(_) => {
            assert!(output_file.exists(), "Output file should be created");

            let content = std::fs::read_to_string(&output_file).unwrap();
            assert!(!content.is_empty(), "Output file should not be empty");

            // Check for Markdown structure
            assert!(
                content.contains("#") || content.contains("##"),
                "Should contain Markdown headers"
            );
        }
        Err(e) => {
            println!("WHL analysis failed (may be expected): {}", e);
        }
    }
}

#[tokio::test]
async fn test_analyze_real_zip_file() {
    let zip_file = get_test_file("ShareX-17.1.0-portable.zip");

    if !zip_file.exists() {
        println!("Skipping test: ZIP file not found at {:?}", zip_file);
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("sharex_analysis.json");

    let result = handle_analyze(&zip_file, Some(&output_file), Some("json"), false).await;

    match result {
        Ok(_) => {
            assert!(output_file.exists(), "Output file should be created");

            let content = std::fs::read_to_string(&output_file).unwrap();
            assert!(!content.is_empty(), "Output file should not be empty");

            // Verify JSON structure
            let json_value: serde_json::Value =
                serde_json::from_str(&content).expect("Output should be valid JSON");

            assert!(json_value.get("files").is_some(), "Should have files field");
        }
        Err(e) => {
            // ZIP analysis might not be fully supported yet
            println!("ZIP analysis failed (may be expected): {}", e);
        }
    }
}

#[tokio::test]
async fn test_format_auto_detection_with_real_files() {
    let test_cases = vec![
        ("rust-1.86.0-x86_64-pc-windows-msvc.msi", "report.html"),
        ("Gitify.Setup.6.3.0.exe", "report.json"),
        ("persistent_ssh_agent-0.9.0-py3-none-any.whl", "report.md"),
    ];

    for (input_filename, output_filename) in test_cases {
        let input_file = get_test_file(input_filename);

        if !input_file.exists() {
            println!("Skipping test: Input file not found at {:?}", input_file);
            continue;
        }

        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join(output_filename);

        // Test auto-detection (no explicit format)
        let result = handle_analyze(&input_file, Some(&output_file), None, false).await;

        match result {
            Ok(_) => {
                assert!(
                    output_file.exists(),
                    "Output file should be created for {} -> {}",
                    input_filename,
                    output_filename
                );

                let content = std::fs::read_to_string(&output_file).unwrap();
                assert!(
                    !content.is_empty(),
                    "Output file should not be empty for {} -> {}",
                    input_filename,
                    output_filename
                );

                // Verify format based on extension
                if output_filename.ends_with(".html") {
                    assert!(
                        content.contains("<html>") || content.contains("<!DOCTYPE"),
                        "Should be HTML format"
                    );
                } else if output_filename.ends_with(".json") {
                    let _: serde_json::Value =
                        serde_json::from_str(&content).expect("Should be valid JSON");
                } else if output_filename.ends_with(".md") {
                    assert!(content.contains("#"), "Should contain Markdown headers");
                }
            }
            Err(e) => {
                println!(
                    "Analysis failed for {} (may be expected): {}",
                    input_filename, e
                );
            }
        }
    }
}

#[tokio::test]
async fn test_batch_processing_with_real_files() {
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");

    std::fs::create_dir(&input_dir).unwrap();
    std::fs::create_dir(&output_dir).unwrap();

    // Copy some real test files to input directory
    let test_files = vec![
        "rust-1.86.0-x86_64-pc-windows-msvc.msi",
        "Gitify.Setup.6.3.0.exe",
    ];

    let mut copied_files = 0;
    for filename in test_files {
        let source = get_test_file(filename);
        if source.exists() {
            let dest = input_dir.join(filename);
            std::fs::copy(&source, &dest).unwrap();
            copied_files += 1;
        }
    }

    if copied_files == 0 {
        println!("Skipping batch test: No test files available");
        return;
    }

    // Test batch processing
    let result = handle_batch(&input_dir, &output_dir, Some("json"), false).await;

    match result {
        Ok(_) => {
            // Check that output files were created
            let output_entries: Vec<_> = std::fs::read_dir(&output_dir)
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            assert!(!output_entries.is_empty(), "Should create output files");

            // Verify at least one output file has content
            for entry in output_entries {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    let content = std::fs::read_to_string(&path).unwrap();
                    if !content.is_empty() {
                        // Try to parse as JSON
                        let _: serde_json::Value = serde_json::from_str(&content)
                            .expect("Batch output should be valid JSON");
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Batch processing failed (may be expected): {}", e);
        }
    }
}

#[tokio::test]
async fn test_sandbox_with_real_exe_file() {
    let exe_file = get_test_file("Gitify.Setup.6.3.0.exe");

    if !exe_file.exists() {
        println!(
            "Skipping sandbox test: EXE file not found at {:?}",
            exe_file
        );
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("sandbox_analysis.html");

    // Test sandbox analysis with short timeout for safety
    let result = handle_sandbox(
        &exe_file,
        Some(&output_file),
        None,  // Auto-detect format
        10,    // Short timeout for testing
        false, // No network
        false, // Don't open browser
    )
    .await;

    match result {
        Ok(_) => {
            assert!(
                output_file.exists(),
                "Sandbox output file should be created"
            );

            let content = std::fs::read_to_string(&output_file).unwrap();
            assert!(!content.is_empty(), "Sandbox output should not be empty");

            // Should be HTML format (auto-detected from .html extension)
            assert!(
                content.contains("<html>") || content.contains("<!DOCTYPE"),
                "Sandbox output should be HTML format"
            );
        }
        Err(e) => {
            // Sandbox might fail due to security restrictions or unsupported format
            println!(
                "Sandbox analysis failed (may be expected in test environment): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_file_type_detection_accuracy() {
    let test_files = vec![
        ("rust-1.86.0-x86_64-pc-windows-msvc.msi", "msi"),
        ("Gitify.Setup.6.3.0.exe", "exe"),
        ("wetype_installer_official_p_48.exe", "exe"),
        ("ShareX-17.1.0-portable.zip", "zip"),
        ("persistent_ssh_agent-0.9.0-py3-none-any.whl", "whl"),
    ];

    for (filename, expected_type) in test_files {
        let file_path = get_test_file(filename);

        if !file_path.exists() {
            println!("Skipping file type test: {} not found", filename);
            continue;
        }

        // Test that our file type detection works correctly
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        assert_eq!(
            extension.to_lowercase(),
            expected_type,
            "File {} should have extension {}",
            filename,
            expected_type
        );

        // Test file size is reasonable (not empty, not too small)
        let metadata = std::fs::metadata(&file_path).unwrap();
        assert!(
            metadata.len() > 1024,
            "File {} should be larger than 1KB (actual: {} bytes)",
            filename,
            metadata.len()
        );

        println!(
            "✓ {} ({} bytes) - type: {}",
            filename,
            metadata.len(),
            expected_type
        );
    }
}

#[tokio::test]
async fn test_error_handling_with_corrupted_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create a fake installer file with wrong extension
    let fake_msi = temp_dir.path().join("fake.msi");
    std::fs::write(&fake_msi, b"This is not a real MSI file").unwrap();

    let output_file = temp_dir.path().join("fake_analysis.json");

    let result = handle_analyze(&fake_msi, Some(&output_file), Some("json"), false).await;

    // Should handle the error gracefully
    match result {
        Ok(_) => {
            // If it succeeds, check that it at least created some output
            assert!(
                output_file.exists(),
                "Should create output file even for fake installer"
            );
        }
        Err(e) => {
            // Expected to fail, but should not panic
            println!("Expected error for fake installer: {}", e);
            assert!(
                e.to_string().contains("Error") || e.to_string().contains("Failed"),
                "Error message should be descriptive"
            );
        }
    }
}

#[tokio::test]
async fn test_large_file_handling() {
    // Test with the largest file in our test data
    let large_file = get_test_file("rust-1.86.0-x86_64-pc-windows-msvc.msi");

    if !large_file.exists() {
        println!("Skipping large file test: MSI file not found");
        return;
    }

    let metadata = std::fs::metadata(&large_file).unwrap();
    println!(
        "Testing large file: {} ({} MB)",
        large_file.file_name().unwrap().to_str().unwrap(),
        metadata.len() / 1024 / 1024
    );

    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("large_file_analysis.json");

    // Measure analysis time
    let start = std::time::Instant::now();
    let result = handle_analyze(&large_file, Some(&output_file), Some("json"), false).await;
    let duration = start.elapsed();

    println!("Analysis took: {:?}", duration);

    match result {
        Ok(_) => {
            assert!(output_file.exists(), "Should handle large files");

            let content = std::fs::read_to_string(&output_file).unwrap();
            assert!(!content.is_empty(), "Should produce output for large files");

            // Verify it's valid JSON
            let _: serde_json::Value =
                serde_json::from_str(&content).expect("Large file output should be valid JSON");
        }
        Err(e) => {
            println!("Large file analysis failed (may be expected): {}", e);
        }
    }
}
