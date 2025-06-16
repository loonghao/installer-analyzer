# Auto-Update Guide

This guide explains how to use the auto-update functionality in installer-analyzer.

## Overview

installer-analyzer includes built-in auto-update functionality that allows you to:
- Check for new versions automatically
- Download and install updates with a single command
- Keep your tool up-to-date with the latest features and security fixes

## Quick Start

### Check for Updates

```bash
# Check if updates are available (no installation)
installer-analyzer update --check-only
```

This command will:
- Connect to GitHub to check for the latest release
- Compare with your current version
- Display version information and release notes
- Exit without installing anything

### Install Updates

```bash
# Install available updates
installer-analyzer update
```

This command will:
- Check for updates
- Prompt for confirmation (unless `--yes` is used)
- Download the latest version
- Replace the current executable
- Restart with the new version

### Force Update

```bash
# Force update even if current version is newer
installer-analyzer update --force
```

Use this option to:
- Downgrade to a specific version
- Reinstall the current version
- Override version checks

### Silent Update

```bash
# Update without prompts (for automation)
installer-analyzer update --yes
```

Perfect for:
- Automated scripts
- CI/CD pipelines
- Scheduled updates

## Command Options

| Option | Description | Example |
|--------|-------------|---------|
| `--check-only` | Only check for updates, don't install | `installer-analyzer update --check-only` |
| `--force` | Force update even if current version is newer | `installer-analyzer update --force` |
| `--yes`, `-y` | Skip confirmation prompts | `installer-analyzer update -y` |

## Update Process

### 1. Version Check
- Connects to GitHub API
- Retrieves latest release information
- Compares versions using semantic versioning
- Shows current vs. latest version

### 2. Download
- Downloads the Windows executable from GitHub Releases
- Verifies file integrity with SHA256 checksum
- Shows download progress with speed and ETA

### 3. Installation
- **Windows**: Uses platform-specific update strategy
  - Direct replacement if possible
  - In-place update with temporary files
  - Elevation prompt if required

### 4. Verification
- Verifies the new executable works correctly
- Displays success message with new version

## Update Strategies (Windows)

installer-analyzer uses different update strategies based on your environment:

### Direct Update
- **When**: Executable is in a writable location
- **Process**: Direct file replacement
- **Requirements**: Write permissions to executable directory

### In-Place Update
- **When**: Limited write permissions
- **Process**: Staged update with temporary files
- **Requirements**: Temporary directory access

### Require Elevation
- **When**: Executable is in protected location (e.g., Program Files)
- **Process**: Prompts for administrator privileges
- **Requirements**: User approval for elevation

## Troubleshooting

### Common Issues

**"Permission denied" error**
```
Solution: Run as administrator or move executable to user directory
Command: Right-click Command Prompt → "Run as administrator"
```

**"Network error" or "Connection failed"**
```
Solution: Check internet connection and GitHub accessibility
Test: Try accessing https://github.com/loonghao/installer-analyzer/releases
```

**"Update failed" after download**
```
Solution: Close all instances of installer-analyzer and retry
Command: taskkill /f /im installer-analyzer.exe
```

**"Version check failed"**
```
Solution: Verify GitHub API is accessible
Test: curl https://api.github.com/repos/loonghao/installer-analyzer/releases/latest
```

### Debug Information

Enable verbose output for troubleshooting:

```bash
# Show detailed update process
installer-analyzer update --verbose

# Check current version and build info
installer-analyzer --version

# Verify executable location and permissions
where installer-analyzer
```

## Security Considerations

### File Integrity
- All downloads are verified with SHA256 checksums
- Files are downloaded over HTTPS from GitHub
- No third-party servers or mirrors are used

### Digital Signatures
- Updates preserve any existing digital signatures
- Verification can be enabled with `--verify-signatures` (future feature)

### Network Security
- Uses GitHub's official API and CDN
- No sensitive information is transmitted
- Rate limiting respects GitHub's API limits

## Automation and CI/CD

### Scheduled Updates

**Windows Task Scheduler:**
```powershell
# Create scheduled task for weekly updates
schtasks /create /tn "installer-analyzer-update" /tr "installer-analyzer update --yes" /sc weekly
```

**PowerShell Script:**
```powershell
# update-installer-analyzer.ps1
try {
    $result = & installer-analyzer update --check-only
    if ($LASTEXITCODE -eq 0 -and $result -match "update available") {
        Write-Host "Installing update..."
        & installer-analyzer update --yes
    } else {
        Write-Host "No updates available"
    }
} catch {
    Write-Error "Update check failed: $_"
}
```

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Update installer-analyzer
  run: |
    installer-analyzer update --check-only
    if [ $? -eq 0 ]; then
      installer-analyzer update --yes
    fi
```

## Alternative Update Methods

### Chocolatey (Recommended)

If you installed via Chocolatey, use Chocolatey for updates:

```powershell
# Update via Chocolatey
choco upgrade installer-analyzer

# Check for outdated packages
choco outdated
```

### Manual Update

1. Download from [GitHub Releases](https://github.com/loonghao/installer-analyzer/releases)
2. Replace the existing executable
3. Verify with `installer-analyzer --version`

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `INSTALLER_ANALYZER_UPDATE_CHECK` | Enable/disable update checks | `true` |
| `INSTALLER_ANALYZER_UPDATE_URL` | Custom update server URL | GitHub API |
| `INSTALLER_ANALYZER_TEMP_DIR` | Temporary directory for updates | System temp |

### Update Settings

Currently, update settings are built-in. Future versions may support:
- Custom update channels (stable, beta, nightly)
- Update frequency configuration
- Proxy server support
- Custom download mirrors

## Best Practices

1. **Regular Updates**: Check for updates weekly or monthly
2. **Backup**: Keep a backup of working versions for critical environments
3. **Testing**: Test updates in non-production environments first
4. **Monitoring**: Monitor update logs in automated environments
5. **Rollback**: Know how to rollback if an update causes issues

## Support

For update-related issues:
- Check this guide first
- Review [GitHub Issues](https://github.com/loonghao/installer-analyzer/issues)
- Create a new issue with update logs and system information
- Include output from `installer-analyzer --version` and `installer-analyzer update --check-only`
