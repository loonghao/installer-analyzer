use installer_analyzer::analyzers::{InstallerAnalyzer, NsisAnalyzer};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Testing NSIS analyzer...\n");

    let analyzer = NsisAnalyzer::new();

    // Test files
    let test_files = [
        ("tests/data/Gitify.Setup.6.3.0.exe", true), // Should be NSIS
        ("tests/data/pycharm-2025.1.1.1.exe", true), // Should be NSIS
        ("tests/data/VSCodeSetup-x64-1.100.0.exe", false), // Should be InnoSetup
        ("tests/data/ArtFlow-1.5.6.msi", false),     // Should not be NSIS
    ];

    println!("=== NSIS Detection Test ===");
    for (file_path, expected_nsis) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match analyzer.can_analyze(path).await {
                Ok(can_analyze) => {
                    let status = if can_analyze == *expected_nsis {
                        "✓"
                    } else {
                        "✗"
                    };
                    println!(
                        "  {} {}: NSIS = {} (expected {})",
                        status, file_path, can_analyze, expected_nsis
                    );
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }

    // Test metadata extraction for NSIS files
    println!("\n=== NSIS Metadata Extraction Test ===");
    let nsis_files = [
        "tests/data/Gitify.Setup.6.3.0.exe",
        "tests/data/pycharm-2025.1.1.1.exe",
    ];

    for file_path in &nsis_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing metadata extraction for: {}", file_path);
                    match analyzer.extract_metadata(path).await {
                        Ok(metadata) => {
                            println!("    ✓ Format: {:?}", metadata.format);
                            println!("    ✓ Product: {:?}", metadata.product_name);
                            println!("    ✓ Version: {:?}", metadata.product_version);
                            println!("    ✓ Manufacturer: {:?}", metadata.manufacturer);
                            println!("    ✓ File Size: {}", metadata.file_size);
                            println!("    ✓ Properties: {} items", metadata.properties.len());
                        }
                        Err(e) => {
                            println!("    ✗ Metadata extraction error: {}", e);
                        }
                    }
                } else {
                    println!("  - {}: Not detected as NSIS", file_path);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }

    // Test file extraction
    println!("\n=== NSIS File Extraction Test ===");
    for file_path in &nsis_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing file extraction for: {}", file_path);
                    match analyzer.extract_files(path).await {
                        Ok(files) => {
                            println!("    ✓ Found {} files", files.len());
                            for (i, file) in files.iter().take(5).enumerate() {
                                println!(
                                    "      {}. {} ({} bytes)",
                                    i + 1,
                                    file.path.display(),
                                    file.size
                                );
                            }
                            if files.len() > 5 {
                                println!("      ... and {} more files", files.len() - 5);
                            }
                        }
                        Err(e) => {
                            println!("    ✗ File extraction error: {}", e);
                        }
                    }
                }
            }
        }
    }

    // Test registry extraction
    println!("\n=== NSIS Registry Extraction Test ===");
    for file_path in &nsis_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing registry extraction for: {}", file_path);
                    match analyzer.extract_registry_operations(path).await {
                        Ok(operations) => {
                            println!("    ✓ Found {} registry operations", operations.len());
                            for (i, op) in operations.iter().take(3).enumerate() {
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
                            if operations.len() > 3 {
                                println!("      ... and {} more operations", operations.len() - 3);
                            }
                        }
                        Err(e) => {
                            println!("    ✗ Registry extraction error: {}", e);
                        }
                    }
                }
            }
        }
    }

    println!("\nNSIS analyzer test completed!");
    Ok(())
}
