# Chocolatey Package for Installer Analyzer

This directory contains the Chocolatey package configuration for installer-analyzer.

## Package Structure

```
chocolatey/
├── installer-analyzer.nuspec          # Package specification
├── tools/
│   ├── chocolateyinstall.ps1         # Installation script
│   ├── chocolateyuninstall.ps1       # Uninstallation script
│   └── chocolateybeforemodify.ps1    # Pre-modification script
└── README.md                          # This file
```

## Package Information

- **Package ID**: `installer-analyzer`
- **Maintainer**: loonghao
- **License**: MIT
- **Project URL**: https://github.com/loonghao/installer-analyzer
- **Tags**: installer, analysis, windows, msi, nsis, inno-setup, msix, python-wheel, security, cli, tool

## Installation

### For Users

```powershell
# Install from Chocolatey Community Repository
choco install installer-analyzer

# Upgrade to latest version
choco upgrade installer-analyzer

# Uninstall
choco uninstall installer-analyzer
```

### For Developers

```powershell
# Build package locally (from project root)
choco pack chocolatey/installer-analyzer.nuspec --version 0.6.0

# Install local package
choco install installer-analyzer --source .

# Test installation
installer-analyzer --version
installer-analyzer --help
```

## Package Features

### Installation Process
1. Downloads the latest installer-analyzer.exe from GitHub releases
2. Verifies file integrity using SHA256 checksum
3. Places executable in Chocolatey tools directory
4. Creates automatic shim for global access
5. Verifies installation by running version check

### Automatic Updates
- The installed tool includes built-in update functionality
- Users can check for updates: `installer-analyzer update --check-only`
- Users can install updates: `installer-analyzer update`
- Chocolatey package updates are handled separately through `choco upgrade`

### Clean Uninstallation
- Removes the main executable
- Cleans up temporary update files
- Automatically removes Chocolatey shim
- Stops any running processes gracefully

## CI/CD Integration

This package is automatically built and published through GitHub Actions:

1. **Release Trigger**: New GitHub release is created
2. **Package Build**: CI downloads release assets and builds Chocolatey package
3. **Checksum Calculation**: SHA256 checksums are calculated and embedded
4. **Package Publishing**: Package is pushed to Chocolatey Community Repository
5. **Verification**: Installation is tested in clean environment

## Package Validation

The package includes several validation steps:

### Pre-installation
- Validates download URL accessibility
- Verifies file integrity with checksums
- Checks Windows compatibility

### Post-installation
- Verifies executable exists and is functional
- Tests basic command execution
- Confirms shim creation

### Pre-modification
- Gracefully stops running processes
- Waits for file handles to be released
- Prepares for clean upgrade/uninstall

## Troubleshooting

### Common Issues

**Installation fails with "file not found"**
- Check internet connectivity
- Verify GitHub releases are accessible
- Try installing with `--force` flag

**Executable not found after installation**
- Restart command prompt/PowerShell
- Check PATH environment variable
- Verify Chocolatey shims directory is in PATH

**Update conflicts during upgrade**
- Close all installer-analyzer instances
- Wait a few seconds and retry
- Use `choco uninstall` then `choco install` if needed

### Getting Help

- **Package Issues**: Report at [GitHub Issues](https://github.com/loonghao/installer-analyzer/issues)
- **Chocolatey Issues**: Check [Chocolatey Documentation](https://docs.chocolatey.org/)
- **Tool Usage**: See main [README](../README.md)

## Development Notes

### Package Maintenance

When updating the package:

1. Update version in `installer-analyzer.nuspec`
2. Test installation scripts with new version
3. Verify all URLs and checksums are correct
4. Test upgrade path from previous version
5. Submit to Chocolatey Community Repository

### Script Guidelines

- Use `$ErrorActionPreference = 'Stop'` for fail-fast behavior
- Provide informative output messages
- Handle errors gracefully with try-catch blocks
- Follow Chocolatey PowerShell conventions
- Test scripts in isolated environments

### Security Considerations

- All downloads use HTTPS
- SHA256 checksums verify file integrity
- No elevated privileges required for normal operation
- Scripts follow Chocolatey security best practices
