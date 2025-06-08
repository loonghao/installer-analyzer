//! Utility functions and helpers

use crate::core::Result;
use std::path::Path;

/// Initialize logging system
pub fn init_logging(verbose: bool) -> Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let filter = if verbose {
        "installer_analyzer=debug,info"
    } else {
        "installer_analyzer=info,warn,error"
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(filter)),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}

/// Check if running with administrator privileges
pub fn is_admin() -> bool {
    #[cfg(windows)]
    {
        use windows::Win32::Foundation::{CloseHandle, HANDLE};
        use windows::Win32::Security::{
            GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
        };
        use windows::Win32::System::Threading::GetCurrentProcess;
        use windows::Win32::System::Threading::OpenProcessToken;

        unsafe {
            let mut token: HANDLE = HANDLE::default();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).as_bool() {
                let mut elevation = TOKEN_ELEVATION::default();
                let mut size = 0u32;

                let result = GetTokenInformation(
                    token,
                    TokenElevation,
                    Some(&mut elevation as *mut _ as *mut _),
                    std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                    &mut size,
                );

                if result.as_bool() {
                    let _ = CloseHandle(token).ok();
                    return elevation.TokenIsElevated != 0;
                }
                let _ = CloseHandle(token).ok();
            }
        }
    }

    false
}

/// Validate that a path exists and is accessible
pub async fn validate_path(path: &Path, must_be_file: bool) -> Result<()> {
    if !path.exists() {
        return Err(crate::core::AnalyzerError::file_not_found(path));
    }

    if must_be_file && !path.is_file() {
        return Err(crate::core::AnalyzerError::invalid_format(format!(
            "Path is not a file: {}",
            path.display()
        )));
    }

    Ok(())
}

/// Format file size in human-readable format
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Format duration in human-readable format
pub fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis();

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}.{:03}s", seconds, millis)
    } else {
        format!("{}ms", millis)
    }
}
