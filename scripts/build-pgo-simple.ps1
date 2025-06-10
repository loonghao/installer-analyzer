# Simple PGO Build Script for local development
# Usage: .\scripts\build-pgo-simple.ps1

param(
    [switch]$Clean
)

Write-Host "🚀 Simple PGO build for installer-analyzer" -ForegroundColor Green

if ($Clean) {
    Write-Host "🧹 Cleaning..." -ForegroundColor Yellow
    cargo clean
    if (Test-Path "pgo-data") { Remove-Item -Recurse -Force "pgo-data" }
}

# Create PGO data directory
New-Item -ItemType Directory -Force -Path "pgo-data" | Out-Null

Write-Host "📊 Step 1: Building instrumented binary..." -ForegroundColor Cyan
$env:RUSTFLAGS = "-Cprofile-generate=./pgo-data"
cargo build --profile release-pgo-gen

if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build instrumented binary"
    exit 1
}

Write-Host "🏃 Step 2: Collecting profile data..." -ForegroundColor Cyan
$binary = "target/release-pgo-gen/installer-analyzer.exe"

# Run basic workload
& $binary --version
& $binary --help

# If test data exists, use it
if (Test-Path "datas") {
    $testFiles = Get-ChildItem -Path "datas" -Include "*.msi", "*.exe" -Recurse | Select-Object -First 3
    foreach ($file in $testFiles) {
        Write-Host "  Testing with: $($file.Name)" -ForegroundColor Gray
        try {
            & $binary analyze $file.FullName --format json --output "temp.json" 2>$null
            if (Test-Path "temp.json") { Remove-Item "temp.json" }
        } catch {
            Write-Warning "Failed to process $($file.Name)"
        }
    }
}

Write-Host "🎯 Step 3: Building optimized binary..." -ForegroundColor Cyan
$env:RUSTFLAGS = "-Cprofile-use=./pgo-data"
cargo build --profile release-pgo

if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build PGO-optimized binary"
    exit 1
}

Write-Host "✅ PGO build completed!" -ForegroundColor Green
Write-Host "📦 Optimized binary: target/release-pgo/installer-analyzer.exe" -ForegroundColor Blue

# Test the optimized binary
& "target/release-pgo/installer-analyzer.exe" --version

# Clean up environment
Remove-Item Env:RUSTFLAGS -ErrorAction SilentlyContinue

Write-Host "🎉 Done! Use target/release-pgo/installer-analyzer.exe for best performance." -ForegroundColor Magenta
