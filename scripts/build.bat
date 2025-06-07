@echo off
REM Build script for installer-analyzer

echo Building installer-analyzer...

REM Clean previous builds
cargo clean

REM Run tests
echo Running tests...
cargo test
if %ERRORLEVEL% neq 0 (
    echo Tests failed!
    exit /b 1
)

REM Build release
echo Building release...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Build failed!
    exit /b 1
)

echo Build completed successfully!
echo Binary is located in target\release\installer-analyzer.exe
