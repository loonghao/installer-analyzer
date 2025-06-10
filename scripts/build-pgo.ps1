# PGO (Profile-Guided Optimization) Build Script for installer-analyzer
# This script performs a multi-stage PGO build for maximum performance

param(
    [switch]$Clean,
    [switch]$Verbose,
    [string]$TestData = "datas"
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 Starting PGO build for installer-analyzer" -ForegroundColor Green

# Clean previous PGO data if requested
if ($Clean) {
    Write-Host "🧹 Cleaning previous PGO data..." -ForegroundColor Yellow
    if (Test-Path "pgo-data") {
        Remove-Item -Recurse -Force "pgo-data"
    }
    if (Test-Path "target/release-pgo-gen") {
        Remove-Item -Recurse -Force "target/release-pgo-gen"
    }
    if (Test-Path "target/release-pgo") {
        Remove-Item -Recurse -Force "target/release-pgo"
    }
}

# Create PGO data directory
New-Item -ItemType Directory -Force -Path "pgo-data" | Out-Null

Write-Host "📊 Stage 1: Building instrumented binary for profile collection..." -ForegroundColor Cyan

# Set environment for profile generation
$env:RUSTFLAGS = "-Cprofile-generate=./pgo-data"

# Build instrumented binary
if ($Verbose) {
    cargo build --profile release-pgo-gen --verbose
} else {
    cargo build --profile release-pgo-gen
}

if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build instrumented binary"
    exit 1
}

Write-Host "✅ Instrumented binary built successfully" -ForegroundColor Green

Write-Host "🏃 Stage 2: Running workload to collect profile data..." -ForegroundColor Cyan

# Get the instrumented binary path
$instrumentedBinary = "target/release-pgo-gen/installer-analyzer.exe"

if (-not (Test-Path $instrumentedBinary)) {
    Write-Error "Instrumented binary not found at $instrumentedBinary"
    exit 1
}

# Run representative workloads to collect profile data
$testFiles = @()

# Find test files in the data directory
if (Test-Path $TestData) {
    $testFiles = Get-ChildItem -Path $TestData -Recurse -Include "*.msi", "*.exe", "*.zip" | Select-Object -First 10
}

if ($testFiles.Count -eq 0) {
    Write-Warning "No test files found in $TestData directory. Using synthetic workload..."
    
    # Run basic commands to generate some profile data
    & $instrumentedBinary --version
    & $instrumentedBinary --help
} else {
    Write-Host "Found $($testFiles.Count) test files for profiling" -ForegroundColor Blue
    
    foreach ($file in $testFiles) {
        Write-Host "  Processing: $($file.Name)" -ForegroundColor Gray
        try {
            # Run analysis on each test file (with timeout)
            $job = Start-Job -ScriptBlock {
                param($binary, $filePath)
                & $binary analyze $filePath --format json --output "temp-profile-output.json" 2>$null
            } -ArgumentList $instrumentedBinary, $file.FullName
            
            # Wait for job with timeout (30 seconds per file)
            Wait-Job $job -Timeout 30 | Out-Null
            Remove-Job $job -Force
            
            # Clean up temporary output
            if (Test-Path "temp-profile-output.json") {
                Remove-Item "temp-profile-output.json"
            }
        } catch {
            Write-Warning "Failed to process $($file.Name): $($_.Exception.Message)"
        }
    }
}

Write-Host "✅ Profile data collection completed" -ForegroundColor Green

Write-Host "🔄 Stage 3: Merging profile data..." -ForegroundColor Cyan

# Check if we have profile data
$profileFiles = Get-ChildItem -Path "pgo-data" -Filter "*.profraw" -Recurse

if ($profileFiles.Count -eq 0) {
    Write-Warning "No profile data files found. PGO optimization may not be effective."
} else {
    Write-Host "Found $($profileFiles.Count) profile data files" -ForegroundColor Blue
}

# Merge profile data using llvm-profdata
try {
    $llvmProfdata = "llvm-profdata"
    
    # Try to find llvm-profdata in common locations
    $possiblePaths = @(
        "llvm-profdata",
        "C:\Program Files\LLVM\bin\llvm-profdata.exe",
        "$env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\llvm-profdata.exe"
    )
    
    $llvmProfdataPath = $null
    foreach ($path in $possiblePaths) {
        if (Get-Command $path -ErrorAction SilentlyContinue) {
            $llvmProfdataPath = $path
            break
        }
    }
    
    if ($llvmProfdataPath) {
        & $llvmProfdataPath merge -output=pgo-data/merged.profdata pgo-data/*.profraw
        Write-Host "✅ Profile data merged successfully" -ForegroundColor Green
    } else {
        Write-Warning "llvm-profdata not found. Skipping profile data merging."
        Write-Host "Profile data will be used directly by rustc" -ForegroundColor Yellow
    }
} catch {
    Write-Warning "Failed to merge profile data: $($_.Exception.Message)"
    Write-Host "Continuing with individual profile files..." -ForegroundColor Yellow
}

Write-Host "🎯 Stage 4: Building optimized binary with PGO..." -ForegroundColor Cyan

# Clear previous RUSTFLAGS and set for PGO use
if (Test-Path "pgo-data/merged.profdata") {
    $env:RUSTFLAGS = "-Cprofile-use=./pgo-data/merged.profdata"
} else {
    $env:RUSTFLAGS = "-Cprofile-use=./pgo-data"
}

# Build optimized binary
if ($Verbose) {
    cargo build --profile release-pgo --verbose
} else {
    cargo build --profile release-pgo
}

if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build PGO-optimized binary"
    exit 1
}

Write-Host "✅ PGO-optimized binary built successfully" -ForegroundColor Green

# Verify the optimized binary
$optimizedBinary = "target/release-pgo/installer-analyzer.exe"
if (Test-Path $optimizedBinary) {
    $fileInfo = Get-Item $optimizedBinary
    Write-Host "📦 Optimized binary: $optimizedBinary" -ForegroundColor Blue
    Write-Host "📏 Size: $([math]::Round($fileInfo.Length / 1MB, 2)) MB" -ForegroundColor Blue
    
    # Test the optimized binary
    Write-Host "🧪 Testing optimized binary..." -ForegroundColor Cyan
    & $optimizedBinary --version
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ PGO build completed successfully!" -ForegroundColor Green
        Write-Host "🎉 Optimized binary is ready at: $optimizedBinary" -ForegroundColor Magenta
    } else {
        Write-Error "Optimized binary failed basic test"
        exit 1
    }
} else {
    Write-Error "Optimized binary not found"
    exit 1
}

# Clean up environment
Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue

Write-Host "🏁 PGO build process completed!" -ForegroundColor Green
