use std::path::Path;
use installer_analyzer::analyzers::{ArchiveAnalyzer, InstallerAnalyzer, common};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Testing Archive analyzer...\n");
    
    let analyzer = ArchiveAnalyzer::new();
    
    // Test files
    let test_files = [
        ("tests/data/persistent_ssh_agent-0.9.0-py3-none-any.whl", true),  // Should be ZIP
        ("tests/data/Gitify.Setup.6.3.0.exe", false),      // Should not be archive
        ("tests/data/ArtFlow-1.5.6.msi", false),           // Should not be archive
        ("Cargo.toml", false),                              // Should not be archive
    ];
    
    println!("=== Archive Detection Test ===");
    for (file_path, expected_archive) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match analyzer.can_analyze(path).await {
                Ok(can_analyze) => {
                    let status = if can_analyze == *expected_archive { "✓" } else { "✗" };
                    println!("  {} {}: Archive = {} (expected {})", 
                        status, file_path, can_analyze, expected_archive);
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test archive format detection
    println!("\n=== Archive Format Detection Test ===");
    for (file_path, _) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match common::is_archive_file(path).await {
                Ok(is_archive) => {
                    if is_archive {
                        match common::detect_archive_format(path).await {
                            Ok(format) => {
                                println!("  ✓ {}: Archive format = {}", file_path, format);
                            }
                            Err(e) => {
                                println!("  ✗ {}: Format detection error = {}", file_path, e);
                            }
                        }
                    } else {
                        println!("  - {}: Not an archive", file_path);
                    }
                }
                Err(e) => {
                    println!("  ✗ {}: Archive check error = {}", file_path, e);
                }
            }
        }
    }
    
    // Test metadata extraction for archive files
    println!("\n=== Archive Metadata Extraction Test ===");
    let archive_files = [
        "tests/data/persistent_ssh_agent-0.9.0-py3-none-any.whl",
    ];
    
    for file_path in &archive_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing metadata extraction for: {}", file_path);
                    match analyzer.extract_metadata(path).await {
                        Ok(metadata) => {
                            println!("    ✓ Format: {:?}", metadata.format);
                            println!("    ✓ Product: {:?}", metadata.product_name);
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
                    println!("  - {}: Not detected as archive", file_path);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test file extraction
    println!("\n=== Archive File Extraction Test ===");
    for file_path in &archive_files {
        let path = Path::new(file_path);
        if path.exists() {
            if let Ok(can_analyze) = analyzer.can_analyze(path).await {
                if can_analyze {
                    println!("  Testing file extraction for: {}", file_path);
                    match analyzer.extract_files(path).await {
                        Ok(files) => {
                            println!("    ✓ Found {} files", files.len());
                            for (i, file) in files.iter().take(10).enumerate() {
                                println!("      {}. {} ({} bytes) [{}]", 
                                    i + 1, 
                                    file.path.display(), 
                                    file.size,
                                    file.compression.as_deref().unwrap_or("None")
                                );
                            }
                            if files.len() > 10 {
                                println!("      ... and {} more files", files.len() - 10);
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
    
    // Test registry extraction (should be empty for archives)
    println!("\n=== Archive Registry Extraction Test ===");
    for file_path in &archive_files {
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
    
    println!("\nArchive analyzer test completed!");
    Ok(())
}
