use installer_analyzer::analyzers::{InnoAnalyzer, InstallerAnalyzer};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Testing InnoSetup analyzer...\n");

    let analyzer = InnoAnalyzer::new();

    // Test files
    let test_files = [
        ("tests/data/VSCodeSetup-x64-1.100.0.exe", true), // Should be InnoSetup
        ("tests/data/Gitify.Setup.6.3.0.exe", false),     // Should be NSIS
        ("tests/data/pycharm-2025.1.1.1.exe", false),     // Should be NSIS
        ("tests/data/ArtFlow-1.5.6.msi", false),          // Should not be InnoSetup
    ];

    println!("=== InnoSetup Detection Test ===");
    for (file_path, expected_inno) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match analyzer.can_analyze(path).await {
                Ok(can_analyze) => {
                    let status = if can_analyze == *expected_inno {
                        "✓"
                    } else {
                        "✗"
                    };
                    println!(
                        "  {} {}: InnoSetup = {} (expected {})",
                        status, file_path, can_analyze, expected_inno
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

    // Test metadata extraction for InnoSetup files
    println!("\n=== InnoSetup Metadata Extraction Test ===");
    let inno_files = ["tests/data/VSCodeSetup-x64-1.100.0.exe"];

    for file_path in &inno_files {
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

                            // Show some properties
                            for (key, value) in metadata.properties.iter().take(5) {
                                println!("      - {}: {}", key, value);
                            }
                        }
                        Err(e) => {
                            println!("    ✗ Metadata extraction error: {}", e);
                        }
                    }
                } else {
                    println!("  - {}: Not detected as InnoSetup", file_path);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }

    // Test file extraction
    println!("\n=== InnoSetup File Extraction Test ===");
    for file_path in &inno_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing file extraction for: {}", file_path);
                    match analyzer.extract_files(path).await {
                        Ok(files) => {
                            println!("    ✓ Found {} files", files.len());
                            for (i, file) in files.iter().take(8).enumerate() {
                                println!(
                                    "      {}. {} ({} bytes) [{}]",
                                    i + 1,
                                    file.path.display(),
                                    file.size,
                                    file.compression.as_deref().unwrap_or("None")
                                );
                            }
                            if files.len() > 8 {
                                println!("      ... and {} more files", files.len() - 8);
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
    println!("\n=== InnoSetup Registry Extraction Test ===");
    for file_path in &inno_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing registry extraction for: {}", file_path);
                    match analyzer.extract_registry_operations(path).await {
                        Ok(operations) => {
                            println!("    ✓ Found {} registry operations", operations.len());
                            for (i, op) in operations.iter().take(5).enumerate() {
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
                            if operations.len() > 5 {
                                println!("      ... and {} more operations", operations.len() - 5);
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

    // Test format comparison
    println!("\n=== Format Comparison Test ===");
    let all_files = [
        "tests/data/VSCodeSetup-x64-1.100.0.exe", // InnoSetup
        "tests/data/Gitify.Setup.6.3.0.exe",      // NSIS
        "tests/data/ArtFlow-1.5.6.msi",           // MSI
    ];

    for file_path in &all_files {
        let path = Path::new(file_path);
        if path.exists() {
            match analyzer.can_analyze(path).await {
                Ok(can_analyze) => {
                    if can_analyze {
                        println!("  ✓ {}: Detected as InnoSetup", file_path);
                    } else {
                        println!("  - {}: Not InnoSetup", file_path);
                    }
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        }
    }

    println!("\nInnoSetup analyzer test completed!");
    Ok(())
}
