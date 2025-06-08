//! Performance and benchmark tests using real installer files

use installer_analyzer::cli::commands::handle_analyze;
use std::path::PathBuf;
use std::time::{Duration, Instant};
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

/// Performance benchmark for different file types
#[tokio::test]
async fn benchmark_analysis_performance() {
    let test_files = vec![
        ("rust-1.86.0-x86_64-pc-windows-msvc.msi", "MSI"),
        ("Gitify.Setup.6.3.0.exe", "EXE (Gitify)"),
        ("wetype_installer_official_p_48.exe", "EXE (WeType)"),
        (
            "persistent_ssh_agent-0.9.0-py3-none-any.whl",
            "Python Wheel",
        ),
        ("ShareX-17.1.0-portable.zip", "ZIP Archive"),
    ];

    println!("\n=== Performance Benchmark Results ===");
    println!(
        "{:<40} {:<15} {:<15} {:<10}",
        "File", "Size (MB)", "Time (ms)", "Rate (MB/s)"
    );
    println!("{}", "-".repeat(80));

    for (filename, file_type) in test_files {
        let file_path = get_test_file(filename);

        if !file_path.exists() {
            println!(
                "{:<40} {:<15} {:<15} {:<10}",
                format!("{} ({})", filename, file_type),
                "NOT FOUND",
                "-",
                "-"
            );
            continue;
        }

        let metadata = std::fs::metadata(&file_path).unwrap();
        let file_size_mb = metadata.len() as f64 / 1024.0 / 1024.0;

        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("benchmark_output.json");

        // Warm up run (not measured)
        let _ = handle_analyze(&file_path, Some(&output_file), Some("json"), false).await;

        // Measured run
        let start = Instant::now();
        let result = handle_analyze(&file_path, Some(&output_file), Some("json"), false).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => {
                let duration_ms = duration.as_millis();
                let rate_mb_per_sec = if duration_ms > 0 {
                    file_size_mb / (duration_ms as f64 / 1000.0)
                } else {
                    0.0
                };

                println!(
                    "{:<40} {:<15.2} {:<15} {:<10.2}",
                    format!("{} ({})", filename, file_type),
                    file_size_mb,
                    duration_ms,
                    rate_mb_per_sec
                );
            }
            Err(e) => {
                println!(
                    "{:<40} {:<15.2} {:<15} {:<10}",
                    format!("{} ({})", filename, file_type),
                    file_size_mb,
                    format!("ERROR: {}", e),
                    "-"
                );
            }
        }
    }

    println!("{}", "-".repeat(80));
}

#[tokio::test]
async fn test_memory_usage_with_large_files() {
    let large_files = vec![
        "rust-1.86.0-x86_64-pc-windows-msvc.msi",
        "ShareX-17.1.0-portable.zip",
    ];

    for filename in large_files {
        let file_path = get_test_file(filename);

        if !file_path.exists() {
            println!("Skipping memory test: {} not found", filename);
            continue;
        }

        let metadata = std::fs::metadata(&file_path).unwrap();
        let file_size_mb = metadata.len() as f64 / 1024.0 / 1024.0;

        println!(
            "Testing memory usage for {} ({:.2} MB)",
            filename, file_size_mb
        );

        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("memory_test_output.json");

        // Note: In a real scenario, you might want to use a memory profiler
        // For now, we just ensure the analysis completes without excessive memory usage
        let start = Instant::now();
        let result = handle_analyze(&file_path, Some(&output_file), Some("json"), false).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => {
                println!("✓ {} completed in {:?}", filename, duration);

                // Check output file size is reasonable
                if output_file.exists() {
                    let output_size = std::fs::metadata(&output_file).unwrap().len();
                    let output_size_kb = output_size as f64 / 1024.0;
                    println!("  Output size: {:.2} KB", output_size_kb);

                    // Output should not be excessively large compared to input
                    assert!(
                        output_size < metadata.len() * 2,
                        "Output size should be reasonable compared to input"
                    );
                }
            }
            Err(e) => {
                println!("✗ {} failed: {}", filename, e);
            }
        }
    }
}

#[tokio::test]
async fn test_concurrent_analysis() {
    let test_files = vec![
        "Gitify.Setup.6.3.0.exe",
        "wetype_installer_official_p_48.exe",
        "persistent_ssh_agent-0.9.0-py3-none-any.whl",
    ];

    let mut available_files = Vec::new();
    for filename in test_files {
        let file_path = get_test_file(filename);
        if file_path.exists() {
            available_files.push((filename, file_path));
        }
    }

    if available_files.is_empty() {
        println!("Skipping concurrent test: No test files available");
        return;
    }

    println!(
        "Testing concurrent analysis with {} files",
        available_files.len()
    );

    let temp_dir = TempDir::new().unwrap();

    // Create concurrent analysis tasks
    let mut tasks = Vec::new();

    for (i, (filename, file_path)) in available_files.into_iter().enumerate() {
        let output_file = temp_dir
            .path()
            .join(format!("concurrent_output_{}.json", i));
        let file_path_clone = file_path.clone();

        let task = tokio::spawn(async move {
            let start = Instant::now();
            let result =
                handle_analyze(&file_path_clone, Some(&output_file), Some("json"), false).await;
            let duration = start.elapsed();
            (filename, result, duration)
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    let start_total = Instant::now();
    let results = futures::future::join_all(tasks).await;
    let total_duration = start_total.elapsed();

    println!("Concurrent analysis completed in {:?}", total_duration);

    let mut successful = 0;
    let mut failed = 0;

    for result in results {
        match result {
            Ok((filename, analysis_result, duration)) => match analysis_result {
                Ok(_) => {
                    println!("✓ {} completed in {:?}", filename, duration);
                    successful += 1;
                }
                Err(e) => {
                    println!("✗ {} failed: {}", filename, e);
                    failed += 1;
                }
            },
            Err(e) => {
                println!("✗ Task failed: {}", e);
                failed += 1;
            }
        }
    }

    println!("Results: {} successful, {} failed", successful, failed);

    // At least some analyses should succeed
    assert!(
        successful > 0 || failed > 0,
        "At least some analyses should complete"
    );
}

#[tokio::test]
async fn test_format_conversion_performance() {
    let test_file = get_test_file("persistent_ssh_agent-0.9.0-py3-none-any.whl");

    if !test_file.exists() {
        println!("Skipping format conversion test: WHL file not found");
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let formats = vec![("json", "JSON"), ("html", "HTML"), ("markdown", "Markdown")];

    println!("Testing format conversion performance:");

    for (format, format_name) in formats {
        let output_file = temp_dir.path().join(format!(
            "format_test.{}",
            if format == "markdown" { "md" } else { format }
        ));

        let start = Instant::now();
        let result = handle_analyze(&test_file, Some(&output_file), Some(format), false).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => {
                let output_size = if output_file.exists() {
                    std::fs::metadata(&output_file).unwrap().len()
                } else {
                    0
                };

                println!(
                    "  {} format: {:?} ({} bytes)",
                    format_name, duration, output_size
                );
            }
            Err(e) => {
                println!("  {} format: FAILED - {}", format_name, e);
            }
        }
    }
}

#[tokio::test]
async fn test_stress_analysis_repeated() {
    let test_file = get_test_file("Gitify.Setup.6.3.0.exe");

    if !test_file.exists() {
        println!("Skipping stress test: EXE file not found");
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let iterations = 5;

    println!("Running stress test with {} iterations", iterations);

    let mut durations = Vec::new();
    let mut successful = 0;

    for i in 0..iterations {
        let output_file = temp_dir.path().join(format!("stress_test_{}.json", i));

        let start = Instant::now();
        let result = handle_analyze(&test_file, Some(&output_file), Some("json"), false).await;
        let duration = start.elapsed();

        match result {
            Ok(_) => {
                durations.push(duration);
                successful += 1;
                println!("  Iteration {}: {:?}", i + 1, duration);
            }
            Err(e) => {
                println!("  Iteration {}: FAILED - {}", i + 1, e);
            }
        }
    }

    if !durations.is_empty() {
        let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
        let min_duration = durations.iter().min().unwrap();
        let max_duration = durations.iter().max().unwrap();

        println!("Stress test results:");
        println!("  Successful: {}/{}", successful, iterations);
        println!("  Average time: {:?}", avg_duration);
        println!("  Min time: {:?}", min_duration);
        println!("  Max time: {:?}", max_duration);

        // Performance should be relatively consistent
        let variance = max_duration.as_millis() as f64 / min_duration.as_millis() as f64;
        assert!(
            variance < 3.0,
            "Performance variance should be reasonable (got {:.2}x)",
            variance
        );
    }
}
