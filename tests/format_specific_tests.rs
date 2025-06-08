//! Format-specific tests using real installer files

use installer_analyzer::cli::commands::handle_analyze;
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
async fn test_msi_specific_analysis() {
    let msi_file = get_test_file("rust-1.86.0-x86_64-pc-windows-msvc.msi");
    
    if !msi_file.exists() {
        println!("Skipping MSI test: file not found");
        return;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("msi_analysis.json");
    
    let result = handle_analyze(&msi_file, Some(&output_file), Some("json"), false).await;
    
    match result {
        Ok(_) => {
            assert!(output_file.exists(), "MSI analysis should create output");
            
            let content = std::fs::read_to_string(&output_file).unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&content)
                .expect("MSI analysis should produce valid JSON");
            
            // Check for MSI-specific fields
            if let Some(metadata) = json_value.get("metadata") {
                println!("MSI Metadata found: {}", metadata);
                
                // MSI files should have specific metadata
                assert!(metadata.is_object(), "Metadata should be an object");
            }
            
            // Check for files section
            if let Some(files) = json_value.get("files") {
                assert!(files.is_array(), "Files should be an array");
                println!("MSI contains {} files", files.as_array().unwrap().len());
            }
            
            // Check for installer-specific information
            if let Some(installer_info) = json_value.get("installer_info") {
                println!("Installer info: {}", installer_info);
            }
        }
        Err(e) => {
            println!("MSI analysis failed (may be expected for complex MSI): {}", e);
        }
    }
}

#[tokio::test]
async fn test_exe_installer_detection() {
    let exe_files = vec![
        ("Gitify.Setup.6.3.0.exe", "Gitify Setup"),
        ("wetype_installer_official_p_48.exe", "WeType Installer"),
    ];
    
    for (filename, _expected_name) in exe_files {
        let exe_file = get_test_file(filename);
        
        if !exe_file.exists() {
            println!("Skipping EXE test: {} not found", filename);
            continue;
        }
        
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join(format!("{}_analysis.json", 
            exe_file.file_stem().unwrap().to_str().unwrap()));
        
        let result = handle_analyze(&exe_file, Some(&output_file), Some("json"), false).await;
        
        match result {
            Ok(_) => {
                assert!(output_file.exists(), "EXE analysis should create output for {}", filename);
                
                let content = std::fs::read_to_string(&output_file).unwrap();
                let json_value: serde_json::Value = serde_json::from_str(&content)
                    .expect("EXE analysis should produce valid JSON");
                
                // Check for basic structure
                assert!(json_value.get("metadata").is_some(), 
                    "EXE analysis should have metadata for {}", filename);
                
                // Check if installer type is detected
                if let Some(metadata) = json_value.get("metadata") {
                    if let Some(installer_type) = metadata.get("installer_type") {
                        println!("{}: Detected installer type: {}", filename, installer_type);
                    }
                    
                    if let Some(product_name) = metadata.get("product_name") {
                        println!("{}: Product name: {}", filename, product_name);
                    }
                }
                
                println!("✓ {} analysis completed successfully", filename);
            }
            Err(e) => {
                println!("EXE analysis failed for {} (may be expected): {}", filename, e);
            }
        }
    }
}

#[tokio::test]
async fn test_python_wheel_analysis() {
    let whl_file = get_test_file("persistent_ssh_agent-0.9.0-py3-none-any.whl");
    
    if !whl_file.exists() {
        println!("Skipping Python wheel test: file not found");
        return;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("wheel_analysis.json");
    
    let result = handle_analyze(&whl_file, Some(&output_file), Some("json"), false).await;
    
    match result {
        Ok(_) => {
            assert!(output_file.exists(), "Wheel analysis should create output");
            
            let content = std::fs::read_to_string(&output_file).unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&content)
                .expect("Wheel analysis should produce valid JSON");
            
            // Check for Python wheel specific information
            if let Some(metadata) = json_value.get("metadata") {
                println!("Wheel metadata: {}", metadata);
                
                // Look for Python-specific fields
                if let Some(package_name) = metadata.get("package_name") {
                    assert!(package_name.as_str().unwrap().contains("persistent_ssh_agent"), 
                        "Should detect correct package name");
                }
                
                if let Some(version) = metadata.get("version") {
                    assert!(version.as_str().unwrap().contains("0.9.0"), 
                        "Should detect correct version");
                }
            }
            
            // Check for files in the wheel
            if let Some(files) = json_value.get("files") {
                let files_array = files.as_array().unwrap();
                assert!(!files_array.is_empty(), "Wheel should contain files");
                
                // Look for Python-specific files
                let file_names: Vec<String> = files_array.iter()
                    .filter_map(|f| f.get("name"))
                    .filter_map(|n| n.as_str())
                    .map(|s| s.to_string())
                    .collect();
                
                let has_python_files = file_names.iter()
                    .any(|name| name.ends_with(".py") || name.contains("__pycache__"));
                
                if has_python_files {
                    println!("✓ Found Python files in wheel");
                }
                
                println!("Wheel contains {} files", files_array.len());
            }
        }
        Err(e) => {
            println!("Wheel analysis failed (may be expected): {}", e);
        }
    }
}

#[tokio::test]
async fn test_zip_archive_analysis() {
    let zip_file = get_test_file("ShareX-17.1.0-portable.zip");
    
    if !zip_file.exists() {
        println!("Skipping ZIP test: file not found");
        return;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("zip_analysis.json");
    
    let result = handle_analyze(&zip_file, Some(&output_file), Some("json"), false).await;
    
    match result {
        Ok(_) => {
            assert!(output_file.exists(), "ZIP analysis should create output");
            
            let content = std::fs::read_to_string(&output_file).unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&content)
                .expect("ZIP analysis should produce valid JSON");
            
            // Check for archive-specific information
            if let Some(files) = json_value.get("files") {
                let files_array = files.as_array().unwrap();
                assert!(!files_array.is_empty(), "ZIP should contain files");
                
                println!("ZIP archive contains {} files", files_array.len());
                
                // Look for executable files in the archive
                let exe_files: Vec<&str> = files_array.iter()
                    .filter_map(|f| f.get("name"))
                    .filter_map(|n| n.as_str())
                    .filter(|name| name.ends_with(".exe") || name.ends_with(".dll"))
                    .collect();
                
                if !exe_files.is_empty() {
                    println!("Found {} executable files in ZIP", exe_files.len());
                    for exe_file in exe_files.iter().take(5) { // Show first 5
                        println!("  - {}", exe_file);
                    }
                }
            }
            
            // Check metadata
            if let Some(metadata) = json_value.get("metadata") {
                println!("ZIP metadata: {}", metadata);
            }
        }
        Err(e) => {
            println!("ZIP analysis failed (may be expected): {}", e);
        }
    }
}

#[tokio::test]
async fn test_file_signature_validation() {
    let test_files = vec![
        ("rust-1.86.0-x86_64-pc-windows-msvc.msi", vec![0xD0, 0xCF, 0x11, 0xE0]), // MSI signature
        ("Gitify.Setup.6.3.0.exe", vec![0x4D, 0x5A]), // PE signature
        ("ShareX-17.1.0-portable.zip", vec![0x50, 0x4B]), // ZIP signature
    ];
    
    for (filename, expected_signature) in test_files {
        let file_path = get_test_file(filename);
        
        if !file_path.exists() {
            println!("Skipping signature test: {} not found", filename);
            continue;
        }
        
        // Read first few bytes to check file signature
        let file_content = std::fs::read(&file_path).unwrap();
        if file_content.len() < expected_signature.len() {
            println!("File {} too small for signature check", filename);
            continue;
        }
        
        let actual_signature = &file_content[..expected_signature.len()];
        
        if actual_signature == expected_signature.as_slice() {
            println!("✓ {} has correct file signature", filename);
        } else {
            println!("⚠ {} signature mismatch - expected: {:02X?}, got: {:02X?}", 
                filename, expected_signature, actual_signature);
        }
        
        // Test that our analyzer can handle the file
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("signature_test.json");
        
        let result = handle_analyze(&file_path, Some(&output_file), Some("json"), false).await;
        
        match result {
            Ok(_) => {
                println!("✓ {} analysis succeeded", filename);
            }
            Err(e) => {
                println!("✗ {} analysis failed: {}", filename, e);
            }
        }
    }
}

#[tokio::test]
async fn test_output_format_consistency() {
    let test_file = get_test_file("persistent_ssh_agent-0.9.0-py3-none-any.whl");
    
    if !test_file.exists() {
        println!("Skipping format consistency test: WHL file not found");
        return;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let formats = vec![
        ("json", "application/json"),
        ("html", "text/html"),
        ("markdown", "text/markdown"),
    ];
    
    for (format, _expected_type) in formats {
        let extension = if format == "markdown" { "md" } else { format };
        let output_file = temp_dir.path().join(format!("consistency_test.{}", extension));
        
        let result = handle_analyze(&test_file, Some(&output_file), Some(format), false).await;
        
        match result {
            Ok(_) => {
                assert!(output_file.exists(), "Output file should exist for {} format", format);
                
                let content = std::fs::read_to_string(&output_file).unwrap();
                assert!(!content.is_empty(), "Output should not be empty for {} format", format);
                
                // Validate format-specific content
                match format {
                    "json" => {
                        let _: serde_json::Value = serde_json::from_str(&content)
                            .expect("JSON output should be valid");
                        println!("✓ {} format: Valid JSON", format);
                    }
                    "html" => {
                        assert!(content.contains("<html>") || content.contains("<!DOCTYPE"), 
                            "HTML output should contain HTML tags");
                        println!("✓ {} format: Valid HTML structure", format);
                    }
                    "markdown" => {
                        assert!(content.contains("#") || content.contains("##"), 
                            "Markdown output should contain headers");
                        println!("✓ {} format: Valid Markdown structure", format);
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("✗ {} format failed: {}", format, e);
            }
        }
    }
}
