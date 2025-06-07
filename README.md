# Installer Analyzer

[![CI](https://github.com/loonghao/installer-analyzer/workflows/CI/badge.svg)](https://github.com/loonghao/installer-analyzer/actions)
[![Release](https://github.com/loonghao/installer-analyzer/workflows/Release/badge.svg)](https://github.com/loonghao/installer-analyzer/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

[English](README.md) | [中文](README_zh.md)

A comprehensive tool for analyzing software installation packages and monitoring installation behavior. Supports multiple installer formats with detailed analysis and interactive reporting.

## ✨ Features

### 📦 Multi-Format Support
- **MSI** - Microsoft Installer packages
- **WiX** - WiX Toolset generated MSI packages  
- **NSIS** - Nullsoft Scriptable Install System
- **Squirrel** - Electron application installers
- **InnoSetup** - Inno Setup installers
- **InstallShield** - Enterprise installation packages
- **MSIX/AppX** - Modern Windows app packages
- **Python Wheel** - Python package format

### 🔍 Analysis Capabilities
- **File Extraction** - Extract and analyze embedded files
- **Registry Operations** - Detect registry modifications
- **Metadata Extraction** - Product info, version, publisher
- **Security Analysis** - File signatures, certificates
- **Installation Simulation** - Sandbox environment support

### 📊 Interactive Reporting
- **HTML Reports** - Modern, responsive web interface
- **File Tree View** - Hierarchical file structure with search
- **JSON Export** - Machine-readable analysis results
- **Visual Charts** - File type distribution and statistics

### 🛠️ Developer Tools
- **CLI Interface** - Command-line analysis tool
- **Modular Architecture** - Extensible analyzer framework
- **Cross-Platform** - Windows, Linux, macOS support

## 🚀 Quick Start

### Installation

Download the latest release from [GitHub Releases](https://github.com/loonghao/installer-analyzer/releases):

```bash
# Windows
installer-analyzer.exe --help

# Linux/macOS  
./installer-analyzer --help
```

### Basic Usage

```bash
# Analyze an installer package
installer-analyzer analyze setup.exe

# Generate HTML report
installer-analyzer analyze setup.msi --output report.html

# JSON output for automation
installer-analyzer analyze app.msix --format json --output analysis.json

# Get installer information
installer-analyzer info package.exe

# List supported formats
installer-analyzer formats
```

### Advanced Usage

```bash
# Analyze with custom output directory
installer-analyzer analyze installer.exe --output-dir ./analysis/

# Enable verbose logging
installer-analyzer analyze setup.msi --verbose

# Analyze multiple files
installer-analyzer analyze *.msi --batch

# Extract files only
installer-analyzer extract setup.exe --output-dir ./extracted/
```

## 📋 Supported Formats

| Format | Extension | Detection | File Extraction | Registry Analysis | Metadata |
|--------|-----------|-----------|-----------------|-------------------|----------|
| MSI | `.msi` | ✅ | ✅ | ✅ | ✅ |
| WiX | `.msi` | ✅ | ✅ | ✅ | ✅ |
| NSIS | `.exe` | ✅ | ✅ | ✅ | ✅ |
| Squirrel | `.exe` | ✅ | ✅ | ✅ | ✅ |
| InnoSetup | `.exe` | ✅ | ✅ | ✅ | ✅ |
| InstallShield | `.exe` | ✅ | ⚠️ | ⚠️ | ✅ |
| MSIX/AppX | `.msix`, `.appx` | ✅ | ✅ | ✅ | ✅ |
| Python Wheel | `.whl` | ✅ | ✅ | ❌ | ✅ |

**Legend**: ✅ Full Support | ⚠️ Basic Support | ❌ Not Applicable

## 🏗️ Architecture

```
installer-analyzer/
├── src/
│   ├── analyzers/          # Format-specific analyzers
│   │   ├── msi/            # MSI analyzer
│   │   ├── wix/            # WiX analyzer  
│   │   ├── nsis/           # NSIS analyzer
│   │   ├── squirrel/       # Squirrel analyzer
│   │   ├── inno/           # InnoSetup analyzer
│   │   ├── installshield/  # InstallShield analyzer
│   │   ├── msix/           # MSIX/AppX analyzer
│   │   └── wheel/          # Python Wheel analyzer
│   ├── core/               # Core types and traits
│   ├── reporting/          # Report generation
│   ├── sandbox/            # Sandbox environment
│   └── utils/              # Utility functions
├── templates/              # HTML report templates
└── tests/                  # Test data and cases
```

## 🔧 Development

### Prerequisites

- Rust 1.70 or later
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/loonghao/installer-analyzer.git
cd installer-analyzer

# Build the project
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific analyzer tests
cargo test msi
cargo test nsis

# Run with test data
cargo run --bin test_msi
cargo run --bin test_file_tree
```

## 📖 Documentation

- [User Guide](docs/user-guide.md) - Comprehensive usage guide
- [Developer Guide](docs/developer-guide.md) - Development and contribution guide
- [API Reference](docs/api-reference.md) - API documentation
- [Format Support](docs/formats.md) - Detailed format support information

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [msi-rs](https://crates.io/crates/msi) - MSI file parsing
- [zip](https://crates.io/crates/zip) - Archive handling
- [serde](https://crates.io/crates/serde) - Serialization framework
- [tokio](https://crates.io/crates/tokio) - Async runtime
- [clap](https://crates.io/crates/clap) - Command line parsing

## 📞 Support

- 🐛 [Report Issues](https://github.com/loonghao/installer-analyzer/issues)
- 💬 [Discussions](https://github.com/loonghao/installer-analyzer/discussions)
- 📧 [Email Support](mailto:hal.long@outlook.com)

---

Made with ❤️ by [loonghao](https://github.com/loonghao)

---

**Ready for review and testing!** 🚀
