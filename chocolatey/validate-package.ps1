# Validation script for Chocolatey package configuration
# This script validates the package structure and configuration files

$ErrorActionPreference = 'Stop'

Write-Host "Validating Chocolatey package configuration..." -ForegroundColor Green
Write-Host ""

$packageDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$errors = @()
$warnings = @()

# Validate package structure
Write-Host "1. Checking package structure..." -ForegroundColor Yellow

$requiredFiles = @(
    "installer-analyzer.nuspec",
    "tools\chocolateyinstall.ps1",
    "tools\chocolateyuninstall.ps1"
)

foreach ($file in $requiredFiles) {
    $filePath = Join-Path $packageDir $file
    if (Test-Path $filePath) {
        Write-Host "  ✓ $file exists" -ForegroundColor Green
    } else {
        $errors += "Missing required file: $file"
        Write-Host "  ✗ $file missing" -ForegroundColor Red
    }
}

# Validate nuspec file
Write-Host ""
Write-Host "2. Validating nuspec file..." -ForegroundColor Yellow

$nuspecPath = Join-Path $packageDir "installer-analyzer.nuspec"
if (Test-Path $nuspecPath) {
    try {
        [xml]$nuspec = Get-Content $nuspecPath
        Write-Host "  ✓ XML format is valid" -ForegroundColor Green
        
        # Check required metadata
        $metadata = $nuspec.package.metadata
        $requiredFields = @('id', 'version', 'authors', 'description')
        
        foreach ($field in $requiredFields) {
            if ($metadata.$field) {
                Write-Host "  ✓ $field is present" -ForegroundColor Green
            } else {
                $errors += "Missing required metadata field: $field"
                Write-Host "  ✗ $field is missing" -ForegroundColor Red
            }
        }
        
        # Check version format
        if ($metadata.version -eq '$version$') {
            Write-Host "  ✓ Version placeholder is correct" -ForegroundColor Green
        } else {
            $warnings += "Version should be '\$version\$' for CI/CD replacement"
            Write-Host "  ⚠ Version is not a placeholder" -ForegroundColor Yellow
        }
        
    } catch {
        $errors += "Invalid XML format in nuspec file: $($_.Exception.Message)"
        Write-Host "  ✗ XML parsing failed" -ForegroundColor Red
    }
}

# Validate PowerShell scripts
Write-Host ""
Write-Host "3. Validating PowerShell scripts..." -ForegroundColor Yellow

$scriptFiles = @(
    "tools\chocolateyinstall.ps1",
    "tools\chocolateyuninstall.ps1",
    "tools\chocolateybeforemodify.ps1"
)

foreach ($script in $scriptFiles) {
    $scriptPath = Join-Path $packageDir $script
    if (Test-Path $scriptPath) {
        try {
            # Basic syntax check by reading the file
            $content = Get-Content $scriptPath -Raw
            if ($content.Length -gt 0) {
                Write-Host "  ✓ ${script} is readable and not empty" -ForegroundColor Green

                # Check for required patterns
                if ($script -eq "tools\chocolateyinstall.ps1") {
                    if ($content -match 'Install-ChocolateyPackage') {
                        Write-Host "  ✓ Install script contains Install-ChocolateyPackage" -ForegroundColor Green
                    } else {
                        $warnings += "Install script should use Install-ChocolateyPackage"
                        Write-Host "  ⚠ Install-ChocolateyPackage not found" -ForegroundColor Yellow
                    }
                }
                
                if ($content -match '\$ErrorActionPreference') {
                    Write-Host "  ✓ ${script} sets ErrorActionPreference" -ForegroundColor Green
                } else {
                    $warnings += "${script} should set ErrorActionPreference"
                    Write-Host "  ⚠ ErrorActionPreference not set" -ForegroundColor Yellow
                }
            } else {
                $errors += "${script} is empty"
                Write-Host "  ✗ ${script} is empty" -ForegroundColor Red
            }
        } catch {
            $errors += "Error reading ${script}: $($_.Exception.Message)"
            Write-Host "  ✗ Error reading ${script}" -ForegroundColor Red
        }
    }
}

# Summary
Write-Host ""
Write-Host "Validation Summary:" -ForegroundColor Cyan
Write-Host "==================" -ForegroundColor Cyan

if ($errors.Count -eq 0) {
    Write-Host "✓ No errors found" -ForegroundColor Green
} else {
    Write-Host "✗ $($errors.Count) error(s) found:" -ForegroundColor Red
    foreach ($error in $errors) {
        Write-Host "  - $error" -ForegroundColor Red
    }
}

if ($warnings.Count -eq 0) {
    Write-Host "✓ No warnings" -ForegroundColor Green
} else {
    Write-Host "⚠ $($warnings.Count) warning(s):" -ForegroundColor Yellow
    foreach ($warning in $warnings) {
        Write-Host "  - $warning" -ForegroundColor Yellow
    }
}

Write-Host ""
if ($errors.Count -eq 0) {
    Write-Host "Package configuration is valid and ready for use!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "Package configuration has errors that need to be fixed." -ForegroundColor Red
    exit 1
}
