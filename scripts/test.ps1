# Test runner script for Installer Analyzer
# This script runs comprehensive tests and generates reports

param(
    [switch]$Coverage,
    [switch]$Integration,
    [switch]$Unit,
    [switch]$All,
    [switch]$Verbose
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Colors for output
$Green = "`e[32m"
$Red = "`e[31m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Reset = "`e[0m"

function Write-ColorOutput {
    param($Color, $Message)
    Write-Host "$Color$Message$Reset"
}

function Test-Command {
    param($Command)
    try {
        & $Command 2>$null
        return $true
    } catch {
        return $false
    }
}

# Check if we're in the right directory
if (!(Test-Path "Cargo.toml")) {
    Write-ColorOutput $Red "Error: Please run this script from the project root directory"
    exit 1
}

Write-ColorOutput $Blue "🧪 Installer Analyzer Test Suite"
Write-ColorOutput $Blue "================================"

# Default to all tests if no specific type is specified
if (!$Coverage -and !$Integration -and !$Unit) {
    $All = $true
}

$TestsPassed = $true

# Format check
Write-ColorOutput $Yellow "📝 Checking code formatting..."
try {
    cargo fmt --all -- --check
    Write-ColorOutput $Green "✅ Code formatting check passed"
} catch {
    Write-ColorOutput $Red "❌ Code formatting check failed"
    $TestsPassed = $false
}

# Clippy check
Write-ColorOutput $Yellow "🔍 Running Clippy lints..."
try {
    if ($Verbose) {
        cargo clippy --all-targets --all-features -- -D warnings
    } else {
        cargo clippy --all-targets --all-features -- -D warnings 2>$null
    }
    Write-ColorOutput $Green "✅ Clippy check passed"
} catch {
    Write-ColorOutput $Red "❌ Clippy check failed"
    $TestsPassed = $false
}

# Unit tests
if ($Unit -or $All) {
    Write-ColorOutput $Yellow "🔬 Running unit tests..."
    try {
        if ($Verbose) {
            cargo test --lib
        } else {
            cargo test --lib --quiet
        }
        Write-ColorOutput $Green "✅ Unit tests passed"
    } catch {
        Write-ColorOutput $Red "❌ Unit tests failed"
        $TestsPassed = $false
    }
}

# Integration tests
if ($Integration -or $All) {
    Write-ColorOutput $Yellow "🔗 Running integration tests..."
    try {
        if ($Verbose) {
            cargo test --test '*'
        } else {
            cargo test --test '*' --quiet
        }
        Write-ColorOutput $Green "✅ Integration tests passed"
    } catch {
        Write-ColorOutput $Red "❌ Integration tests failed"
        $TestsPassed = $false
    }
}

# Doc tests
if ($All) {
    Write-ColorOutput $Yellow "📚 Running documentation tests..."
    try {
        if ($Verbose) {
            cargo test --doc
        } else {
            cargo test --doc --quiet
        }
        Write-ColorOutput $Green "✅ Documentation tests passed"
    } catch {
        Write-ColorOutput $Red "❌ Documentation tests failed"
        $TestsPassed = $false
    }
}

# All tests together
if ($All) {
    Write-ColorOutput $Yellow "🎯 Running all tests together..."
    try {
        if ($Verbose) {
            cargo test --all-features
        } else {
            cargo test --all-features --quiet
        }
        Write-ColorOutput $Green "✅ All tests passed"
    } catch {
        Write-ColorOutput $Red "❌ Some tests failed"
        $TestsPassed = $false
    }
}

# Coverage (optional)
if ($Coverage) {
    Write-ColorOutput $Yellow "📊 Generating test coverage..."
    
    # Check if cargo-tarpaulin is installed
    if (Test-Command "cargo tarpaulin --version") {
        try {
            cargo tarpaulin --out Html --output-dir coverage
            Write-ColorOutput $Green "✅ Coverage report generated in coverage/"
        } catch {
            Write-ColorOutput $Red "❌ Coverage generation failed"
            $TestsPassed = $false
        }
    } else {
        Write-ColorOutput $Yellow "⚠️  cargo-tarpaulin not installed. Install with: cargo install cargo-tarpaulin"
    }
}

# Build check
Write-ColorOutput $Yellow "🔨 Building release version..."
try {
    if ($Verbose) {
        cargo build --release
    } else {
        cargo build --release --quiet
    }
    Write-ColorOutput $Green "✅ Release build successful"
} catch {
    Write-ColorOutput $Red "❌ Release build failed"
    $TestsPassed = $false
}

# Test the built binary
if (Test-Path "target/release/installer-analyzer.exe") {
    Write-ColorOutput $Yellow "🚀 Testing release binary..."
    try {
        & "./target/release/installer-analyzer.exe" --version | Out-Null
        & "./target/release/installer-analyzer.exe" --help | Out-Null
        Write-ColorOutput $Green "✅ Release binary works correctly"
    } catch {
        Write-ColorOutput $Red "❌ Release binary test failed"
        $TestsPassed = $false
    }
}

# Summary
Write-ColorOutput $Blue "`n📋 Test Summary"
Write-ColorOutput $Blue "==============="

if ($TestsPassed) {
    Write-ColorOutput $Green "🎉 All tests passed successfully!"
    exit 0
} else {
    Write-ColorOutput $Red "💥 Some tests failed. Please check the output above."
    exit 1
}
