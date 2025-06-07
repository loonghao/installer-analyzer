use std::path::Path;
use installer_analyzer::analyzers::{WheelAnalyzer, InstallerAnalyzer, AnalyzerFactory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Testing Python Wheel analyzer...\n");
    
    let analyzer = WheelAnalyzer::new();
    
    // Test files
    let test_files = [
        ("tests/data/persistent_ssh_agent-0.9.0-py3-none-any.whl", true),  // Should be Wheel
        ("tests/data/Gitify.Setup.6.3.0.exe", false),      // Should not be Wheel
        ("tests/data/ArtFlow-1.5.6.msi", false),           // Should not be Wheel
        ("Cargo.toml", false),                              // Should not be Wheel
    ];
    
    println!("=== Python Wheel Detection Test ===");
    for (file_path, expected_wheel) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match analyzer.can_analyze(path).await {
                Ok(can_analyze) => {
                    let status = if can_analyze == *expected_wheel { "✓" } else { "✗" };
                    println!("  {} {}: Wheel = {} (expected {})", 
                        status, file_path, can_analyze, expected_wheel);
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test AnalyzerFactory integration
    println!("\n=== AnalyzerFactory Integration Test ===");
    for (file_path, expected_wheel) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match AnalyzerFactory::create_analyzer(path).await {
                Ok(factory_analyzer) => {
                    let is_wheel = matches!(factory_analyzer.format(), installer_analyzer::core::InstallerFormat::PythonWheel);
                    let status = if is_wheel == *expected_wheel { "✓" } else { "✗" };
                    println!("  {} {}: Factory selected {:?} (wheel = {})", 
                        status, file_path, factory_analyzer.format(), is_wheel);
                }
                Err(e) => {
                    if !expected_wheel {
                        println!("  ✓ {}: Correctly rejected ({})", file_path, e);
                    } else {
                        println!("  ✗ {}: Unexpected rejection ({})", file_path, e);
                    }
                }
            }
        }
    }
    
    // Test metadata extraction for wheel files
    println!("\n=== Python Wheel Metadata Extraction Test ===");
    let wheel_files = [
        "tests/data/persistent_ssh_agent-0.9.0-py3-none-any.whl",
    ];
    
    for file_path in &wheel_files {
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
                            
                            // Show wheel-specific properties
                            let wheel_props = [
                                "wheel_name", "wheel_version", "wheel_summary", 
                                "wheel_author", "wheel_license", "wheel_requires_python",
                                "wheel_dependencies_count", "package_type"
                            ];
                            
                            for prop in &wheel_props {
                                if let Some(value) = metadata.properties.get(*prop) {
                                    println!("      - {}: {}", prop, value);
                                }
                            }
                        }
                        Err(e) => {
                            println!("    ✗ Metadata extraction error: {}", e);
                        }
                    }
                } else {
                    println!("  - {}: Not detected as Python Wheel", file_path);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test file extraction
    println!("\n=== Python Wheel File Extraction Test ===");
    for file_path in &wheel_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing file extraction for: {}", file_path);
                    match analyzer.extract_files(path).await {
                        Ok(files) => {
                            println!("    ✓ Found {} files", files.len());
                            
                            // Group files by type
                            let mut python_files = 0;
                            let mut metadata_files = 0;
                            let mut other_files = 0;
                            
                            for file in &files {
                                let file_name = file.path.to_string_lossy();
                                if file_name.ends_with(".py") {
                                    python_files += 1;
                                } else if file_name.contains("METADATA") || file_name.contains("WHEEL") || file_name.contains(".dist-info") {
                                    metadata_files += 1;
                                } else {
                                    other_files += 1;
                                }
                            }
                            
                            println!("      - Python files: {}", python_files);
                            println!("      - Metadata files: {}", metadata_files);
                            println!("      - Other files: {}", other_files);
                            
                            // Show sample files
                            println!("    Sample files:");
                            for (i, file) in files.iter().take(8).enumerate() {
                                println!("      {}. {} ({} bytes) [{}]", 
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
    
    // Test registry extraction (should be empty for Python wheels)
    println!("\n=== Python Wheel Registry Extraction Test ===");
    for file_path in &wheel_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing registry extraction for: {}", file_path);
                    match analyzer.extract_registry_operations(path).await {
                        Ok(operations) => {
                            println!("    ✓ Found {} registry operations (expected 0)", operations.len());
                        }
                        Err(e) => {
                            println!("    ✗ Registry extraction error: {}", e);
                        }
                    }
                }
            }
        }
    }
    
    // Test METADATA parsing
    println!("\n=== METADATA File Parsing Test ===");
    for file_path in &wheel_files {
        let path = Path::new(file_path);
        if path.exists() {
            println!("  Testing METADATA parsing for: {}", file_path);
            let parser = installer_analyzer::analyzers::wheel::WheelParser::new();
            match parser.extract_metadata(path) {
                Ok(metadata) => {
                    println!("    ✓ Package name: {}", metadata.name);
                    println!("    ✓ Package version: {}", metadata.version);
                    if let Some(summary) = &metadata.summary {
                        println!("    ✓ Summary: {}", summary);
                    }
                    if let Some(author) = &metadata.author {
                        println!("    ✓ Author: {}", author);
                    }
                    if let Some(license) = &metadata.license {
                        println!("    ✓ License: {}", license);
                    }
                    if let Some(requires_python) = &metadata.requires_python {
                        println!("    ✓ Requires Python: {}", requires_python);
                    }
                    println!("    ✓ Dependencies: {} items", metadata.requires_dist.len());
                    println!("    ✓ Classifiers: {} items", metadata.classifier.len());
                    
                    // Show sample dependencies
                    if !metadata.requires_dist.is_empty() {
                        println!("    Sample dependencies:");
                        for (i, dep) in metadata.requires_dist.iter().take(3).enumerate() {
                            println!("      {}. {} {}", 
                                i + 1, 
                                dep.name, 
                                dep.version_spec.as_deref().unwrap_or("")
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("    ✗ METADATA parsing error: {}", e);
                }
            }
        }
    }
    
    println!("\nPython Wheel analyzer test completed!");
    Ok(())
}
