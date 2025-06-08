//! System monitoring components

use crate::core::{FileOperation, RegistryOperation, Result};
use async_trait::async_trait;

/// Trait for system monitors
#[async_trait]
pub trait SystemMonitor: Send + Sync {
    /// Start monitoring
    async fn start(&mut self) -> Result<()>;

    /// Stop monitoring and return collected operations
    async fn stop(&mut self) -> Result<()>;

    /// Check if monitoring is active
    fn is_active(&self) -> bool;
}

/// File system monitor (placeholder)
pub struct FileSystemMonitor {
    active: bool,
    operations: Vec<FileOperation>,
}

impl Default for FileSystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemMonitor {
    pub fn new() -> Self {
        Self {
            active: false,
            operations: Vec::new(),
        }
    }

    pub fn get_operations(&self) -> &[FileOperation] {
        &self.operations
    }
}

#[async_trait]
impl SystemMonitor for FileSystemMonitor {
    async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting file system monitoring");
        self.active = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping file system monitoring");
        self.active = false;
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

/// Registry monitor (placeholder)
pub struct RegistryMonitor {
    active: bool,
    operations: Vec<RegistryOperation>,
}

impl Default for RegistryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryMonitor {
    pub fn new() -> Self {
        Self {
            active: false,
            operations: Vec::new(),
        }
    }

    pub fn get_operations(&self) -> &[RegistryOperation] {
        &self.operations
    }
}

#[async_trait]
impl SystemMonitor for RegistryMonitor {
    async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting registry monitoring");
        self.active = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping registry monitoring");
        self.active = false;
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.active
    }
}
