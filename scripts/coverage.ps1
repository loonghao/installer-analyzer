# Code Coverage Analysis Script for installer-analyzer
# Usage: .\scripts\coverage.ps1 [--full] [--open]

param(
    [switch]$Full,      # Include all tests (including slow real data tests)
    [switch]$Open,      # Open HTML report in browser
    [switch]$CI         # CI mode - generate LCOV only
)

Write-Host "🧪 Running Code Coverage Analysis for installer-analyzer" -ForegroundColor Cyan
Write-Host "=" * 60

# Clean previous coverage data
if (Test-Path "target/llvm-cov") {
    Remove-Item -Recurse -Force "target/llvm-cov"
    Write-Host "✓ Cleaned previous coverage data" -ForegroundColor Green
}

# Determine test scope
if ($Full) {
    Write-Host "📊 Running FULL coverage analysis (including real data tests)" -ForegroundColor Yellow
    $testArgs = "--all-features --workspace"
    $excludeArgs = "--ignore-filename-regex=`"performance_tests`""
} else {
    Write-Host "📊 Running FAST coverage analysis (excluding slow tests)" -ForegroundColor Green
    $testArgs = "--lib --tests cli_tests output_tests"
    $excludeArgs = "--ignore-filename-regex=`"real_data_tests|format_specific_tests|performance_tests`""
}

# Generate coverage reports
Write-Host "`n🔍 Generating coverage reports..." -ForegroundColor Blue

if ($CI) {
    # CI mode - only generate LCOV
    Write-Host "🤖 CI Mode: Generating LCOV report only"
    $cmd = "cargo llvm-cov $testArgs $excludeArgs --lcov --output-path coverage.lcov"
    Write-Host "Executing: $cmd" -ForegroundColor Gray
    Invoke-Expression $cmd
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ LCOV report generated: coverage.lcov" -ForegroundColor Green
    } else {
        Write-Host "❌ Coverage generation failed" -ForegroundColor Red
        exit 1
    }
} else {
    # Development mode - generate HTML and LCOV
    Write-Host "🌐 Development Mode: Generating HTML and LCOV reports"
    
    # Generate HTML report
    $htmlCmd = "cargo llvm-cov $testArgs $excludeArgs --html"
    Write-Host "Executing: $htmlCmd" -ForegroundColor Gray
    Invoke-Expression $htmlCmd
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ HTML coverage generation failed" -ForegroundColor Red
        exit 1
    }
    
    # Generate LCOV report
    $lcovCmd = "cargo llvm-cov $testArgs $excludeArgs --lcov --output-path coverage.lcov"
    Write-Host "Executing: $lcovCmd" -ForegroundColor Gray
    Invoke-Expression $lcovCmd
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ LCOV coverage generation failed" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ Coverage reports generated successfully!" -ForegroundColor Green
    Write-Host "  📁 HTML Report: target/llvm-cov/html/index.html" -ForegroundColor Cyan
    Write-Host "  📄 LCOV Report: coverage.lcov" -ForegroundColor Cyan
}

# Display coverage summary
Write-Host "`n📈 Coverage Summary:" -ForegroundColor Blue
if (Test-Path "coverage.lcov") {
    # Parse LCOV for basic stats
    $lcovContent = Get-Content "coverage.lcov" -Raw
    $totalLines = ($lcovContent | Select-String "LF:" | Measure-Object).Count

    if ($totalLines -gt 0) {
        Write-Host "  📊 Total source files analyzed: $totalLines" -ForegroundColor White
        Write-Host "  📈 Line coverage data available in coverage.lcov" -ForegroundColor White
    }
}

# Open HTML report if requested
if ($Open -and !$CI) {
    $htmlReport = "target/llvm-cov/html/index.html"
    if (Test-Path $htmlReport) {
        Write-Host "`n🌐 Opening HTML coverage report..." -ForegroundColor Green
        Start-Process $htmlReport
    } else {
        Write-Host "⚠️  HTML report not found at $htmlReport" -ForegroundColor Yellow
    }
}

# Coverage improvement suggestions
Write-Host "`n💡 Coverage Improvement Suggestions:" -ForegroundColor Magenta
Write-Host "  1. Add unit tests for analyzer modules (currently 0% coverage)" -ForegroundColor White
Write-Host "  2. Test error handling paths in CLI commands" -ForegroundColor White
Write-Host "  3. Add integration tests for reporting modules" -ForegroundColor White
Write-Host "  4. Test sandbox controller functionality" -ForegroundColor White
Write-Host "  5. Add tests for utility functions" -ForegroundColor White

Write-Host "`n🎯 Current Focus Areas:" -ForegroundColor Blue
Write-Host "  • CLI modules: ~50% coverage (good progress!)" -ForegroundColor Green
Write-Host "  • Core modules: ~7% coverage (needs improvement)" -ForegroundColor Yellow
Write-Host "  • Analyzer modules: 0% coverage (high priority)" -ForegroundColor Red
Write-Host "  • Reporting modules: 0% coverage (medium priority)" -ForegroundColor Yellow

Write-Host "`n✨ Coverage analysis complete!" -ForegroundColor Green
Write-Host "=" * 60
