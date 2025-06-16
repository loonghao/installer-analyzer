//! Version comparison and management for auto-updates

use crate::core::Result;
use semver::Version as SemVer;
use std::fmt;
use std::str::FromStr;

/// Version wrapper that provides additional functionality
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    inner: SemVer,
}

impl Version {
    /// Create a new version from a semver::Version
    pub fn new(version: SemVer) -> Self {
        Self { inner: version }
    }

    /// Parse a version string
    pub fn parse(version_str: &str) -> Result<Self> {
        // Clean up the version string (remove 'v' prefix if present)
        let clean_version = version_str.trim_start_matches('v');

        let semver = SemVer::parse(clean_version).map_err(|e| {
            crate::core::AnalyzerError::parse_error(format!(
                "Invalid version format '{}': {}",
                version_str, e
            ))
        })?;

        Ok(Self::new(semver))
    }

    /// Get the major version number
    pub fn major(&self) -> u64 {
        self.inner.major
    }

    /// Get the minor version number
    pub fn minor(&self) -> u64 {
        self.inner.minor
    }

    /// Get the patch version number
    pub fn patch(&self) -> u64 {
        self.inner.patch
    }

    /// Get the pre-release identifier
    pub fn pre(&self) -> &semver::Prerelease {
        &self.inner.pre
    }

    /// Get the build metadata
    pub fn build(&self) -> &semver::BuildMetadata {
        &self.inner.build
    }

    /// Check if this is a pre-release version
    pub fn is_prerelease(&self) -> bool {
        !self.inner.pre.is_empty()
    }

    /// Check if this version is compatible with another version (same major version)
    pub fn is_compatible_with(&self, other: &Version) -> bool {
        self.inner.major == other.inner.major && self.inner.major > 0
    }

    /// Get the underlying semver::Version
    pub fn as_semver(&self) -> &SemVer {
        &self.inner
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl FromStr for Version {
    type Err = crate::core::AnalyzerError;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

impl From<SemVer> for Version {
    fn from(version: SemVer) -> Self {
        Self::new(version)
    }
}

impl From<Version> for SemVer {
    fn from(version: Version) -> Self {
        version.inner
    }
}

/// Version checker for managing current and available versions
pub struct VersionChecker;

impl VersionChecker {
    /// Create a new version checker
    pub fn new() -> Self {
        Self
    }

    /// Get the current version of the application
    pub fn get_current_version(&self) -> Result<Version> {
        let version_str = env!("CARGO_PKG_VERSION");
        Version::parse(version_str)
    }

    /// Parse a version string into a Version object
    pub fn parse_version(&self, version_str: &str) -> Result<Version> {
        Version::parse(version_str)
    }

    /// Compare two versions
    pub fn compare_versions(&self, current: &Version, latest: &Version) -> VersionComparison {
        use std::cmp::Ordering;

        match current.cmp(latest) {
            Ordering::Less => {
                if latest.major() > current.major() {
                    VersionComparison::MajorUpdate
                } else if latest.minor() > current.minor() {
                    VersionComparison::MinorUpdate
                } else {
                    VersionComparison::PatchUpdate
                }
            }
            Ordering::Equal => VersionComparison::UpToDate,
            Ordering::Greater => VersionComparison::Downgrade,
        }
    }

    /// Check if an update is recommended based on version comparison
    pub fn is_update_recommended(&self, current: &Version, latest: &Version) -> bool {
        match self.compare_versions(current, latest) {
            VersionComparison::MajorUpdate
            | VersionComparison::MinorUpdate
            | VersionComparison::PatchUpdate => true,
            VersionComparison::UpToDate | VersionComparison::Downgrade => false,
        }
    }

    /// Check if an update is a breaking change (major version bump)
    pub fn is_breaking_update(&self, current: &Version, latest: &Version) -> bool {
        matches!(
            self.compare_versions(current, latest),
            VersionComparison::MajorUpdate
        )
    }

    /// Get a human-readable description of the version difference
    pub fn get_update_description(&self, current: &Version, latest: &Version) -> String {
        match self.compare_versions(current, latest) {
            VersionComparison::MajorUpdate => {
                format!(
                    "Major update available: {} → {} (may include breaking changes)",
                    current, latest
                )
            }
            VersionComparison::MinorUpdate => {
                format!(
                    "Minor update available: {} → {} (new features)",
                    current, latest
                )
            }
            VersionComparison::PatchUpdate => {
                format!(
                    "Patch update available: {} → {} (bug fixes)",
                    current, latest
                )
            }
            VersionComparison::UpToDate => {
                format!("You are running the latest version: {}", current)
            }
            VersionComparison::Downgrade => {
                format!(
                    "You are running a newer version: {} (latest: {})",
                    current, latest
                )
            }
        }
    }

    /// Validate that a version string is well-formed
    pub fn validate_version_string(&self, version_str: &str) -> bool {
        Version::parse(version_str).is_ok()
    }
}

impl Default for VersionChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Version comparison result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionComparison {
    /// A major version update is available (breaking changes possible)
    MajorUpdate,
    /// A minor version update is available (new features)
    MinorUpdate,
    /// A patch version update is available (bug fixes)
    PatchUpdate,
    /// Current version is up to date
    UpToDate,
    /// Current version is newer than the latest available
    Downgrade,
}

impl fmt::Display for VersionComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionComparison::MajorUpdate => write!(f, "Major Update"),
            VersionComparison::MinorUpdate => write!(f, "Minor Update"),
            VersionComparison::PatchUpdate => write!(f, "Patch Update"),
            VersionComparison::UpToDate => write!(f, "Up to Date"),
            VersionComparison::Downgrade => write!(f, "Downgrade"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let version = Version::parse("1.2.3").unwrap();
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
        assert!(!version.is_prerelease());
    }

    #[test]
    fn test_version_parsing_with_v_prefix() {
        let version = Version::parse("v1.2.3").unwrap();
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
    }

    #[test]
    fn test_version_comparison() {
        let checker = VersionChecker::new();
        let v1 = Version::parse("1.0.0").unwrap();
        let v2 = Version::parse("1.1.0").unwrap();
        let v3 = Version::parse("2.0.0").unwrap();

        assert_eq!(
            checker.compare_versions(&v1, &v2),
            VersionComparison::MinorUpdate
        );
        assert_eq!(
            checker.compare_versions(&v1, &v3),
            VersionComparison::MajorUpdate
        );
        assert_eq!(
            checker.compare_versions(&v2, &v1),
            VersionComparison::Downgrade
        );
    }
}
