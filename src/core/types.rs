//! Core type definitions for the installer analyzer

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// Supported installer formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallerFormat {
    /// Microsoft Installer format
    MSI,
    /// Nullsoft Scriptable Install System
    NSIS,
    /// Inno Setup installer
    InnoSetup,
    /// Windows Installer XML
    WiX,
    /// InstallShield installer
    InstallShield,
    /// Python Wheel package
    PythonWheel,
    /// MSIX/AppX package format
    MSIX,
    /// Squirrel installer (Electron apps)
    Squirrel,
    /// Unknown or unsupported format
    Unknown,
}

/// Installer metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerMetadata {
    /// Installer format
    pub format: InstallerFormat,
    /// Product name
    pub product_name: Option<String>,
    /// Product version
    pub product_version: Option<String>,
    /// Manufacturer/Publisher
    pub manufacturer: Option<String>,
    /// File size in bytes
    pub file_size: u64,
    /// File hash (SHA-256)
    pub file_hash: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// File entry in an installer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Relative path within the installer
    pub path: PathBuf,
    /// Target installation path
    pub target_path: Option<PathBuf>,
    /// File size in bytes
    pub size: u64,
    /// File hash
    pub hash: Option<String>,
    /// File attributes
    pub attributes: FileAttributes,
    /// Compression method used
    pub compression: Option<String>,
}

/// File attributes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileAttributes {
    pub readonly: bool,
    pub hidden: bool,
    pub system: bool,
    pub executable: bool,
}

/// Registry operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryOperation {
    CreateKey {
        key_path: String,
        timestamp: DateTime<Utc>,
    },
    SetValue {
        key_path: String,
        value_name: String,
        value_type: RegistryValueType,
        value_data: RegistryValue,
        timestamp: DateTime<Utc>,
    },
    DeleteKey {
        key_path: String,
        timestamp: DateTime<Utc>,
    },
    DeleteValue {
        key_path: String,
        value_name: String,
        timestamp: DateTime<Utc>,
    },
}

/// Registry value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryValueType {
    String,
    ExpandString,
    Binary,
    DWord,
    QWord,
    MultiString,
}

/// Registry value data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegistryValue {
    String(String),
    Binary(Vec<u8>),
    DWord(u32),
    QWord(u64),
    MultiString(Vec<String>),
}

/// File system operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperation {
    Create {
        path: PathBuf,
        size: u64,
        timestamp: DateTime<Utc>,
    },
    Write {
        path: PathBuf,
        bytes_written: u64,
        timestamp: DateTime<Utc>,
    },
    Delete {
        path: PathBuf,
        timestamp: DateTime<Utc>,
    },
    Move {
        from_path: PathBuf,
        to_path: PathBuf,
        timestamp: DateTime<Utc>,
    },
    SetAttributes {
        path: PathBuf,
        attributes: FileAttributes,
        timestamp: DateTime<Utc>,
    },
}

/// Process operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOperation {
    pub operation_type: ProcessOpType,
    pub process_id: u32,
    pub process_name: String,
    pub command_line: Option<String>,
    pub parent_process_id: Option<u32>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessOpType {
    Create,
    Terminate,
}

/// Network operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOperation {
    pub operation_type: NetworkOpType,
    pub local_address: String,
    pub remote_address: String,
    pub protocol: String,
    pub bytes_transferred: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkOpType {
    Connect,
    Send,
    Receive,
    Disconnect,
}

/// Complete analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Unique analysis session ID
    pub session_id: Uuid,
    /// Source file path (for basename extraction)
    pub source_file_path: Option<PathBuf>,
    /// Installer metadata
    pub metadata: InstallerMetadata,
    /// Extracted files
    pub files: Vec<FileEntry>,
    /// Registry operations (from static analysis or dynamic monitoring)
    pub registry_operations: Vec<RegistryOperation>,
    /// File operations (from dynamic monitoring)
    pub file_operations: Vec<FileOperation>,
    /// Process operations (from dynamic monitoring)
    pub process_operations: Vec<ProcessOperation>,
    /// Network operations (from dynamic monitoring)
    pub network_operations: Vec<NetworkOperation>,
    /// Analysis timestamp
    pub analyzed_at: DateTime<Utc>,
    /// Analysis duration
    pub analysis_duration: std::time::Duration,
    /// Whether dynamic analysis was performed
    pub dynamic_analysis: bool,
}

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Enable network monitoring
    pub enable_network: bool,
    /// Maximum execution time
    pub max_execution_time: std::time::Duration,
    /// Maximum number of file operations to record
    pub max_file_operations: usize,
    /// Paths allowed for installation
    pub allowed_paths: Vec<PathBuf>,
    /// Paths blocked from access
    pub blocked_paths: Vec<PathBuf>,
    /// Enable detailed logging
    pub verbose_logging: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enable_network: false,
            max_execution_time: std::time::Duration::from_secs(300), // 5 minutes
            max_file_operations: 10000,
            allowed_paths: vec![
                PathBuf::from("C:\\Program Files"),
                PathBuf::from("C:\\Program Files (x86)"),
                PathBuf::from("C:\\ProgramData"),
            ],
            blocked_paths: vec![
                PathBuf::from("C:\\Windows\\System32"),
                PathBuf::from("C:\\Windows\\SysWOW64"),
            ],
            verbose_logging: false,
        }
    }
}
