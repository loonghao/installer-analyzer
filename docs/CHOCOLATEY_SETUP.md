# Chocolatey Package Publishing Setup

This document explains how to set up automatic Chocolatey package publishing for the installer-analyzer project.

## Overview

The project includes automated Chocolatey package publishing through GitHub Actions. When a new release is created, the CI/CD pipeline automatically:

1. Builds the Windows binary
2. Calculates SHA256 checksum
3. Updates Chocolatey package configuration
4. Builds the Chocolatey package
5. Publishes to Chocolatey Community Repository

## Prerequisites

### 1. Chocolatey Community Account

You need a Chocolatey Community account to publish packages:

1. Visit [Chocolatey Community](https://community.chocolatey.org/)
2. Create an account or sign in
3. Navigate to your account settings
4. Generate an API key for package publishing

### 2. GitHub Repository Secrets

Add the following secret to your GitHub repository:

**Secret Name**: `CHOCOLATEY_API_KEY`
**Secret Value**: Your Chocolatey Community API key

#### How to Add GitHub Secrets:

1. Go to your GitHub repository
2. Click on **Settings** tab
3. Navigate to **Secrets and variables** → **Actions**
4. Click **New repository secret**
5. Name: `CHOCOLATEY_API_KEY`
6. Value: Paste your Chocolatey API key
7. Click **Add secret**

## Workflow Configuration

The Chocolatey publishing is integrated into the existing release workflow (`.github/workflows/release-please.yml`):

### Trigger Conditions

- **Automatic**: Triggered when release-please creates a new release
- **Conditional**: Only runs if `CHOCOLATEY_API_KEY` secret is configured
- **Platform**: Runs on Windows runner for compatibility

### Publishing Process

1. **Binary Preparation**:
   - Uses the Windows binary built in the same workflow
   - Calculates SHA256 checksum for integrity verification

2. **Package Configuration**:
   - Updates `chocolatey/installer-analyzer.nuspec` with release version
   - Updates `chocolatey/tools/chocolateyinstall.ps1` with download URL and checksum

3. **Package Building**:
   - Installs Chocolatey CLI on the runner
   - Builds the package using `choco pack`

4. **Package Publishing**:
   - Pushes to Chocolatey Community Repository
   - Uses the configured API key for authentication

## Package Information

- **Package ID**: `installer-analyzer`
- **Repository**: [Chocolatey Community](https://community.chocolatey.org/packages/installer-analyzer)
- **Maintainer**: loonghao
- **License**: MIT

## Manual Publishing (Fallback)

If automatic publishing fails, you can manually publish:

### Prerequisites

1. Install Chocolatey on your local machine
2. Download the release binary
3. Update package configuration manually

### Steps

```powershell
# 1. Clone the repository
git clone https://github.com/loonghao/installer-analyzer.git
cd installer-analyzer

# 2. Download the release binary
$version = "0.6.0"  # Replace with actual version
$url = "https://github.com/loonghao/installer-analyzer/releases/download/v$version/installer-analyzer.exe"
Invoke-WebRequest -Uri $url -OutFile "installer-analyzer.exe"

# 3. Calculate checksum
$checksum = (Get-FileHash -Path "installer-analyzer.exe" -Algorithm SHA256).Hash

# 4. Update nuspec file
$nuspecContent = Get-Content "chocolatey/installer-analyzer.nuspec" -Raw
$nuspecContent = $nuspecContent -replace '\$version\$', $version
Set-Content -Path "chocolatey/installer-analyzer.nuspec" -Value $nuspecContent

# 5. Update install script
$installContent = Get-Content "chocolatey/tools/chocolateyinstall.ps1" -Raw
$installContent = $installContent -replace "checksum64\s*=\s*''", "checksum64 = '$checksum'"
$installContent = $installContent -replace 'url64bit\s*=\s*"[^"]*"', "url64bit = '$url'"
Set-Content -Path "chocolatey/tools/chocolateyinstall.ps1" -Value $installContent

# 6. Build package
choco pack chocolatey/installer-analyzer.nuspec --version $version

# 7. Publish package
choco push "installer-analyzer.$version.nupkg" --source https://push.chocolatey.org/ --api-key YOUR_API_KEY
```

## Troubleshooting

### Common Issues

**1. API Key Not Working**
- Verify the API key is correct in GitHub secrets
- Check if the API key has publishing permissions
- Ensure the key hasn't expired

**2. Package Validation Errors**
- Run `chocolatey/validate-package.ps1` locally
- Check nuspec file format and required fields
- Verify PowerShell script syntax

**3. Download URL Issues**
- Ensure the GitHub release contains the Windows binary
- Verify the binary is named `installer-analyzer.exe`
- Check if the release is public and accessible

**4. Checksum Mismatch**
- Verify the binary hasn't been modified after checksum calculation
- Ensure the download URL points to the correct file
- Check if the file was corrupted during upload

### Debugging Steps

1. **Check Workflow Logs**:
   - Go to GitHub Actions tab
   - Find the failed workflow run
   - Examine the "Build and publish Chocolatey package" step

2. **Validate Package Locally**:
   ```powershell
   # Run validation script
   powershell -ExecutionPolicy Bypass -File "chocolatey/validate-package.ps1"
   ```

3. **Test Package Installation**:
   ```powershell
   # Install locally built package
   choco install installer-analyzer --source .
   
   # Test the installed tool
   installer-analyzer --version
   ```

## Security Considerations

- **API Key Protection**: Never commit API keys to the repository
- **Secret Rotation**: Regularly rotate your Chocolatey API key
- **Access Control**: Limit repository access to trusted contributors
- **Audit Trail**: Monitor package publishing through Chocolatey dashboard

## Monitoring and Maintenance

### Package Statistics

Monitor your package through the Chocolatey Community dashboard:
- Download statistics
- User feedback and ratings
- Version adoption rates

### Update Process

When updating the package:
1. Ensure all tests pass
2. Update version in Cargo.toml
3. Create release through release-please
4. Monitor automatic publishing
5. Verify package installation works correctly

### Support

For issues related to:
- **Package Publishing**: Check GitHub Actions logs
- **Package Installation**: Test with `choco install installer-analyzer`
- **Chocolatey Community**: Contact Chocolatey support
- **Tool Issues**: Report on GitHub Issues

## References

- [Chocolatey Package Creation](https://docs.chocolatey.org/en-us/create/create-packages)
- [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [Release Please Documentation](https://github.com/googleapis/release-please)
- [Chocolatey Community Guidelines](https://docs.chocolatey.org/en-us/community-repository/moderation)
