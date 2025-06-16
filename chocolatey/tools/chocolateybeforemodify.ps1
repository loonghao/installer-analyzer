# Chocolatey before modify script for installer-analyzer
# This script runs before package modification (upgrade/uninstall)

$ErrorActionPreference = 'Stop'

$packageName = 'installer-analyzer'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

Write-Host "Preparing $packageName for modification..." -ForegroundColor Yellow

# Check if installer-analyzer is currently running
$runningProcesses = Get-Process -Name "installer-analyzer" -ErrorAction SilentlyContinue

if ($runningProcesses) {
    Write-Host "Found running installer-analyzer processes. Attempting to stop them..." -ForegroundColor Yellow
    
    foreach ($process in $runningProcesses) {
        try {
            Write-Host "Stopping process ID $($process.Id)..." -ForegroundColor Yellow
            $process.CloseMainWindow()
            
            # Wait up to 5 seconds for graceful shutdown
            if (!$process.WaitForExit(5000)) {
                Write-Host "Force stopping process ID $($process.Id)..." -ForegroundColor Yellow
                $process.Kill()
            }
            
            Write-Host "Process stopped successfully." -ForegroundColor Green
        } catch {
            Write-Warning "Could not stop process ID $($process.Id): $($_.Exception.Message)"
        }
    }
    
    # Wait a moment for file handles to be released
    Start-Sleep -Seconds 2
}

# Check for any update processes that might be running
$updateProcesses = Get-Process | Where-Object { $_.ProcessName -like "*installer-analyzer*" -or $_.CommandLine -like "*installer-analyzer*" } -ErrorAction SilentlyContinue

if ($updateProcesses) {
    Write-Host "Found related processes, waiting for them to complete..." -ForegroundColor Yellow
    Start-Sleep -Seconds 3
}

Write-Host "Ready for package modification." -ForegroundColor Green
