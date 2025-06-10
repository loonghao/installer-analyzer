# Profile-Guided Optimization (PGO) Build Guide

This document explains how to build installer-analyzer with Profile-Guided Optimization (PGO) for maximum performance.

## What is PGO?

Profile-Guided Optimization (PGO) is a compiler optimization technique that uses runtime profiling data to optimize the generated code. It typically provides 10-20% performance improvements for CPU-intensive applications.

## Build Profiles

The project includes three build profiles for PGO:

- `release`: Standard optimized release build
- `release-pgo-gen`: Instrumented build for collecting profile data
- `release-pgo`: PGO-optimized build using collected profile data

## Quick Start (Local Development)

For local development, use the simple PGO build script:

```powershell
# Simple PGO build
.\scripts\build-pgo-simple.ps1

# Clean build
.\scripts\build-pgo-simple.ps1 -Clean
```

This will:
1. Build an instrumented binary
2. Run basic workloads to collect profile data
3. Build the final optimized binary

## Advanced PGO Build

For more comprehensive optimization, use the full PGO build script:

```powershell
# Full PGO build with verbose output
.\scripts\build-pgo.ps1 -Verbose

# Clean previous data and rebuild
.\scripts\build-pgo.ps1 -Clean -Verbose

# Use custom test data directory
.\scripts\build-pgo.ps1 -TestData "path/to/test/files"
```

## Manual PGO Build

You can also perform PGO builds manually:

### Step 1: Build Instrumented Binary

```bash
# Set environment for profile generation
export RUSTFLAGS="-Cprofile-generate=./pgo-data"
# On Windows PowerShell:
# $env:RUSTFLAGS = "-Cprofile-generate=./pgo-data"

# Build instrumented binary
cargo build --profile release-pgo-gen
```

### Step 2: Collect Profile Data

```bash
# Run representative workloads
./target/release-pgo-gen/installer-analyzer --version
./target/release-pgo-gen/installer-analyzer analyze sample.msi
./target/release-pgo-gen/installer-analyzer analyze sample.exe
# ... run more representative workloads
```

### Step 3: Merge Profile Data (Optional)

```bash
# If llvm-profdata is available
llvm-profdata merge -output=pgo-data/merged.profdata pgo-data/*.profraw
```

### Step 4: Build Optimized Binary

```bash
# Set environment for profile use
export RUSTFLAGS="-Cprofile-use=./pgo-data"
# On Windows PowerShell:
# $env:RUSTFLAGS = "-Cprofile-use=./pgo-data"

# Build optimized binary
cargo build --profile release-pgo
```

## CI/CD Integration

PGO builds are automatically performed in CI for main branch pushes:

- Standard release builds are created for all pushes
- PGO-optimized builds are created only for main branch pushes
- Profile data is cached between builds for efficiency

## Performance Benefits

PGO typically provides:

- **10-20% faster execution** for CPU-intensive operations
- **Better branch prediction** for conditional code
- **Improved instruction cache utilization**
- **Optimized function inlining** based on actual usage patterns

## Best Practices

1. **Use Representative Workloads**: Collect profile data using workloads that represent typical usage patterns.

2. **Include Edge Cases**: Run both common and edge case scenarios during profiling.

3. **Update Profile Data**: Regenerate profile data when making significant code changes.

4. **Test Multiple File Types**: For installer-analyzer, test with various installer formats (MSI, NSIS, InnoSetup, etc.).

5. **Monitor Binary Size**: PGO can sometimes increase binary size due to optimizations.

## Troubleshooting

### Profile Data Not Generated

- Ensure the instrumented binary runs successfully
- Check that the `pgo-data` directory is writable
- Verify RUSTFLAGS environment variable is set correctly

### Build Failures

- Clean previous builds: `cargo clean`
- Remove old profile data: `rm -rf pgo-data`
- Check Rust toolchain version compatibility

### Performance Not Improved

- Ensure profile data represents actual usage patterns
- Try collecting more diverse profile data
- Verify the optimized binary is being used

## File Locations

- **Instrumented binary**: `target/release-pgo-gen/installer-analyzer.exe`
- **Optimized binary**: `target/release-pgo/installer-analyzer.exe`
- **Profile data**: `pgo-data/` directory
- **Build scripts**: `scripts/build-pgo*.ps1`

## Dependencies

- **Rust toolchain**: Stable Rust with PGO support
- **LLVM tools**: Optional, for profile data merging
- **Test data**: Sample installer files for profiling

## Further Reading

- [Rust PGO Documentation](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)
- [LLVM PGO Guide](https://llvm.org/docs/HowToBuildWithPGO.html)
- [Profile-Guided Optimization Best Practices](https://clang.llvm.org/docs/UsersManual.html#profile-guided-optimization)
