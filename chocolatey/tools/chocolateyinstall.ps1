# Chocolatey install script for installer-analyzer

$ErrorActionPreference = 'Stop'

$packageName = 'installer-analyzer'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$packageVersion = $env:ChocolateyPackageVersion

# Package parameters
$packageArgs = @{
  packageName    = $packageName
  unzipLocation  = $toolsDir
  fileType       = 'exe'
  
  # Download URL will be dynamically set by CI/CD pipeline
  url64bit       = "https://github.com/loonghao/installer-analyzer/releases/download/v$packageVersion/installer-analyzer.exe"
  
  # Checksums will be dynamically set by CI/CD pipeline
  checksum64     = ''
  checksumType64 = 'sha256'
  
  # Silent install options
  silentArgs     = ''
  validExitCodes = @(0)
}

Write-Host "Installing $packageName version $packageVersion..." -ForegroundColor Green

# Download and install the executable
Install-ChocolateyPackage @packageArgs

# Create a shim for the executable
$exePath = Join-Path $toolsDir "installer-analyzer.exe"

if (Test-Path $exePath) {
    Write-Host "Creating shim for installer-analyzer..." -ForegroundColor Green
    
    # The executable will be automatically shimmed by Chocolatey
    # No additional action needed as the exe is in the tools directory
    
    Write-Host "Installation completed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Usage:" -ForegroundColor Yellow
    Write-Host "  installer-analyzer --help" -ForegroundColor White
    Write-Host "  installer-analyzer analyze setup.exe" -ForegroundColor White
    Write-Host "  installer-analyzer update --check-only" -ForegroundColor White
    Write-Host ""
    Write-Host "For more information, visit: https://github.com/loonghao/installer-analyzer" -ForegroundColor Cyan
} else {
    throw "Installation failed: installer-analyzer.exe not found at $exePath"
}

# Verify installation
try {
    $version = & "$exePath" --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Verification successful: $version" -ForegroundColor Green
    } else {
        Write-Warning "Could not verify installation, but executable exists"
    }
} catch {
    Write-Warning "Could not verify installation: $($_.Exception.Message)"
}

Write-Host ""
Write-Host "installer-analyzer has been installed and is ready to use!" -ForegroundColor Green
