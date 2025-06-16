# CI/CD Configuration Validation Script
# This script validates the GitHub Actions workflows and Chocolatey integration

$ErrorActionPreference = 'Stop'

Write-Host "Validating CI/CD Configuration..." -ForegroundColor Green
Write-Host ""

$projectRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Definition)
$errors = @()
$warnings = @()

# Validate GitHub Actions workflows
Write-Host "1. Checking GitHub Actions workflows..." -ForegroundColor Yellow

$workflowFiles = @(
    ".github/workflows/ci.yml",
    ".github/workflows/release.yml",
    ".github/workflows/release-please.yml"
)

foreach ($workflow in $workflowFiles) {
    $workflowPath = Join-Path $projectRoot $workflow
    if (Test-Path $workflowPath) {
        Write-Host "  ✓ $workflow exists" -ForegroundColor Green
        
        # Basic YAML syntax check (read file)
        try {
            $content = Get-Content $workflowPath -Raw
            if ($content.Length -gt 0) {
                Write-Host "  ✓ $workflow is readable and not empty" -ForegroundColor Green
            } else {
                $errors += "$workflow is empty"
                Write-Host "  ✗ $workflow is empty" -ForegroundColor Red
            }
        } catch {
            $errors += "Error reading ${workflow}: $($_.Exception.Message)"
            Write-Host "  ✗ Error reading $workflow" -ForegroundColor Red
        }
    } else {
        $warnings += "Workflow file not found: $workflow"
        Write-Host "  ⚠ $workflow not found" -ForegroundColor Yellow
    }
}

# Validate Chocolatey integration in release-please.yml
Write-Host ""
Write-Host "2. Validating Chocolatey integration..." -ForegroundColor Yellow

$releasePleaseFile = Join-Path $projectRoot ".github/workflows/release-please.yml"
if (Test-Path $releasePleaseFile) {
    $content = Get-Content $releasePleaseFile -Raw
    
    # Check for Chocolatey-related steps
    $chocolateyChecks = @{
        "Prepare Chocolatey package" = "Prepare Chocolatey package"
        "Install Chocolatey" = "Install Chocolatey"
        "Build and publish Chocolatey package" = "Build and publish Chocolatey package"
        "CHOCOLATEY_API_KEY" = "CHOCOLATEY_API_KEY"
        "choco pack" = "choco pack"
        "choco push" = "choco push"
    }
    
    foreach ($check in $chocolateyChecks.GetEnumerator()) {
        if ($content -match [regex]::Escape($check.Value)) {
            Write-Host "  ✓ Found: $($check.Key)" -ForegroundColor Green
        } else {
            $errors += "Missing Chocolatey integration: $($check.Key)"
            Write-Host "  ✗ Missing: $($check.Key)" -ForegroundColor Red
        }
    }
    
    # Check for conditional execution
    if ($content -match "if:\s*env\.CHOCOLATEY_API_KEY\s*!=\s*''") {
        Write-Host "  ✓ Conditional execution configured" -ForegroundColor Green
    } else {
        $warnings += "Chocolatey publishing should be conditional on API key presence"
        Write-Host "  ⚠ Conditional execution not found" -ForegroundColor Yellow
    }
    
} else {
    $errors += "release-please.yml not found"
    Write-Host "  ✗ release-please.yml not found" -ForegroundColor Red
}

# Validate Chocolatey package configuration
Write-Host ""
Write-Host "3. Checking Chocolatey package configuration..." -ForegroundColor Yellow

$chocolateyFiles = @(
    "chocolatey/installer-analyzer.nuspec",
    "chocolatey/tools/chocolateyinstall.ps1",
    "chocolatey/tools/chocolateyuninstall.ps1"
)

foreach ($file in $chocolateyFiles) {
    $filePath = Join-Path $projectRoot $file
    if (Test-Path $filePath) {
        Write-Host "  ✓ $file exists" -ForegroundColor Green
    } else {
        $errors += "Missing Chocolatey file: $file"
        Write-Host "  ✗ $file missing" -ForegroundColor Red
    }
}

# Check for documentation
Write-Host ""
Write-Host "4. Checking documentation..." -ForegroundColor Yellow

$docFiles = @(
    "docs/CHOCOLATEY_SETUP.md",
    "chocolatey/README.md"
)

foreach ($doc in $docFiles) {
    $docPath = Join-Path $projectRoot $doc
    if (Test-Path $docPath) {
        Write-Host "  ✓ $doc exists" -ForegroundColor Green
    } else {
        $warnings += "Documentation file not found: $doc"
        Write-Host "  ⚠ $doc not found" -ForegroundColor Yellow
    }
}

# Validate release configuration files
Write-Host ""
Write-Host "5. Checking release configuration..." -ForegroundColor Yellow

$releaseFiles = @(
    ".release-please-config.json",
    ".release-please-manifest.json"
)

foreach ($file in $releaseFiles) {
    $filePath = Join-Path $projectRoot $file
    if (Test-Path $filePath) {
        Write-Host "  ✓ $file exists" -ForegroundColor Green
        
        # Validate JSON syntax
        try {
            $json = Get-Content $filePath -Raw | ConvertFrom-Json
            Write-Host "  ✓ $file has valid JSON syntax" -ForegroundColor Green
        } catch {
            $errors += "Invalid JSON in ${file}: $($_.Exception.Message)"
            Write-Host "  ✗ $file has invalid JSON syntax" -ForegroundColor Red
        }
    } else {
        $warnings += "Release configuration file not found: $file"
        Write-Host "  ⚠ $file not found" -ForegroundColor Yellow
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
    Write-Host "CI/CD configuration is valid and ready for use!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Add CHOCOLATEY_API_KEY secret to GitHub repository" -ForegroundColor White
    Write-Host "2. Test the workflow by creating a release" -ForegroundColor White
    Write-Host "3. Monitor the Chocolatey package publishing process" -ForegroundColor White
    Write-Host ""
    Write-Host "For setup instructions, see: docs/CHOCOLATEY_SETUP.md" -ForegroundColor Cyan
    exit 0
} else {
    Write-Host "CI/CD configuration has errors that need to be fixed." -ForegroundColor Red
    exit 1
}
