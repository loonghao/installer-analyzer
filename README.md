# Installer Analyzer

[![CI](https://github.com/loonghao/installer-analyzer/workflows/CI/badge.svg)](https://github.com/loonghao/installer-analyzer/actions)
[![Release](https://github.com/loonghao/installer-analyzer/workflows/Release/badge.svg)](https://github.com/loonghao/installer-analyzer/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Downloads](https://img.shields.io/github/downloads/loonghao/installer-analyzer/total.svg)](https://github.com/loonghao/installer-analyzer/releases)
[![Stars](https://img.shields.io/github/stars/loonghao/installer-analyzer.svg)](https://github.com/loonghao/installer-analyzer/stargazers)

[English](README.md) | [中文](README_zh.md)

A comprehensive, cross-platform tool for analyzing software installation packages and monitoring installation behavior. Supports 8 major installer formats with detailed analysis, interactive reporting, and modern web-based visualization.

## ✨ Features

### 📦 Multi-Format Support (8 Formats)
- **MSI** - Microsoft Installer packages with database parsing
- **WiX** - WiX Toolset generated MSI with extension detection
- **NSIS** - Nullsoft Scriptable Install System with script analysis
- **Squirrel** - Electron application installers with auto-update detection
- **InnoSetup** - Inno Setup installers with script parsing
- **InstallShield** - Enterprise installation packages with version detection
- **MSIX/AppX** - Modern Windows app packages with manifest parsing
- **Python Wheel** - Python package format with metadata extraction

### 🔍 Advanced Analysis Capabilities
- **File Extraction** - Extract and analyze embedded files with type detection
- **Registry Operations** - Detect and analyze registry modifications
- **Metadata Extraction** - Product info, version, publisher, certificates
- **Security Analysis** - File signatures, digital certificates, trust validation
- **Installation Simulation** - Sandbox environment support (planned)
- **Dependency Analysis** - Identify package dependencies and requirements

### 📊 Interactive Reporting & Visualization
- **Modern HTML Reports** - Responsive web interface with Bootstrap 5
- **Interactive File Tree** - Hierarchical structure with expand/collapse
- **Real-time Search** - Filter files and directories instantly
- **Visual Charts** - File type distribution, size statistics, and trends
- **JSON Export** - Machine-readable analysis results for automation
- **Detailed Metadata** - Comprehensive package information display

### 🛠️ Developer & Enterprise Tools
- **Powerful CLI** - Command-line interface with multiple analysis modes
- **Modular Architecture** - Extensible analyzer framework with plugin support
- **Cross-Platform** - Native support for Windows, Linux, macOS
- **Batch Processing** - Analyze multiple packages simultaneously
- **API Integration** - Programmatic access for CI/CD pipelines
- **Performance Optimized** - Efficient memory usage and fast processing

## 🚀 Quick Start

### Installation

#### Option 1: Download Pre-built Binaries
Download the latest release from [GitHub Releases](https://github.com/loonghao/installer-analyzer/releases):

```bash
# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/loonghao/installer-analyzer/releases/latest/download/installer-analyzer-windows-amd64.zip" -OutFile "installer-analyzer.zip"
Expand-Archive installer-analyzer.zip
.\installer-analyzer\installer-analyzer.exe --help

# Linux
curl -L "https://github.com/loonghao/installer-analyzer/releases/latest/download/installer-analyzer-linux-amd64.tar.gz" | tar xz
./installer-analyzer --help

# macOS
curl -L "https://github.com/loonghao/installer-analyzer/releases/latest/download/installer-analyzer-macos-amd64.tar.gz" | tar xz
./installer-analyzer --help
```

#### Option 2: Build from Source
```bash
git clone https://github.com/loonghao/installer-analyzer.git
cd installer-analyzer
cargo build --release
./target/release/installer-analyzer --help
```

### Basic Usage

```bash
# Analyze an installer package
installer-analyzer analyze setup.exe

# Generate HTML report with interactive file tree
installer-analyzer analyze setup.msi --output report.html

# JSON output for automation and CI/CD
installer-analyzer analyze app.msix --format json --output analysis.json

# Quick package information
installer-analyzer info package.exe

# List all supported formats
installer-analyzer formats

# Extract files from package
installer-analyzer extract setup.exe --output-dir ./extracted/
```

### Advanced Usage

```bash
# Analyze with custom output directory and verbose logging
installer-analyzer analyze installer.exe --output-dir ./analysis/ --verbose

# Batch analyze multiple files with parallel processing
installer-analyzer analyze *.msi *.exe --batch --parallel

# Generate both HTML and JSON reports
installer-analyzer analyze setup.exe --output report.html --format json --output analysis.json

# Extract files with filtering
installer-analyzer extract setup.exe --output-dir ./extracted/ --filter "*.dll,*.exe"

# Analyze with specific format hint (skip auto-detection)
installer-analyzer analyze package.exe --format nsis

# Generate report with custom template
installer-analyzer analyze app.msi --template custom.html --output custom-report.html

# Security-focused analysis
installer-analyzer analyze setup.exe --security-scan --verify-signatures

# Performance analysis with timing
installer-analyzer analyze large-package.exe --timing --memory-profile
```

## 📋 Supported Formats

| Format | Extensions | Detection | File Extraction | Registry Analysis | Metadata | Security Analysis |
|--------|------------|-----------|-----------------|-------------------|----------|-------------------|
| **MSI** | `.msi` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **WiX** | `.msi` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **NSIS** | `.exe` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Squirrel** | `.exe` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **InnoSetup** | `.exe` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **InstallShield** | `.exe` | ✅ | ⚠️ | ⚠️ | ✅ | ⚠️ |
| **MSIX/AppX** | `.msix`, `.appx` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Python Wheel** | `.whl` | ✅ | ✅ | ❌ | ✅ | ⚠️ |

**Legend**: ✅ Full Support | ⚠️ Basic Support | ❌ Not Applicable

### Format-Specific Features

- **MSI/WiX**: Complete database parsing, component analysis, feature detection
- **NSIS**: Script decompilation, plugin detection, custom page analysis
- **Squirrel**: Electron app detection, auto-updater analysis, framework identification
- **InnoSetup**: Script parsing, custom action detection, compression analysis
- **InstallShield**: Version detection, setup type identification, basic file listing
- **MSIX/AppX**: Manifest parsing, capability analysis, dependency resolution
- **Python Wheel**: Metadata extraction, dependency analysis, entry point detection

## 🏗️ Architecture

### Project Structure
```
installer-analyzer/
├── src/
│   ├── analyzers/          # Format-specific analyzers
│   │   ├── msi/            # MSI analyzer with database parsing
│   │   ├── wix/            # WiX analyzer with extension detection
│   │   ├── nsis/           # NSIS analyzer with script analysis
│   │   ├── squirrel/       # Squirrel analyzer for Electron apps
│   │   ├── inno/           # InnoSetup analyzer with script parsing
│   │   ├── installshield/  # InstallShield analyzer with version detection
│   │   ├── msix/           # MSIX/AppX analyzer with manifest parsing
│   │   ├── wheel/          # Python Wheel analyzer
│   │   ├── archive/        # Generic archive analyzer
│   │   └── common.rs       # Shared utilities and detection logic
│   ├── core/               # Core types, traits, and error handling
│   ├── reporting/          # Report generation and templating
│   ├── cli/                # Command-line interface
│   ├── api/                # Programmatic API (planned)
│   ├── sandbox/            # Sandbox environment (planned)
│   ├── monitoring/         # Runtime monitoring (planned)
│   └── utils/              # Utility functions and helpers
├── templates/              # HTML report templates and assets
├── tests/                  # Test data and test cases
├── scripts/                # Build and deployment scripts
└── docs/                   # Documentation (planned)
```

### Design Principles

- **Modular Architecture**: Each installer format has its own dedicated analyzer
- **Factory Pattern**: Intelligent format detection and analyzer selection
- **Trait-based Design**: Common `InstallerAnalyzer` trait for consistency
- **Async-first**: Full async/await support for I/O operations
- **Error Handling**: Comprehensive error types with graceful degradation
- **Performance**: Memory-efficient parsing with streaming support
- **Extensibility**: Plugin-ready architecture for custom analyzers

## 🔧 Development

### Prerequisites

- **Rust 1.70+** - Latest stable Rust toolchain
- **Git** - Version control
- **Platform-specific tools**:
  - Windows: MSVC Build Tools or Visual Studio
  - Linux: GCC or Clang, pkg-config
  - macOS: Xcode Command Line Tools

### Building from Source

```bash
# Clone the repository
git clone https://github.com/loonghao/installer-analyzer.git
cd installer-analyzer

# Build in debug mode (faster compilation)
cargo build

# Build optimized release version
cargo build --release

# Run tests to ensure everything works
cargo test

# Install locally for development
cargo install --path .
```

### Development Workflow

```bash
# Run all tests with verbose output
cargo test -- --nocapture

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test '*'

# Run specific analyzer tests
cargo test msi::tests
cargo test nsis::tests

# Run tests with coverage
cargo test --all-features

# Run doc tests
cargo test --doc

# Run with test data (example binaries)
cargo run --bin test_msi
cargo run --bin test_file_tree
cargo run --bin test_all_files

# Check code formatting
cargo fmt --all -- --check

# Run clippy for linting
cargo clippy --all-targets --all-features -- -D warnings

# Generate documentation
cargo doc --open
```

### Cross-Platform Building

```bash
# Install cross-compilation targets
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin

# Build for specific targets
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
```

## 📖 Documentation

### User Documentation
- [User Guide](docs/user-guide.md) - Comprehensive usage guide and tutorials
- [CLI Reference](docs/cli-reference.md) - Complete command-line interface documentation
- [Report Guide](docs/reports.md) - Understanding HTML and JSON reports
- [Format Support](docs/formats.md) - Detailed format support and capabilities

### Developer Documentation
- [Developer Guide](docs/developer-guide.md) - Development setup and contribution guide
- [API Reference](docs/api-reference.md) - Programmatic API documentation
- [Architecture Guide](docs/architecture.md) - System design and component overview
- [Adding Analyzers](docs/adding-analyzers.md) - How to add support for new formats

### Examples and Tutorials
- [Basic Usage Examples](examples/basic-usage.md) - Common use cases and examples
- [Advanced Scenarios](examples/advanced.md) - Complex analysis scenarios
- [CI/CD Integration](examples/cicd.md) - Integrating with build pipelines
- [Security Analysis](examples/security.md) - Security-focused analysis workflows

## 🤝 Contributing

We welcome contributions from the community! Whether you're fixing bugs, adding features, improving documentation, or suggesting new ideas, your help is appreciated.

### Quick Start for Contributors

1. **Fork** the repository on GitHub
2. **Clone** your fork locally: `git clone https://github.com/YOUR_USERNAME/installer-analyzer.git`
3. **Create** a feature branch: `git checkout -b feature/amazing-feature`
4. **Make** your changes and add tests
5. **Test** your changes: `cargo test`
6. **Commit** with conventional format: `git commit -m 'feat: add amazing feature'`
7. **Push** to your fork: `git push origin feature/amazing-feature`
8. **Open** a Pull Request with detailed description

### Ways to Contribute

- 🐛 **Bug Reports**: Found an issue? Please report it with details
- ✨ **Feature Requests**: Have an idea? We'd love to hear it
- 📝 **Documentation**: Help improve our docs and examples
- 🔧 **Code**: Fix bugs, add features, or improve performance
- 🧪 **Testing**: Add test cases or improve test coverage
- 🌍 **Localization**: Help translate to other languages

### Development Guidelines

- Follow Rust best practices and idioms
- Write tests for new functionality
- Update documentation for user-facing changes
- Use conventional commit messages
- Ensure CI passes before submitting PR

See our [Contributing Guide](CONTRIBUTING.md) for detailed information.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

This project builds upon the excellent work of the Rust community and several key libraries:

### Core Dependencies
- [msi-rs](https://crates.io/crates/msi) - MSI file parsing and database access
- [zip](https://crates.io/crates/zip) - Archive handling for various formats
- [serde](https://crates.io/crates/serde) - Serialization framework for JSON/YAML
- [tokio](https://crates.io/crates/tokio) - Async runtime for high-performance I/O
- [clap](https://crates.io/crates/clap) - Command line argument parsing

### Additional Libraries
- [handlebars](https://crates.io/crates/handlebars) - HTML template rendering
- [tracing](https://crates.io/crates/tracing) - Structured logging and diagnostics
- [chrono](https://crates.io/crates/chrono) - Date and time handling
- [uuid](https://crates.io/crates/uuid) - UUID generation for session tracking
- [sha2](https://crates.io/crates/sha2) - Cryptographic hashing for file integrity

### Inspiration
- [binwalk](https://github.com/ReFirmLabs/binwalk) - Firmware analysis tool
- [7-Zip](https://www.7-zip.org/) - Archive format support reference
- [NSIS](https://nsis.sourceforge.io/) - Installer system documentation

## 📞 Support & Community

### Get Help
- 🐛 [Report Issues](https://github.com/loonghao/installer-analyzer/issues) - Bug reports and feature requests
- 💬 [Discussions](https://github.com/loonghao/installer-analyzer/discussions) - Community Q&A and ideas
- 📚 [Documentation](https://github.com/loonghao/installer-analyzer/wiki) - Comprehensive guides and tutorials
- 📧 [Email Support](mailto:hal.long@outlook.com) - Direct support for complex issues

### Stay Updated
- ⭐ [Star the project](https://github.com/loonghao/installer-analyzer) to show support
- 👀 [Watch releases](https://github.com/loonghao/installer-analyzer/releases) for updates
- 🐦 Follow [@loonghao](https://github.com/loonghao) for project updates

---

## 📊 Project Stats

![GitHub stars](https://img.shields.io/github/stars/loonghao/installer-analyzer?style=social)
![GitHub forks](https://img.shields.io/github/forks/loonghao/installer-analyzer?style=social)
![GitHub issues](https://img.shields.io/github/issues/loonghao/installer-analyzer)
![GitHub pull requests](https://img.shields.io/github/issues-pr/loonghao/installer-analyzer)

---

Made with ❤️ by [loonghao](https://github.com/loonghao) and the open source community

**Ready for production use!** 🚀
