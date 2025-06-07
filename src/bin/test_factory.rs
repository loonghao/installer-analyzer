use std::path::Path;
use installer_analyzer::analyzers::AnalyzerFactory;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Testing AnalyzerFactory...\n");
    
    // Test files with expected formats
    let test_files = [
        ("tests/data/ArtFlow-1.5.6.msi", "MSI"),
        ("tests/data/Gitify.Setup.6.3.0.exe", "NSIS"),
        ("tests/data/pycharm-2025.1.1.1.exe", "NSIS"),
        ("tests/data/VSCodeSetup-x64-1.100.0.exe", "InnoSetup"),
    ];
    
    println!("=== Analyzer Selection Test ===");
    for (file_path, expected_format) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match AnalyzerFactory::create_analyzer(path).await {
                Ok(analyzer) => {
                    let detected_format = format!("{:?}", analyzer.format());
                    let status = if detected_format == *expected_format { "✓" } else { "✗" };
                    println!("  {} {}: {} (expected {})", 
                        status, file_path, detected_format, expected_format);
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test get_all_analyzers
    println!("\n=== Available Analyzers Test ===");
    let analyzers = AnalyzerFactory::get_all_analyzers();
    println!("  Found {} analyzers:", analyzers.len());
    for (i, analyzer) in analyzers.iter().enumerate() {
        println!("    {}. {:?} Analyzer", i + 1, analyzer.format());
    }
    
    // Test get_supported_formats
    println!("\n=== Supported Formats Test ===");
    let formats = AnalyzerFactory::get_supported_formats();
    println!("  Supported formats ({}):", formats.len());
    for (i, format) in formats.iter().enumerate() {
        println!("    {}. {:?}", i + 1, format);
    }
    
    // Test get_analyzer_by_format
    println!("\n=== Analyzer by Format Test ===");
    for format in &formats {
        match AnalyzerFactory::get_analyzer_by_format(*format) {
            Some(analyzer) => {
                println!("  ✓ {:?}: Analyzer available", format);
                assert_eq!(analyzer.format(), *format);
            }
            None => {
                println!("  ✗ {:?}: No analyzer available", format);
            }
        }
    }
    
    // Test unsupported file
    println!("\n=== Unsupported File Test ===");
    let unsupported_files = [
        "Cargo.toml",
        "README.md",
    ];
    
    for file_path in &unsupported_files {
        let path = Path::new(file_path);
        if path.exists() {
            match AnalyzerFactory::create_analyzer(path).await {
                Ok(analyzer) => {
                    println!("  ✗ {}: Unexpected analyzer found: {:?}", file_path, analyzer.format());
                }
                Err(_) => {
                    println!("  ✓ {}: Correctly rejected as unsupported", file_path);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test analyzer priority (MSI should have highest priority)
    println!("\n=== Analyzer Priority Test ===");
    println!("  Testing analyzer selection priority...");
    
    // Test with each file type to ensure correct analyzer is selected
    for (file_path, expected_format) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(analyzer) = AnalyzerFactory::create_analyzer(path).await {
                let detected_format = format!("{:?}", analyzer.format());
                if detected_format == *expected_format {
                    println!("    ✓ {}: Priority correct ({})", file_path, detected_format);
                } else {
                    println!("    ✗ {}: Priority issue - got {} expected {}", 
                        file_path, detected_format, expected_format);
                }
            }
        }
    }
    
    // Test complete analysis workflow
    println!("\n=== Complete Analysis Workflow Test ===");
    let test_file = "tests/data/Gitify.Setup.6.3.0.exe";
    let path = Path::new(test_file);
    
    if path.exists() {
        println!("  Testing complete workflow with: {}", test_file);
        match AnalyzerFactory::create_analyzer(path).await {
            Ok(analyzer) => {
                println!("    ✓ Analyzer created: {:?}", analyzer.format());
                
                // Test metadata extraction
                match analyzer.extract_metadata(path).await {
                    Ok(metadata) => {
                        println!("    ✓ Metadata extracted: {} bytes", metadata.file_size);
                    }
                    Err(e) => {
                        println!("    ✗ Metadata extraction failed: {}", e);
                    }
                }
                
                // Test file extraction
                match analyzer.extract_files(path).await {
                    Ok(files) => {
                        println!("    ✓ Files extracted: {} files", files.len());
                    }
                    Err(e) => {
                        println!("    ✗ File extraction failed: {}", e);
                    }
                }
                
                // Test registry extraction
                match analyzer.extract_registry_operations(path).await {
                    Ok(operations) => {
                        println!("    ✓ Registry operations extracted: {} operations", operations.len());
                    }
                    Err(e) => {
                        println!("    ✗ Registry extraction failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("    ✗ Analyzer creation failed: {}", e);
            }
        }
    } else {
        println!("  - Test file not found: {}", test_file);
    }
    
    println!("\nAnalyzerFactory test completed!");
    Ok(())
}
