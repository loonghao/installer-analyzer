# Chocolatey uninstall script for installer-analyzer

$ErrorActionPreference = 'Stop'

$packageName = 'installer-analyzer'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

Write-Host "Uninstalling $packageName..." -ForegroundColor Yellow

# Remove the executable
$exePath = Join-Path $toolsDir "installer-analyzer.exe"

if (Test-Path $exePath) {
    try {
        Write-Host "Removing installer-analyzer executable..." -ForegroundColor Yellow
        Remove-Item $exePath -Force
        Write-Host "Executable removed successfully." -ForegroundColor Green
    } catch {
        Write-Warning "Could not remove executable: $($_.Exception.Message)"
        Write-Host "You may need to manually remove: $exePath" -ForegroundColor Yellow
    }
} else {
    Write-Host "Executable not found at expected location: $exePath" -ForegroundColor Yellow
}

# Clean up any temporary files that might have been created
$tempUpdateDir = Join-Path $env:TEMP "installer-analyzer-updates"
if (Test-Path $tempUpdateDir) {
    try {
        Write-Host "Cleaning up temporary update files..." -ForegroundColor Yellow
        Remove-Item $tempUpdateDir -Recurse -Force
        Write-Host "Temporary files cleaned up." -ForegroundColor Green
    } catch {
        Write-Warning "Could not clean up temporary files: $($_.Exception.Message)"
    }
}

# Note: Chocolatey will automatically remove the shim
# No additional action needed for shim removal

Write-Host ""
Write-Host "installer-analyzer has been uninstalled successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Thank you for using installer-analyzer!" -ForegroundColor Cyan
Write-Host "If you encountered any issues, please report them at:" -ForegroundColor White
Write-Host "https://github.com/loonghao/installer-analyzer/issues" -ForegroundColor White
