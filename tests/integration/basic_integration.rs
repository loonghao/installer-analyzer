use installer_analyzer::analyzers::AnalyzerFactory;
use installer_analyzer::core::types::InstallerFormat;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analyzer_factory_with_real_files() {
        let test_files = vec![
            ("tests/data/ArtFlow-1.5.6.msi", Some(InstallerFormat::WiX)), // This MSI is actually WiX-based
            ("tests/data/persistent_ssh_agent-0.9.0-py3-none-any.whl", Some(InstallerFormat::PythonWheel)),
            ("tests/data/Gitify.Setup.6.3.0.exe", Some(InstallerFormat::NSIS)),
        ];
        
        for (file_path, expected_format) in test_files {
            let path = Path::new(file_path);
            
            if !path.exists() {
                println!("Skipping test - file not found: {}", file_path);
                continue;
            }
            
            match AnalyzerFactory::create_analyzer(path).await {
                Ok(analyzer) => {
                    if let Some(expected) = expected_format {
                        assert_eq!(analyzer.format(), expected, "Wrong format for {}", file_path);
                        
                        // Test that the analyzer can analyze the file
                        let can_analyze = analyzer.can_analyze(path).await.unwrap();
                        assert!(can_analyze, "Analyzer should be able to analyze {}", file_path);
                        
                        // Test basic analysis
                        let result = analyzer.analyze(path).await;
                        assert!(result.is_ok(), "Analysis should succeed for {}", file_path);
                        
                        let (metadata, files, registry_ops) = result.unwrap();
                        assert_eq!(metadata.format, expected, "Metadata format mismatch for {}", file_path);
                        
                        // Basic sanity checks
                        assert!(metadata.file_size > 0, "File size should be greater than 0");
                        assert!(!metadata.file_hash.is_empty(), "File hash should not be empty");
                        
                        println!("✓ Successfully analyzed {} as {:?}", file_path, expected);
                        println!("  - Files extracted: {}", files.len());
                        println!("  - Registry operations: {}", registry_ops.len());
                    }
                }
                Err(e) => {
                    if expected_format.is_some() {
                        panic!("Expected to create analyzer for {} but got error: {}", file_path, e);
                    } else {
                        println!("✓ Correctly rejected unsupported file: {}", file_path);
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_analyzer_factory_unsupported_files() {
        let unsupported_files = vec![
            "Cargo.toml",
            "README.md",
        ];
        
        for file_path in unsupported_files {
            let path = Path::new(file_path);
            
            if !path.exists() {
                continue;
            }
            
            let result = AnalyzerFactory::create_analyzer(path).await;
            assert!(result.is_err(), "Should reject unsupported file: {}", file_path);
            println!("✓ Correctly rejected unsupported file: {}", file_path);
        }
    }

    #[tokio::test]
    async fn test_analyzer_factory_nonexistent_file() {
        let result = AnalyzerFactory::create_analyzer(Path::new("nonexistent_file.exe")).await;
        assert!(result.is_err(), "Should reject nonexistent file");
    }

    #[tokio::test]
    async fn test_get_supported_formats() {
        let formats = AnalyzerFactory::get_supported_formats();
        assert!(!formats.is_empty(), "Should have supported formats");
        
        // Check that common formats are supported
        assert!(formats.contains(&InstallerFormat::MSI));
        assert!(formats.contains(&InstallerFormat::NSIS));
        assert!(formats.contains(&InstallerFormat::PythonWheel));
        
        println!("✓ Supported formats: {:?}", formats);
    }

    #[tokio::test]
    async fn test_get_analyzer_by_format() {
        let test_formats = vec![
            InstallerFormat::MSI,
            InstallerFormat::NSIS,
            InstallerFormat::PythonWheel,
            InstallerFormat::InnoSetup,
        ];
        
        for format in test_formats {
            let analyzer = AnalyzerFactory::get_analyzer_by_format(format);
            assert!(analyzer.is_some(), "Should have analyzer for format: {:?}", format);
            
            let analyzer = analyzer.unwrap();
            assert_eq!(analyzer.format(), format, "Analyzer format should match requested format");
            
            println!("✓ Got analyzer for format: {:?}", format);
        }
        
        // Test unsupported format
        let analyzer = AnalyzerFactory::get_analyzer_by_format(InstallerFormat::Unknown);
        assert!(analyzer.is_none(), "Should not have analyzer for Unknown format");
    }

    #[tokio::test]
    async fn test_get_all_analyzers() {
        let analyzers = AnalyzerFactory::get_all_analyzers();
        assert!(!analyzers.is_empty(), "Should have analyzers");
        
        // Check that we have analyzers for major formats
        let formats: Vec<InstallerFormat> = analyzers.iter().map(|a| a.format()).collect();
        assert!(formats.contains(&InstallerFormat::MSI));
        assert!(formats.contains(&InstallerFormat::NSIS));
        assert!(formats.contains(&InstallerFormat::PythonWheel));
        
        println!("✓ Got {} analyzers", analyzers.len());
        for analyzer in &analyzers {
            println!("  - {:?}", analyzer.format());
        }
    }
}
