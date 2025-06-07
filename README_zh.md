# 安装包分析器

[![CI](https://github.com/loonghao/installer-analyzer/workflows/CI/badge.svg)](https://github.com/loonghao/installer-analyzer/actions)
[![Release](https://github.com/loonghao/installer-analyzer/workflows/Release/badge.svg)](https://github.com/loonghao/installer-analyzer/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

[English](README.md) | [中文](README_zh.md)

一个用于分析软件安装包和监控安装行为的综合工具。支持多种安装包格式，提供详细分析和交互式报告。

## ✨ 功能特性

### 📦 多格式支持
- **MSI** - Microsoft 安装包
- **WiX** - WiX 工具集生成的 MSI 包
- **NSIS** - Nullsoft 脚本安装系统
- **Squirrel** - Electron 应用安装包
- **InnoSetup** - Inno Setup 安装包
- **InstallShield** - 企业级安装包
- **MSIX/AppX** - 现代 Windows 应用包
- **Python Wheel** - Python 包格式

### 🔍 分析能力
- **文件提取** - 提取和分析嵌入文件
- **注册表操作** - 检测注册表修改
- **元数据提取** - 产品信息、版本、发布者
- **安全分析** - 文件签名、证书验证
- **安装模拟** - 沙箱环境支持

### 📊 交互式报告
- **HTML 报告** - 现代化响应式网页界面
- **文件树视图** - 层次化文件结构，支持搜索
- **JSON 导出** - 机器可读的分析结果
- **可视化图表** - 文件类型分布和统计

### 🛠️ 开发者工具
- **CLI 界面** - 命令行分析工具
- **模块化架构** - 可扩展的分析器框架
- **跨平台** - 支持 Windows、Linux、macOS

## 🚀 快速开始

### 安装

从 [GitHub Releases](https://github.com/loonghao/installer-analyzer/releases) 下载最新版本：

```bash
# Windows
installer-analyzer.exe --help

# Linux/macOS  
./installer-analyzer --help
```

### 基本用法

```bash
# 分析安装包
installer-analyzer analyze setup.exe

# 生成 HTML 报告
installer-analyzer analyze setup.msi --output report.html

# JSON 输出用于自动化
installer-analyzer analyze app.msix --format json --output analysis.json

# 获取安装包信息
installer-analyzer info package.exe

# 列出支持的格式
installer-analyzer formats
```

### 高级用法

```bash
# 指定输出目录分析
installer-analyzer analyze installer.exe --output-dir ./analysis/

# 启用详细日志
installer-analyzer analyze setup.msi --verbose

# 批量分析多个文件
installer-analyzer analyze *.msi --batch

# 仅提取文件
installer-analyzer extract setup.exe --output-dir ./extracted/
```

## 📋 支持格式

| 格式 | 扩展名 | 检测 | 文件提取 | 注册表分析 | 元数据 |
|------|--------|------|----------|------------|--------|
| MSI | `.msi` | ✅ | ✅ | ✅ | ✅ |
| WiX | `.msi` | ✅ | ✅ | ✅ | ✅ |
| NSIS | `.exe` | ✅ | ✅ | ✅ | ✅ |
| Squirrel | `.exe` | ✅ | ✅ | ✅ | ✅ |
| InnoSetup | `.exe` | ✅ | ✅ | ✅ | ✅ |
| InstallShield | `.exe` | ✅ | ⚠️ | ⚠️ | ✅ |
| MSIX/AppX | `.msix`, `.appx` | ✅ | ✅ | ✅ | ✅ |
| Python Wheel | `.whl` | ✅ | ✅ | ❌ | ✅ |

**图例**: ✅ 完全支持 | ⚠️ 基础支持 | ❌ 不适用

## 🏗️ 架构

```
installer-analyzer/
├── src/
│   ├── analyzers/          # 格式特定分析器
│   │   ├── msi/            # MSI 分析器
│   │   ├── wix/            # WiX 分析器
│   │   ├── nsis/           # NSIS 分析器
│   │   ├── squirrel/       # Squirrel 分析器
│   │   ├── inno/           # InnoSetup 分析器
│   │   ├── installshield/  # InstallShield 分析器
│   │   ├── msix/           # MSIX/AppX 分析器
│   │   └── wheel/          # Python Wheel 分析器
│   ├── core/               # 核心类型和特征
│   ├── reporting/          # 报告生成
│   ├── sandbox/            # 沙箱环境
│   └── utils/              # 工具函数
├── templates/              # HTML 报告模板
└── tests/                  # 测试数据和用例
```

## 🔧 开发

### 前置要求

- Rust 1.70 或更高版本
- Git

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/loonghao/installer-analyzer.git
cd installer-analyzer

# 构建项目
cargo build --release

# 运行测试
cargo test

# 本地安装
cargo install --path .
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 仅运行单元测试
cargo test --lib

# 仅运行集成测试
cargo test --test '*'

# 运行特定分析器测试
cargo test msi
cargo test nsis

# 运行带覆盖率的测试
cargo test --all-features

# 运行文档测试
cargo test --doc

# 使用测试数据运行（示例二进制文件）
cargo run --bin test_msi
cargo run --bin test_file_tree
cargo run --bin test_all_files
```

## 📖 文档

- [用户指南](docs/user-guide.md) - 详细使用指南
- [开发者指南](docs/developer-guide.md) - 开发和贡献指南
- [API 参考](docs/api-reference.md) - API 文档
- [格式支持](docs/formats.md) - 详细格式支持信息

## 🤝 贡献

我们欢迎贡献！请查看我们的 [贡献指南](CONTRIBUTING.md) 了解详情。

### 开发流程

1. Fork 仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 进行更改
4. 为新功能添加测试
5. 确保所有测试通过 (`cargo test`)
6. 提交更改 (`git commit -m 'Add amazing feature'`)
7. 推送到分支 (`git push origin feature/amazing-feature`)
8. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [msi-rs](https://crates.io/crates/msi) - MSI 文件解析
- [zip](https://crates.io/crates/zip) - 压缩包处理
- [serde](https://crates.io/crates/serde) - 序列化框架
- [tokio](https://crates.io/crates/tokio) - 异步运行时
- [clap](https://crates.io/crates/clap) - 命令行解析

## 📞 支持

- 🐛 [报告问题](https://github.com/loonghao/installer-analyzer/issues)
- 💬 [讨论](https://github.com/loonghao/installer-analyzer/discussions)
- 📧 [邮件支持](mailto:hal.long@outlook.com)

---

由 [loonghao](https://github.com/loonghao) 用 ❤️ 制作
