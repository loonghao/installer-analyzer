//! MSI database access using Windows Installer API

use crate::core::{AnalyzerError, Result};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::ApplicationInstallationAndServicing::{
    MsiCloseHandle, MsiDatabaseOpenViewW, MsiOpenDatabaseW, MsiRecordGetInteger,
    MsiRecordGetStringW, MsiViewClose, MsiViewExecute, MsiViewFetch, MSIDBOPEN_READONLY, MSIHANDLE,
};

/// MSI Database wrapper
pub struct MsiDatabase {
    handle: MSIHANDLE,
}

impl MsiDatabase {
    /// Open an MSI database file
    pub fn open(file_path: &Path) -> Result<Self> {
        let path_wide: Vec<u16> = OsStr::new(file_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut handle = MSIHANDLE(0);

        unsafe {
            let result = MsiOpenDatabaseW(
                PCWSTR(path_wide.as_ptr()),
                PCWSTR(MSIDBOPEN_READONLY.0),
                &mut handle,
            );

            if result != ERROR_SUCCESS.0 {
                return Err(AnalyzerError::windows_api_error(format!(
                    "Failed to open MSI database: error code {}",
                    result
                )));
            }
        }

        Ok(Self { handle })
    }

    /// Execute a SQL query on the database
    pub fn execute_query(&self, query: &str) -> Result<MsiView> {
        let query_wide: Vec<u16> = OsStr::new(query)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut view_handle = MSIHANDLE(0);

        unsafe {
            let result =
                MsiDatabaseOpenViewW(self.handle, PCWSTR(query_wide.as_ptr()), &mut view_handle);

            if result != ERROR_SUCCESS.0 {
                return Err(AnalyzerError::windows_api_error(format!(
                    "Failed to open database view: error code {}",
                    result
                )));
            }

            let execute_result = MsiViewExecute(view_handle, MSIHANDLE(0));
            if execute_result != ERROR_SUCCESS.0 {
                MsiCloseHandle(view_handle);
                return Err(AnalyzerError::windows_api_error(format!(
                    "Failed to execute view: error code {}",
                    execute_result
                )));
            }
        }

        Ok(MsiView {
            handle: view_handle,
        })
    }

    /// Get the handle for direct API calls
    pub fn handle(&self) -> MSIHANDLE {
        self.handle
    }
}

impl Drop for MsiDatabase {
    fn drop(&mut self) {
        if self.handle.0 != 0 {
            unsafe {
                MsiCloseHandle(self.handle);
            }
        }
    }
}

/// MSI View wrapper for query results
pub struct MsiView {
    handle: MSIHANDLE,
}

impl MsiView {
    /// Fetch the next record from the view
    pub fn fetch(&self) -> Result<Option<MsiRecord>> {
        let mut record_handle = MSIHANDLE(0);

        unsafe {
            let result = MsiViewFetch(self.handle, &mut record_handle);

            match result {
                259 => Ok(None), // ERROR_NO_MORE_ITEMS
                0 => Ok(Some(MsiRecord {
                    handle: record_handle,
                })), // ERROR_SUCCESS
                _ => Err(AnalyzerError::windows_api_error(format!(
                    "Failed to fetch record: error code {}",
                    result
                ))),
            }
        }
    }

    /// Collect all records from the view
    pub fn collect_records(&self) -> Result<Vec<MsiRecord>> {
        let mut records = Vec::new();

        while let Some(record) = self.fetch()? {
            records.push(record);
        }

        Ok(records)
    }
}

impl Drop for MsiView {
    fn drop(&mut self) {
        if self.handle.0 != 0 {
            unsafe {
                MsiViewClose(self.handle);
                MsiCloseHandle(self.handle);
            }
        }
    }
}

/// MSI Record wrapper
pub struct MsiRecord {
    handle: MSIHANDLE,
}

impl MsiRecord {
    /// Get string value from a field
    pub fn get_string(&self, field: u32) -> Result<String> {
        let mut buffer_size: u32 = 0;

        // First call to get the required buffer size
        unsafe {
            MsiRecordGetStringW(self.handle, field, PWSTR::null(), Some(&mut buffer_size));
        }

        if buffer_size == 0 {
            return Ok(String::new());
        }

        // Allocate buffer and get the actual string
        let mut buffer: Vec<u16> = vec![0; (buffer_size + 1) as usize];
        buffer_size += 1; // Include null terminator

        unsafe {
            let result = MsiRecordGetStringW(
                self.handle,
                field,
                PWSTR(buffer.as_mut_ptr()),
                Some(&mut buffer_size),
            );

            if result != ERROR_SUCCESS.0 {
                return Err(AnalyzerError::windows_api_error(format!(
                    "Failed to get string from record: error code {}",
                    result
                )));
            }
        }

        // Convert to Rust string
        let end = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        let os_string = std::ffi::OsString::from_wide(&buffer[..end]);

        os_string
            .into_string()
            .map_err(|_| AnalyzerError::parse_error("Invalid UTF-8 in MSI string field"))
    }

    /// Get integer value from a field
    pub fn get_integer(&self, field: u32) -> Result<i32> {
        unsafe {
            let value = MsiRecordGetInteger(self.handle, field);
            Ok(value)
        }
    }

    /// Check if a field is null
    pub fn is_null(&self, field: u32) -> bool {
        unsafe {
            let value = MsiRecordGetInteger(self.handle, field);
            value == -2147483648 // MSI_NULL_INTEGER
        }
    }
}

impl Drop for MsiRecord {
    fn drop(&mut self) {
        if self.handle.0 != 0 {
            unsafe {
                MsiCloseHandle(self.handle);
            }
        }
    }
}
