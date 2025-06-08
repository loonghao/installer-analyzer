# Fast CI Coverage Analysis Script
# Optimized for CI environments - excludes slow real data tests

Write-Host "🤖 CI Code Coverage Analysis" -ForegroundColor Cyan
Write-Host "=" * 50

# Clean previous coverage data
if (Test-Path "target/llvm-cov") {
    Remove-Item -Recurse -Force "target/llvm-cov"
    Write-Host "✓ Cleaned previous coverage data" -ForegroundColor Green
}

# Fast coverage analysis for CI
Write-Host "📊 Running FAST coverage analysis (CI optimized)" -ForegroundColor Green
Write-Host "⏱️  Excluding slow tests: real_data_tests, format_specific_tests, performance_tests" -ForegroundColor Yellow

# Generate LCOV report only (faster for CI)
$lcovCmd = "cargo llvm-cov --lib --tests cli_tests --tests output_tests --lcov --output-path coverage.lcov --ignore-filename-regex=`"real_data_tests|format_specific_tests|performance_tests`""
Write-Host "Executing: $lcovCmd" -ForegroundColor Gray
Invoke-Expression $lcovCmd

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Coverage generation failed" -ForegroundColor Red
    exit 1
}

Write-Host "✅ CI coverage report generated: coverage.lcov" -ForegroundColor Green

# Display basic coverage info
if (Test-Path "coverage.lcov") {
    $lcovContent = Get-Content "coverage.lcov" -Raw
    $totalLines = ($lcovContent | Select-String "LF:" | Measure-Object).Count
    
    if ($totalLines -gt 0) {
        Write-Host "📊 Coverage analysis complete" -ForegroundColor Blue
        Write-Host "  📁 Files analyzed: $totalLines" -ForegroundColor White
        Write-Host "  📄 Report: coverage.lcov" -ForegroundColor White
    }
}

Write-Host "🎯 CI Coverage Summary:" -ForegroundColor Blue
Write-Host "  ✅ Fast analysis completed" -ForegroundColor Green
Write-Host "  ⚡ Excluded slow tests for CI efficiency" -ForegroundColor Yellow
Write-Host "  📈 LCOV report ready for upload" -ForegroundColor Cyan

Write-Host "✨ CI coverage analysis complete!" -ForegroundColor Green
Write-Host "=" * 50
