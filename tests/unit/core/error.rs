use installer_analyzer::core::error::{AnalyzerError, Result};
use std::io;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_error_display() {
        let error = AnalyzerError::unsupported_format("test");
        assert_eq!(error.to_string(), "Unsupported file format: test");
    }

    #[test]
    fn test_analyzer_error_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let analyzer_error: AnalyzerError = io_error.into();

        match analyzer_error {
            AnalyzerError::Io(_) => {},
            _ => panic!("Expected Io variant"),
        }
    }

    #[test]
    fn test_analyzer_error_constructors() {
        let error = AnalyzerError::generic("Custom error message");

        match error {
            AnalyzerError::Generic { message } => assert_eq!(message, "Custom error message"),
            _ => panic!("Expected Generic variant"),
        }
    }

    #[test]
    fn test_result_type_alias() {
        let success: Result<i32> = Ok(42);
        assert_eq!(success.unwrap(), 42);

        let failure: Result<i32> = Err(AnalyzerError::unsupported_format("test"));
        assert!(failure.is_err());
    }

    #[test]
    fn test_analyzer_error_debug() {
        let error = AnalyzerError::unsupported_format("test");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("UnsupportedFormat"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_analyzer_error_source() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let analyzer_error: AnalyzerError = io_error.into();

        assert!(analyzer_error.source().is_some());
    }

    #[test]
    fn test_all_error_variants() {
        let errors = vec![
            AnalyzerError::unsupported_format("format"),
            AnalyzerError::file_not_found("file.txt"),
            AnalyzerError::parse_error("parse"),
            AnalyzerError::sandbox_error("sandbox"),
            AnalyzerError::generic("other"),
        ];

        for error in errors {
            // Ensure all variants can be displayed and debugged
            let _ = error.to_string();
            let _ = format!("{:?}", error);
        }
    }

    #[test]
    fn test_error_constructors() {
        let unsupported = AnalyzerError::unsupported_format("MSI");
        assert!(unsupported.to_string().contains("MSI"));

        let not_found = AnalyzerError::file_not_found("test.exe");
        assert!(not_found.to_string().contains("test.exe"));

        let invalid = AnalyzerError::invalid_format("corrupted");
        assert!(invalid.to_string().contains("corrupted"));

        let parse = AnalyzerError::parse_error("syntax error");
        assert!(parse.to_string().contains("syntax error"));

        let sandbox = AnalyzerError::sandbox_error("injection failed");
        assert!(sandbox.to_string().contains("injection failed"));
    }
}
