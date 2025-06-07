# Contributing to Installer Analyzer

Thank you for your interest in contributing to Installer Analyzer! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A GitHub account

### Setting up the Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/installer-analyzer.git
   cd installer-analyzer
   ```

3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/loonghao/installer-analyzer.git
   ```

4. Install dependencies and build:
   ```bash
   cargo build
   ```

5. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## 🔄 Development Workflow

### Creating a Feature Branch

1. Sync with upstream:
   ```bash
   git checkout main
   git pull upstream main
   ```

2. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

### Making Changes

1. Make your changes in logical, atomic commits
2. Write or update tests for your changes
3. Ensure all tests pass:
   ```bash
   cargo test
   ```

4. Check code formatting:
   ```bash
   cargo fmt --all -- --check
   ```

5. Run clippy for linting:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

### Submitting Changes

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Create a Pull Request on GitHub
3. Fill out the PR template with relevant information
4. Wait for review and address any feedback

## 📝 Coding Standards

### Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and reasonably sized

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(msi): add support for MSI v5.0 format
fix(nsis): handle corrupted NSIS headers gracefully
docs: update installation instructions
```

### Testing

- Write unit tests for new functionality
- Update existing tests when modifying behavior
- Ensure all tests pass before submitting
- Add integration tests for new analyzers

### Documentation

- Update README.md if adding new features
- Add inline documentation for complex code
- Update API documentation for public interfaces

## 🏗️ Architecture Guidelines

### Adding New Analyzers

When adding support for a new installer format:

1. Create a new module in `src/analyzers/`
2. Implement the `InstallerAnalyzer` trait
3. Add format detection logic
4. Update `AnalyzerFactory` to include the new analyzer
5. Add comprehensive tests
6. Update documentation

### Code Organization

```
src/
├── analyzers/          # Format-specific analyzers
│   ├── common.rs      # Shared utilities
│   ├── mod.rs         # Analyzer factory
│   └── format/        # Individual format analyzers
├── core/              # Core types and traits
├── reporting/         # Report generation
├── sandbox/           # Sandbox functionality
└── utils/             # General utilities
```

## 🐛 Reporting Issues

### Bug Reports

When reporting bugs, please include:

- Operating system and version
- Rust version (`rustc --version`)
- Installer Analyzer version
- Steps to reproduce the issue
- Expected vs actual behavior
- Sample files (if possible and safe to share)

### Feature Requests

For feature requests, please provide:

- Clear description of the feature
- Use case and motivation
- Proposed implementation approach (if any)
- Examples of similar features in other tools

## 📋 Pull Request Guidelines

### Before Submitting

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if applicable)

### PR Description

Please include:

- Summary of changes
- Motivation and context
- Type of change (bug fix, feature, etc.)
- Testing performed
- Screenshots (for UI changes)

### Review Process

1. Automated checks must pass
2. At least one maintainer review required
3. Address review feedback
4. Squash commits if requested
5. Maintainer will merge when ready

## 🏷️ Release Process

Releases are handled by maintainers:

1. Version bump in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create release tag
4. GitHub Actions builds and publishes binaries

## 📞 Getting Help

- 💬 [GitHub Discussions](https://github.com/loonghao/installer-analyzer/discussions)
- 🐛 [GitHub Issues](https://github.com/loonghao/installer-analyzer/issues)
- 📧 Email: hal.long@outlook.com

## 📄 License

By contributing to Installer Analyzer, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Installer Analyzer! 🎉
