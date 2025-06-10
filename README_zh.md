# 安装包分析器

[![CI](https://github.com/loonghao/installer-analyzer/workflows/CI/badge.svg)](https://github.com/loonghao/installer-analyzer/actions)
[![Release](https://github.com/loonghao/installer-analyzer/workflows/Release/badge.svg)](https://github.com/loonghao/installer-analyzer/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Downloads](https://img.shields.io/github/downloads/loonghao/installer-analyzer/total.svg)](https://github.com/loonghao/installer-analyzer/releases)
[![Stars](https://img.shields.io/github/stars/loonghao/installer-analyzer.svg)](https://github.com/loonghao/installer-analyzer/stargazers)

[English](README.md) | [中文](README_zh.md)

一个功能全面的 Windows 软件安装包分析工具，用于分析软件安装包和监控安装行为。支持8种主流安装包格式，提供详细分析、交互式报告和现代化的Web可视化界面。

## ✨ 功能特性

### 📦 多格式支持（8种格式）
- **MSI** - Microsoft 安装包，支持数据库解析
- **WiX** - WiX 工具集生成的 MSI 包，支持扩展检测
- **NSIS** - Nullsoft 脚本安装系统，支持脚本分析
- **Squirrel** - Electron 应用安装包，支持自动更新检测
- **InnoSetup** - Inno Setup 安装包，支持脚本解析
- **InstallShield** - 企业级安装包，支持版本检测
- **MSIX/AppX** - 现代 Windows 应用包，支持清单解析
- **Python Wheel** - Python 包格式，支持元数据提取

### 🔍 高级分析能力
- **文件提取** - 提取和分析嵌入文件，支持类型检测
- **注册表操作** - 检测和分析注册表修改
- **元数据提取** - 产品信息、版本、发布者、证书
- **安全分析** - 文件签名、数字证书、信任验证
- **安装模拟** - 沙箱环境支持（计划中）
- **依赖分析** - 识别包依赖关系和要求

### 📊 交互式报告与可视化
- **现代化 HTML 报告** - 基于 Bootstrap 5 的响应式网页界面
- **交互式文件树** - 层次化结构，支持展开/折叠
- **实时搜索** - 即时过滤文件和目录
- **可视化图表** - 文件类型分布、大小统计和趋势
- **JSON 导出** - 机器可读的分析结果，支持自动化
- **详细元数据** - 全面的包信息显示

### 🛠️ 开发者与企业工具
- **强大的 CLI** - 命令行界面，支持多种分析模式
- **模块化架构** - 可扩展的分析器框架，支持插件
- **Windows 原生** - 专为 Windows 环境设计
- **批量处理** - 同时分析多个包
- **API 集成** - 程序化访问，支持 CI/CD 流水线
- **性能优化** - 高效内存使用和快速处理

## 🚀 快速开始

### 安装

#### 方式一：下载预编译二进制文件
从 [GitHub Releases](https://github.com/loonghao/installer-analyzer/releases) 下载最新版本：

```powershell
# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/loonghao/installer-analyzer/releases/latest/download/installer-analyzer-windows-amd64.exe" -OutFile "installer-analyzer.exe"
.\installer-analyzer.exe --help
```

#### 方式二：从源码构建
```bash
git clone https://github.com/loonghao/installer-analyzer.git
cd installer-analyzer
cargo build --release
./target/release/installer-analyzer --help
```

### 基本用法

```bash
# 分析安装包
installer-analyzer analyze setup.exe

# 生成带交互式文件树的 HTML 报告
installer-analyzer analyze setup.msi --output report.html

# JSON 输出用于自动化和 CI/CD
installer-analyzer analyze app.msix --format json --output analysis.json

# 快速获取包信息
installer-analyzer info package.exe

# 列出所有支持的格式
installer-analyzer formats

# 从包中提取文件
installer-analyzer extract setup.exe --output-dir ./extracted/
```

### 高级用法

```bash
# 自定义输出目录和详细日志
installer-analyzer analyze installer.exe --output-dir ./analysis/ --verbose

# 并行批量分析多个文件
installer-analyzer analyze *.msi *.exe --batch --parallel

# 同时生成 HTML 和 JSON 报告
installer-analyzer analyze setup.exe --output report.html --format json --output analysis.json

# 带过滤器的文件提取
installer-analyzer extract setup.exe --output-dir ./extracted/ --filter "*.dll,*.exe"

# 指定格式分析（跳过自动检测）
installer-analyzer analyze package.exe --format nsis

# 使用自定义模板生成报告
installer-analyzer analyze app.msi --template custom.html --output custom-report.html

# 安全性分析
installer-analyzer analyze setup.exe --security-scan --verify-signatures

# 性能分析和计时
installer-analyzer analyze large-package.exe --timing --memory-profile
```

## 📋 支持格式

| 格式 | 扩展名 | 检测 | 文件提取 | 注册表分析 | 元数据 | 安全分析 |
|------|--------|------|----------|------------|--------|----------|
| **MSI** | `.msi` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **WiX** | `.msi` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **NSIS** | `.exe` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Squirrel** | `.exe` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **InnoSetup** | `.exe` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **InstallShield** | `.exe` | ✅ | ⚠️ | ⚠️ | ✅ | ⚠️ |
| **MSIX/AppX** | `.msix`, `.appx` | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Python Wheel** | `.whl` | ✅ | ✅ | ❌ | ✅ | ⚠️ |

**图例**: ✅ 完全支持 | ⚠️ 基础支持 | ❌ 不适用

### 格式特定功能

- **MSI/WiX**: 完整数据库解析、组件分析、功能检测
- **NSIS**: 脚本反编译、插件检测、自定义页面分析
- **Squirrel**: Electron 应用检测、自动更新器分析、框架识别
- **InnoSetup**: 脚本解析、自定义操作检测、压缩分析
- **InstallShield**: 版本检测、安装类型识别、基础文件列表
- **MSIX/AppX**: 清单解析、功能分析、依赖解析
- **Python Wheel**: 元数据提取、依赖分析、入口点检测

## 🏗️ 架构

### 项目结构
```
installer-analyzer/
├── src/
│   ├── analyzers/          # 格式特定分析器
│   │   ├── msi/            # MSI 分析器，支持数据库解析
│   │   ├── wix/            # WiX 分析器，支持扩展检测
│   │   ├── nsis/           # NSIS 分析器，支持脚本分析
│   │   ├── squirrel/       # Squirrel 分析器，用于 Electron 应用
│   │   ├── inno/           # InnoSetup 分析器，支持脚本解析
│   │   ├── installshield/  # InstallShield 分析器，支持版本检测
│   │   ├── msix/           # MSIX/AppX 分析器，支持清单解析
│   │   ├── wheel/          # Python Wheel 分析器
│   │   ├── archive/        # 通用压缩包分析器
│   │   └── common.rs       # 共享工具和检测逻辑
│   ├── core/               # 核心类型、特征和错误处理
│   ├── reporting/          # 报告生成和模板
│   ├── cli/                # 命令行界面
│   ├── api/                # 程序化 API（计划中）
│   ├── sandbox/            # 沙箱环境（计划中）
│   ├── monitoring/         # 运行时监控（计划中）
│   └── utils/              # 工具函数和助手
├── templates/              # HTML 报告模板和资源
├── tests/                  # 测试数据和测试用例
├── scripts/                # 构建和部署脚本
└── docs/                   # 文档（计划中）
```

### 设计原则

- **模块化架构**: 每种安装包格式都有专用的分析器
- **工厂模式**: 智能格式检测和分析器选择
- **基于特征的设计**: 通用的 `InstallerAnalyzer` 特征保证一致性
- **异步优先**: 完整的 async/await 支持用于 I/O 操作
- **错误处理**: 全面的错误类型和优雅降级
- **性能**: 内存高效解析和流式支持
- **可扩展性**: 插件就绪架构，支持自定义分析器

## 🔧 开发

### 前置要求

- **Rust 1.70+** - 最新稳定版 Rust 工具链
- **Git** - 版本控制
- **Windows 构建工具** - MSVC 构建工具或 Visual Studio

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/loonghao/installer-analyzer.git
cd installer-analyzer

# 调试模式构建（编译更快）
cargo build

# 优化的发布版本构建
cargo build --release

# 运行测试确保一切正常
cargo test

# 本地安装用于开发
cargo install --path .
```

### 开发工作流

```bash
# 运行所有测试，带详细输出
cargo test -- --nocapture

# 仅运行单元测试
cargo test --lib

# 仅运行集成测试
cargo test --test '*'

# 运行特定分析器测试
cargo test msi::tests
cargo test nsis::tests

# 运行带覆盖率的测试
cargo test --all-features

# 运行文档测试
cargo test --doc

# 使用测试数据运行（示例二进制文件）
cargo run --bin test_msi
cargo run --bin test_file_tree
cargo run --bin test_all_files

# 检查代码格式
cargo fmt --all -- --check

# 运行 clippy 进行代码检查
cargo clippy --all-targets --all-features -- -D warnings

# 生成文档
cargo doc --open
```

### Windows 构建

```bash
# 为 Windows 构建（默认目标）
cargo build --release

# 为特定 Windows 目标构建
cargo build --release --target x86_64-pc-windows-msvc
```

## 📖 文档

### 用户文档
- [用户指南](docs/user-guide.md) - 全面的使用指南和教程
- [CLI 参考](docs/cli-reference.md) - 完整的命令行界面文档
- [报告指南](docs/reports.md) - 理解 HTML 和 JSON 报告
- [格式支持](docs/formats.md) - 详细的格式支持和功能

### 开发者文档
- [开发者指南](docs/developer-guide.md) - 开发设置和贡献指南
- [API 参考](docs/api-reference.md) - 程序化 API 文档
- [架构指南](docs/architecture.md) - 系统设计和组件概述
- [添加分析器](docs/adding-analyzers.md) - 如何添加新格式支持

### 示例和教程
- [基础用法示例](examples/basic-usage.md) - 常见用例和示例
- [高级场景](examples/advanced.md) - 复杂分析场景
- [CI/CD 集成](examples/cicd.md) - 与构建流水线集成
- [安全分析](examples/security.md) - 安全性分析工作流

## 🤝 贡献

我们欢迎社区贡献！无论您是修复错误、添加功能、改进文档还是提出新想法，我们都非常感谢您的帮助。

### 贡献者快速开始

1. **Fork** 在 GitHub 上 Fork 仓库
2. **克隆** 本地克隆您的 Fork: `git clone https://github.com/YOUR_USERNAME/installer-analyzer.git`
3. **创建** 功能分支: `git checkout -b feature/amazing-feature`
4. **修改** 进行更改并添加测试
5. **测试** 测试您的更改: `cargo test`
6. **提交** 使用约定格式: `git commit -m 'feat: add amazing feature'`
7. **推送** 到您的 Fork: `git push origin feature/amazing-feature`
8. **开启** 带详细描述的 Pull Request

### 贡献方式

- 🐛 **错误报告**: 发现问题？请详细报告
- ✨ **功能请求**: 有想法？我们很乐意听到
- 📝 **文档**: 帮助改进我们的文档和示例
- 🔧 **代码**: 修复错误、添加功能或改进性能
- 🧪 **测试**: 添加测试用例或改进测试覆盖率
- 🌍 **本地化**: 帮助翻译到其他语言

### 开发指南

- 遵循 Rust 最佳实践和习惯用法
- 为新功能编写测试
- 为面向用户的更改更新文档
- 使用约定式提交消息
- 确保 CI 在提交 PR 前通过

查看我们的 [贡献指南](CONTRIBUTING.md) 了解详细信息。

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

本项目基于 Rust 社区和几个关键库的优秀工作：

### 核心依赖
- [msi-rs](https://crates.io/crates/msi) - MSI 文件解析和数据库访问
- [zip](https://crates.io/crates/zip) - 各种格式的压缩包处理
- [serde](https://crates.io/crates/serde) - JSON/YAML 序列化框架
- [tokio](https://crates.io/crates/tokio) - 高性能 I/O 异步运行时
- [clap](https://crates.io/crates/clap) - 命令行参数解析

### 其他库
- [tracing](https://crates.io/crates/tracing) - 结构化日志和诊断
- [chrono](https://crates.io/crates/chrono) - 日期和时间处理
- [uuid](https://crates.io/crates/uuid) - 会话跟踪的 UUID 生成
- [sha2](https://crates.io/crates/sha2) - 文件完整性的加密哈希

### 灵感来源
- [binwalk](https://github.com/ReFirmLabs/binwalk) - 固件分析工具
- [7-Zip](https://www.7-zip.org/) - 压缩格式支持参考
- [NSIS](https://nsis.sourceforge.io/) - 安装系统文档

## 📞 支持与社区

### 获取帮助
- 🐛 [报告问题](https://github.com/loonghao/installer-analyzer/issues) - 错误报告和功能请求
- 💬 [讨论](https://github.com/loonghao/installer-analyzer/discussions) - 社区问答和想法
- 📚 [文档](https://github.com/loonghao/installer-analyzer/wiki) - 全面的指南和教程
- 📧 [邮件支持](mailto:hal.long@outlook.com) - 复杂问题的直接支持

### 保持更新
- ⭐ [为项目加星](https://github.com/loonghao/installer-analyzer) 表示支持
- 👀 [关注发布](https://github.com/loonghao/installer-analyzer/releases) 获取更新
- 🐦 关注 [@loonghao](https://github.com/loonghao) 获取项目更新

---

## 📊 项目统计

![GitHub stars](https://img.shields.io/github/stars/loonghao/installer-analyzer?style=social)
![GitHub forks](https://img.shields.io/github/forks/loonghao/installer-analyzer?style=social)
![GitHub issues](https://img.shields.io/github/issues/loonghao/installer-analyzer)
![GitHub pull requests](https://img.shields.io/github/issues-pr/loonghao/installer-analyzer)

---

由 [loonghao](https://github.com/loonghao) 和开源社区用 ❤️ 制作

**已准备好用于生产环境！** 🚀
