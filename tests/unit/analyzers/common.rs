use installer_analyzer::analyzers::common;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_pe_file_with_valid_pe() {
        // Create a minimal PE file header
        let mut temp_file = NamedTempFile::new().unwrap();

        // Write DOS header with PE signature
        let dos_header = b"MZ\x90\x00\x03\x00\x00\x00\x04\x00\x00\x00\xFF\xFF\x00\x00\xB8\x00\x00\x00\x00\x00\x00\x00\x40\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80\x00\x00\x00";
        temp_file.write_all(dos_header).unwrap();

        // Pad to offset 0x80 and write PE signature
        let padding = vec![0u8; 0x80 - dos_header.len()];
        temp_file.write_all(&padding).unwrap();
        temp_file.write_all(b"PE\x00\x00").unwrap();

        temp_file.flush().unwrap();

        let result = common::is_pe_file(temp_file.path()).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_is_pe_file_with_invalid_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Not a PE file").unwrap();
        temp_file.flush().unwrap();

        let result = common::is_pe_file(temp_file.path()).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_is_pe_file_with_nonexistent_file() {
        let result = common::is_pe_file(Path::new("nonexistent_file.exe")).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_file_content_found() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file
            .write_all(b"This is a test file with NSIS content and some other data")
            .unwrap();
        temp_file.flush().unwrap();

        let patterns = ["NSIS", "test"];
        let result = common::search_file_content(temp_file.path(), &patterns).await;
        assert!(result.is_ok());

        let matches = result.unwrap();
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&"NSIS".to_string()));
        assert!(matches.contains(&"test".to_string()));
    }

    #[tokio::test]
    async fn test_search_file_content_not_found() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"This is a simple text file").unwrap();
        temp_file.flush().unwrap();

        let patterns = ["NSIS", "InnoSetup"];
        let result = common::search_file_content(temp_file.path(), &patterns).await;
        assert!(result.is_ok());

        let matches = result.unwrap();
        assert!(matches.is_empty());
    }

    #[tokio::test]
    async fn test_search_file_content_case_sensitive() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file
            .write_all(b"This file contains nsis in lowercase")
            .unwrap();
        temp_file.flush().unwrap();

        let patterns = ["NSIS"];
        let result = common::search_file_content(temp_file.path(), &patterns).await;
        assert!(result.is_ok());

        let matches = result.unwrap();
        // Should not find NSIS in lowercase text (case sensitive)
        assert_eq!(matches.len(), 0);
    }

    #[tokio::test]
    async fn test_search_file_content_with_nonexistent_file() {
        let patterns = ["test"];
        let result = common::search_file_content(Path::new("nonexistent.txt"), &patterns).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_file_content_empty_patterns() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Some content").unwrap();
        temp_file.flush().unwrap();

        let patterns: [&str; 0] = [];
        let result = common::search_file_content(temp_file.path(), &patterns).await;
        assert!(result.is_ok());

        let matches = result.unwrap();
        assert!(matches.is_empty());
    }

    #[tokio::test]
    async fn test_search_file_content_binary_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let binary_data = vec![0x00, 0x01, 0x02, 0x4E, 0x53, 0x49, 0x53, 0x03, 0x04]; // Contains "NSIS"
        temp_file.write_all(&binary_data).unwrap();
        temp_file.flush().unwrap();

        let patterns = ["NSIS"];
        let result = common::search_file_content(temp_file.path(), &patterns).await;
        assert!(result.is_ok());

        let matches = result.unwrap();
        assert_eq!(matches.len(), 1);
        assert!(matches.contains(&"NSIS".to_string()));
    }

    #[tokio::test]
    async fn test_search_file_content_multiple_occurrences() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file
            .write_all(b"NSIS installer with NSIS scripts and NSIS data")
            .unwrap();
        temp_file.flush().unwrap();

        let patterns = ["NSIS"];
        let result = common::search_file_content(temp_file.path(), &patterns).await;
        assert!(result.is_ok());

        let matches = result.unwrap();
        // Should only return unique matches
        assert_eq!(matches.len(), 1);
        assert!(matches.contains(&"NSIS".to_string()));
    }
}
