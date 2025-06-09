//! Error handling for the installer analyzer

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for the installer analyzer
#[derive(Error, Debug)]
pub enum AnalyzerError {
    /// I/O related errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// File format not supported
    #[error("Analysis Error: Unsupported file format - {format}")]
    UnsupportedFormat { format: String },

    /// File not found
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    /// Invalid file format or corrupted data
    #[error("Invalid file format: {reason}")]
    InvalidFormat { reason: String },

    /// Parsing error
    #[error("Parse error: {message}")]
    ParseError { message: String },

    /// Windows API error
    #[error("Windows API error: {message}")]
    WindowsApiError { message: String },

    /// Sandbox operation error
    #[error("Sandbox error: {message}")]
    SandboxError { message: String },

    /// Permission denied
    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },

    /// Process injection error
    #[error("Process injection failed: {reason}")]
    InjectionError { reason: String },

    /// Hook installation error
    #[error("Hook installation failed: {api_name}")]
    HookError { api_name: String },

    /// Timeout error
    #[error("Operation timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError { message: String },

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Generic error with context
    #[error("Error: {message}")]
    Generic { message: String },
}

impl AnalyzerError {
    /// Create a new unsupported format error
    pub fn unsupported_format(format: impl Into<String>) -> Self {
        Self::UnsupportedFormat {
            format: format.into(),
        }
    }

    /// Create a new file not found error
    pub fn file_not_found(path: impl Into<PathBuf>) -> Self {
        Self::FileNotFound { path: path.into() }
    }

    /// Create a new invalid format error
    pub fn invalid_format(reason: impl Into<String>) -> Self {
        Self::InvalidFormat {
            reason: reason.into(),
        }
    }

    /// Create a new parse error
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::ParseError {
            message: message.into(),
        }
    }

    /// Create a new Windows API error
    pub fn windows_api_error(message: impl Into<String>) -> Self {
        Self::WindowsApiError {
            message: message.into(),
        }
    }

    /// Create a new sandbox error
    pub fn sandbox_error(message: impl Into<String>) -> Self {
        Self::SandboxError {
            message: message.into(),
        }
    }

    /// Create a new permission denied error
    pub fn permission_denied(operation: impl Into<String>) -> Self {
        Self::PermissionDenied {
            operation: operation.into(),
        }
    }

    /// Create a new injection error
    pub fn injection_error(reason: impl Into<String>) -> Self {
        Self::InjectionError {
            reason: reason.into(),
        }
    }

    /// Create a new hook error
    pub fn hook_error(api_name: impl Into<String>) -> Self {
        Self::HookError {
            api_name: api_name.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout(seconds: u64) -> Self {
        Self::Timeout { seconds }
    }

    /// Create a new configuration error
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }

    /// Create a new generic error
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic {
            message: message.into(),
        }
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AnalyzerError>;

/// Convert Windows API errors to AnalyzerError
impl From<windows::core::Error> for AnalyzerError {
    fn from(error: windows::core::Error) -> Self {
        Self::WindowsApiError {
            message: format!("Windows API error: {}", error),
        }
    }
}

/// Convert anyhow errors to AnalyzerError
impl From<anyhow::Error> for AnalyzerError {
    fn from(error: anyhow::Error) -> Self {
        Self::Generic {
            message: error.to_string(),
        }
    }
}
