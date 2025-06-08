use installer_analyzer::analyzers::{common, AnalyzerFactory};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Testing all files in tests/data directory...\n");

    // Read all files in tests/data directory
    let test_dir = Path::new("tests/data");
    if !test_dir.exists() {
        println!("❌ tests/data directory not found!");
        return Ok(());
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(test_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }

    files.sort();

    println!("Found {} test files:", files.len());
    for (i, file) in files.iter().enumerate() {
        println!(
            "  {}. {}",
            i + 1,
            file.file_name().unwrap().to_string_lossy()
        );
    }

    println!("\n{}", "=".repeat(80));
    println!("COMPREHENSIVE FILE ANALYSIS");
    println!("{}", "=".repeat(80));

    let mut results = Vec::new();

    for file_path in &files {
        println!(
            "\n📁 Analyzing: {}",
            file_path.file_name().unwrap().to_string_lossy()
        );
        println!("{}", "-".repeat(60));

        let mut result = FileAnalysisResult {
            file_name: file_path.file_name().unwrap().to_string_lossy().to_string(),
            file_size: 0,
            is_pe: false,
            detected_format: None,
            analyzer_found: false,
            metadata_extracted: false,
            files_extracted: false,
            registry_extracted: false,
            error: None,
        };

        // Get file size
        if let Ok(metadata) = fs::metadata(file_path) {
            result.file_size = metadata.len();
            println!(
                "  📊 File size: {} bytes ({:.2} MB)",
                result.file_size,
                result.file_size as f64 / 1024.0 / 1024.0
            );
        }

        // Check if PE file
        match common::is_pe_file(file_path).await {
            Ok(is_pe) => {
                result.is_pe = is_pe;
                println!("  🔍 PE file: {}", if is_pe { "Yes" } else { "No" });
            }
            Err(e) => {
                println!("  ❌ PE check error: {}", e);
            }
        }

        // Try to detect format using common utilities
        match common::detect_installer_format(file_path).await {
            Ok(format) => {
                result.detected_format = Some(format!("{:?}", format));
                println!("  🎯 Detected format: {:?}", format);
            }
            Err(e) => {
                println!("  ⚠️  Format detection error: {}", e);
            }
        }

        // Try to create analyzer
        match AnalyzerFactory::create_analyzer(file_path).await {
            Ok(analyzer) => {
                result.analyzer_found = true;
                let format = analyzer.format();
                println!("  ✅ Analyzer found: {:?}", format);

                // Test metadata extraction
                match analyzer.extract_metadata(file_path).await {
                    Ok(metadata) => {
                        result.metadata_extracted = true;
                        println!("  📋 Metadata extracted:");
                        println!("    - Product: {:?}", metadata.product_name);
                        println!("    - Version: {:?}", metadata.product_version);
                        println!("    - Manufacturer: {:?}", metadata.manufacturer);
                        println!("    - Properties: {} items", metadata.properties.len());
                    }
                    Err(e) => {
                        println!("  ❌ Metadata extraction failed: {}", e);
                    }
                }

                // Test file extraction
                match analyzer.extract_files(file_path).await {
                    Ok(files) => {
                        result.files_extracted = true;
                        println!("  📦 Files extracted: {} files", files.len());
                        if !files.is_empty() {
                            println!("    Sample files:");
                            for (i, file) in files.iter().take(3).enumerate() {
                                println!(
                                    "      {}. {} ({} bytes)",
                                    i + 1,
                                    file.path.display(),
                                    file.size
                                );
                            }
                            if files.len() > 3 {
                                println!("      ... and {} more files", files.len() - 3);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  ❌ File extraction failed: {}", e);
                    }
                }

                // Test registry extraction
                match analyzer.extract_registry_operations(file_path).await {
                    Ok(operations) => {
                        result.registry_extracted = true;
                        println!("  🔧 Registry operations: {} operations", operations.len());
                        if !operations.is_empty() {
                            println!("    Sample operations:");
                            for (i, op) in operations.iter().take(2).enumerate() {
                                match op {
                                    installer_analyzer::core::RegistryOperation::CreateKey {
                                        key_path,
                                        ..
                                    } => {
                                        println!("      {}. Create Key: {}", i + 1, key_path);
                                    }
                                    installer_analyzer::core::RegistryOperation::SetValue {
                                        key_path,
                                        value_name,
                                        ..
                                    } => {
                                        println!(
                                            "      {}. Set Value: {}\\{}",
                                            i + 1,
                                            key_path,
                                            value_name
                                        );
                                    }
                                    _ => {
                                        println!("      {}. Other operation", i + 1);
                                    }
                                }
                            }
                            if operations.len() > 2 {
                                println!("      ... and {} more operations", operations.len() - 2);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  ❌ Registry extraction failed: {}", e);
                    }
                }
            }
            Err(e) => {
                result.error = Some(e.to_string());
                println!("  ❌ No analyzer found: {}", e);

                // For unsupported files, try to provide more information
                if let Some(ext) = file_path.extension() {
                    match ext.to_str() {
                        Some("whl") => {
                            println!("  💡 This is a Python wheel file - could be supported with zip extraction");
                        }
                        Some("msi") => {
                            println!("  💡 This is an MSI file - should be supported, check file integrity");
                        }
                        Some("exe") => {
                            println!(
                                "  💡 This is an EXE file - might be a different installer format"
                            );
                        }
                        _ => {
                            println!("  💡 Unknown file extension: {}", ext.to_string_lossy());
                        }
                    }
                }
            }
        }

        results.push(result);
    }

    // Print summary
    println!("\n{}", "=".repeat(80));
    println!("ANALYSIS SUMMARY");
    println!("{}", "=".repeat(80));

    let total_files = results.len();
    let pe_files = results.iter().filter(|r| r.is_pe).count();
    let analyzer_found = results.iter().filter(|r| r.analyzer_found).count();
    let metadata_extracted = results.iter().filter(|r| r.metadata_extracted).count();
    let files_extracted = results.iter().filter(|r| r.files_extracted).count();
    let registry_extracted = results.iter().filter(|r| r.registry_extracted).count();

    println!("📊 Overall Statistics:");
    println!("  - Total files: {}", total_files);
    println!(
        "  - PE files: {} ({:.1}%)",
        pe_files,
        pe_files as f64 / total_files as f64 * 100.0
    );
    println!(
        "  - Analyzer found: {} ({:.1}%)",
        analyzer_found,
        analyzer_found as f64 / total_files as f64 * 100.0
    );
    println!(
        "  - Metadata extracted: {} ({:.1}%)",
        metadata_extracted,
        metadata_extracted as f64 / total_files as f64 * 100.0
    );
    println!(
        "  - Files extracted: {} ({:.1}%)",
        files_extracted,
        files_extracted as f64 / total_files as f64 * 100.0
    );
    println!(
        "  - Registry extracted: {} ({:.1}%)",
        registry_extracted,
        registry_extracted as f64 / total_files as f64 * 100.0
    );

    println!("\n📋 Detailed Results:");
    for result in &results {
        let status = if result.analyzer_found { "✅" } else { "❌" };
        println!(
            "  {} {} ({})",
            status,
            result.file_name,
            result.detected_format.as_deref().unwrap_or("Unknown")
        );
    }

    println!("\n🔍 Unsupported Files:");
    for result in results.iter().filter(|r| !r.analyzer_found) {
        println!(
            "  ❌ {} - {}",
            result.file_name,
            result.error.as_deref().unwrap_or("Unknown error")
        );
    }

    println!("\n✅ Supported Files:");
    for result in results.iter().filter(|r| r.analyzer_found) {
        println!(
            "  ✅ {} - {}",
            result.file_name,
            result.detected_format.as_deref().unwrap_or("Unknown")
        );
    }

    println!("\nAnalysis completed!");
    Ok(())
}

#[derive(Debug)]
struct FileAnalysisResult {
    file_name: String,
    file_size: u64,
    is_pe: bool,
    detected_format: Option<String>,
    analyzer_found: bool,
    metadata_extracted: bool,
    files_extracted: bool,
    registry_extracted: bool,
    error: Option<String>,
}
