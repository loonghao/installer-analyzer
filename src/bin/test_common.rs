use std::path::Path;
use installer_analyzer::analyzers::common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("Testing common analyzer functions...\n");
    
    // Test PE file detection
    println!("=== PE File Detection Test ===");
    
    let test_files = [
        ("tests/data/ArtFlow-1.5.6.msi", false), // MSI file, not PE
        ("tests/data/Gitify.Setup.6.3.0.exe", true), // PE file
        ("tests/data/VSCodeSetup-x64-1.100.0.exe", true), // PE file
        ("tests/data/pycharm-2025.1.1.1.exe", true), // PE file
    ];
    
    for (file_path, expected_pe) in &test_files {
        let path = Path::new(file_path);
        if path.exists() {
            match common::is_pe_file(path).await {
                Ok(is_pe) => {
                    let status = if is_pe == *expected_pe { "✓" } else { "✗" };
                    println!("  {} {}: PE = {} (expected {})", status, file_path, is_pe, expected_pe);
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test string search in PE files
    println!("\n=== String Search Test ===");
    
    let exe_files = [
        "tests/data/Gitify.Setup.6.3.0.exe",
        "tests/data/VSCodeSetup-x64-1.100.0.exe", 
        "tests/data/pycharm-2025.1.1.1.exe",
    ];
    
    for file_path in &exe_files {
        let path = Path::new(file_path);
        if path.exists() {
            println!("  Searching in {}:", file_path);
            
            // Test NSIS patterns
            let nsis_patterns = ["Nullsoft.NSIS.exehead", "NullsoftInst", "NSIS Error"];
            match common::search_file_content(path, &nsis_patterns).await {
                Ok(matches) => {
                    if !matches.is_empty() {
                        println!("    ✓ NSIS patterns found: {:?}", matches);
                    } else {
                        println!("    - No NSIS patterns found");
                    }
                }
                Err(e) => {
                    println!("    ✗ NSIS search error: {}", e);
                }
            }
            
            // Test InnoSetup patterns
            let inno_patterns = ["Inno Setup Setup Data", "JR.Inno.Setup", "InnoSetupVersion"];
            match common::search_file_content(path, &inno_patterns).await {
                Ok(matches) => {
                    if !matches.is_empty() {
                        println!("    ✓ InnoSetup patterns found: {:?}", matches);
                    } else {
                        println!("    - No InnoSetup patterns found");
                    }
                }
                Err(e) => {
                    println!("    ✗ InnoSetup search error: {}", e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    // Test format detection
    println!("\n=== Format Detection Test ===");
    
    let all_files = [
        "tests/data/ArtFlow-1.5.6.msi",
        "tests/data/Gitify.Setup.6.3.0.exe",
        "tests/data/VSCodeSetup-x64-1.100.0.exe",
        "tests/data/pycharm-2025.1.1.1.exe",
    ];
    
    for file_path in &all_files {
        let path = Path::new(file_path);
        if path.exists() {
            match common::detect_installer_format(path).await {
                Ok(format) => {
                    println!("  ✓ {}: Format = {:?}", file_path, format);
                }
                Err(e) => {
                    println!("  ✗ {}: Error = {}", file_path, e);
                }
            }
        } else {
            println!("  - {}: File not found", file_path);
        }
    }
    
    println!("\nTest completed!");
    Ok(())
}
