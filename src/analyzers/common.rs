//! Common utilities for analyzers

use crate::core::{Result, AnalyzerError, InstallerFormat};
use std::path::Path;
use sha2::{Sha256, Digest};
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

/// Calculate SHA-256 hash of a file
pub async fn calculate_file_hash(file_path: &Path) -> Result<String> {
    let data = tokio::fs::read(file_path).await?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Get file size
pub async fn get_file_size(file_path: &Path) -> Result<u64> {
    let metadata = tokio::fs::metadata(file_path).await?;
    Ok(metadata.len())
}

/// Check if file exists and is readable
pub async fn validate_file(file_path: &Path) -> Result<()> {
    if !file_path.exists() {
        return Err(AnalyzerError::file_not_found(file_path));
    }

    if !file_path.is_file() {
        return Err(AnalyzerError::invalid_format(
            format!("Path is not a file: {}", file_path.display())
        ));
    }

    // Try to read the first few bytes to ensure it's readable
    let mut file = tokio::fs::File::open(file_path).await?;
    let mut buffer = [0u8; 4];
    use tokio::io::AsyncReadExt;
    file.read(&mut buffer).await?;

    Ok(())
}

/// Detect file format by extension
pub fn detect_format_by_extension(file_path: &Path) -> Option<crate::core::InstallerFormat> {
    match file_path.extension()?.to_str()? {
        "msi" => Some(crate::core::InstallerFormat::MSI),
        "exe" => {
            // Could be NSIS, InnoSetup, or other formats
            // We'll need to analyze the content to determine the exact format
            None
        },
        "whl" => Some(crate::core::InstallerFormat::PythonWheel),
        _ => None,
    }
}

/// Read file header to help with format detection
pub async fn read_file_header(file_path: &Path, size: usize) -> Result<Vec<u8>> {
    let mut file = tokio::fs::File::open(file_path).await?;
    let mut buffer = vec![0u8; size];
    let bytes_read = file.read(&mut buffer).await?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

/// Check if file is a PE (Portable Executable) file by checking MZ signature
pub async fn is_pe_file(file_path: &Path) -> Result<bool> {
    let header = read_file_header(file_path, 2).await?;

    if header.len() >= 2 {
        // PE files start with MZ signature (0x4D5A)
        Ok(header[0] == 0x4D && header[1] == 0x5A)
    } else {
        Ok(false)
    }
}

/// Read a specific range of file content
pub async fn read_file_content_range(file_path: &Path, start: u64, size: usize) -> Result<Vec<u8>> {
    let mut file = tokio::fs::File::open(file_path).await?;
    file.seek(SeekFrom::Start(start)).await?;

    let mut buffer = vec![0u8; size];
    let bytes_read = file.read(&mut buffer).await?;
    buffer.truncate(bytes_read);
    Ok(buffer)
}

/// Search for patterns in file content using chunked reading for memory efficiency
pub async fn search_file_content(file_path: &Path, patterns: &[&str]) -> Result<Vec<String>> {
    const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks
    const OVERLAP_SIZE: usize = 1024; // Overlap to catch patterns across chunk boundaries

    let mut file = tokio::fs::File::open(file_path).await?;
    let file_size = get_file_size(file_path).await?;
    let mut found_patterns = Vec::new();
    let mut position = 0u64;
    let mut overlap_buffer = Vec::new();

    while position < file_size {
        // Calculate chunk size for this iteration
        let remaining = file_size - position;
        let current_chunk_size = std::cmp::min(CHUNK_SIZE, remaining as usize);

        // Read chunk
        let mut chunk = vec![0u8; current_chunk_size];
        file.seek(SeekFrom::Start(position)).await?;
        let bytes_read = file.read(&mut chunk).await?;
        chunk.truncate(bytes_read);

        // Combine with overlap from previous chunk
        let mut search_buffer = overlap_buffer.clone();
        search_buffer.extend_from_slice(&chunk);

        // Convert to string for pattern matching (handle invalid UTF-8 gracefully)
        let search_text = String::from_utf8_lossy(&search_buffer);

        // Search for each pattern
        for pattern in patterns {
            if search_text.contains(pattern) && !found_patterns.contains(&pattern.to_string()) {
                found_patterns.push(pattern.to_string());
                tracing::debug!("Found pattern '{}' at position ~{}", pattern, position);
            }
        }

        // Prepare overlap for next iteration
        if bytes_read == current_chunk_size && (position + current_chunk_size as u64) < file_size {
            let overlap_start = if chunk.len() > OVERLAP_SIZE { chunk.len() - OVERLAP_SIZE } else { 0 };
            overlap_buffer = chunk[overlap_start..].to_vec();
        } else {
            overlap_buffer.clear();
        }

        position += current_chunk_size as u64;

        // Break early if all patterns found
        if found_patterns.len() == patterns.len() {
            break;
        }
    }

    Ok(found_patterns)
}

/// Detect installer format by analyzing file content
pub async fn detect_installer_format(file_path: &Path) -> Result<InstallerFormat> {
    // First try extension-based detection
    if let Some(format) = detect_format_by_extension(file_path) {
        return Ok(format);
    }

    // For .exe files, analyze content to determine the specific format
    if is_pe_file(file_path).await? {
        return detect_pe_installer_format(file_path).await;
    }

    // Default to unknown format
    Err(AnalyzerError::unsupported_format(
        format!("Unable to determine installer format for: {}", file_path.display())
    ))
}

/// Detect specific installer format for PE files
async fn detect_pe_installer_format(file_path: &Path) -> Result<InstallerFormat> {
    // Define patterns for different installer types
    let nsis_patterns = ["Nullsoft.NSIS.exehead", "NullsoftInst", "NSIS Error"];
    let inno_patterns = ["Inno Setup Setup Data", "JR.Inno.Setup", "InnoSetupVersion"];
    let installshield_patterns = ["InstallShield", "InstallScript"];
    let wix_patterns = ["Windows Installer XML", "WiX Toolset"];

    // Search for patterns in the file
    let nsis_matches = search_file_content(file_path, &nsis_patterns).await?;
    let inno_matches = search_file_content(file_path, &inno_patterns).await?;
    let installshield_matches = search_file_content(file_path, &installshield_patterns).await?;
    let wix_matches = search_file_content(file_path, &wix_patterns).await?;

    // Determine format based on found patterns
    if !nsis_matches.is_empty() {
        tracing::info!("Detected NSIS installer: found patterns {:?}", nsis_matches);
        Ok(InstallerFormat::NSIS)
    } else if !inno_matches.is_empty() {
        tracing::info!("Detected InnoSetup installer: found patterns {:?}", inno_matches);
        Ok(InstallerFormat::InnoSetup)
    } else if !installshield_matches.is_empty() {
        tracing::info!("Detected InstallShield installer: found patterns {:?}", installshield_matches);
        Ok(InstallerFormat::InstallShield)
    } else if !wix_matches.is_empty() {
        tracing::info!("Detected WiX installer: found patterns {:?}", wix_matches);
        Ok(InstallerFormat::WiX)
    } else {
        // If no specific patterns found, classify as unknown format
        tracing::warn!("No specific installer patterns found in PE file: {}", file_path.display());
        Ok(InstallerFormat::Unknown)
    }
}

/// Check if file is a supported archive format
pub async fn is_archive_file(file_path: &Path) -> Result<bool> {
    let header = read_file_header(file_path, 8).await?;

    if header.len() >= 4 {
        // ZIP signature: PK (0x504B)
        if header[0] == 0x50 && header[1] == 0x4B {
            return Ok(true);
        }

        // 7z signature: 7z¼¯' (0x377ABCAF271C)
        if header.len() >= 6
            && header[0] == 0x37 && header[1] == 0x7A
            && header[2] == 0xBC && header[3] == 0xAF
            && header[4] == 0x27 && header[5] == 0x1C {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Detect archive format from file header
pub async fn detect_archive_format(file_path: &Path) -> Result<String> {
    let header = read_file_header(file_path, 8).await?;

    if header.len() >= 4 {
        // ZIP signature: PK (0x504B)
        if header[0] == 0x50 && header[1] == 0x4B {
            return Ok("ZIP".to_string());
        }

        // 7z signature: 7z¼¯' (0x377ABCAF271C)
        if header.len() >= 6
            && header[0] == 0x37 && header[1] == 0x7A
            && header[2] == 0xBC && header[3] == 0xAF
            && header[4] == 0x27 && header[5] == 0x1C {
            return Ok("7Z".to_string());
        }
    }

    Err(AnalyzerError::unsupported_format(
        format!("Unknown archive format: {}", file_path.display())
    ))
}
