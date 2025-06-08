# Code Coverage Improvement Plan

## Current Coverage Status

### Overall Coverage: 7.18%
- **Total Lines**: 4433
- **Covered Lines**: 1662 (6.11%)
- **Total Functions**: 516
- **Covered Functions**: 488 (5.43%)

## Module Coverage Analysis

### ✅ Well-Covered Modules
| Module | Line Coverage | Function Coverage | Status |
|--------|---------------|-------------------|---------|
| `cli/commands.rs` | 45.39% | 63.33% | 🟢 Good |
| `cli/output.rs` | 42.07% | 34.78% | 🟡 Moderate |
| `core/error.rs` | 7.58% | 7.14% | 🟡 Basic |

### ❌ Zero Coverage Modules (High Priority)
| Module | Lines | Functions | Priority |
|--------|-------|-----------|----------|
| `analyzers/msi/analyzer.rs` | 107 | 19 | 🔴 Critical |
| `analyzers/archive/analyzer.rs` | 84 | 21 | 🔴 Critical |
| `analyzers/wheel/analyzer.rs` | 85 | 22 | 🔴 Critical |
| `reporting/generator.rs` | 281 | 27 | 🟡 High |
| `reporting/templates.rs` | 437 | 28 | 🟡 High |
| `sandbox/controller.rs` | 28 | 7 | 🟡 Medium |

## Coverage Improvement Strategy

### Phase 1: Core Analyzer Testing (Priority 1)
**Target: 60% coverage for analyzer modules**

#### 1.1 MSI Analyzer Tests
```rust
// tests/unit/analyzers/msi_tests.rs
#[test]
fn test_msi_analyzer_can_analyze() {
    let analyzer = MsiAnalyzer::new();
    assert!(analyzer.can_analyze(&PathBuf::from("test.msi")));
    assert!(!analyzer.can_analyze(&PathBuf::from("test.exe")));
}

#[test]
fn test_msi_metadata_extraction() {
    // Test with real MSI file from tests/data
}
```

#### 1.2 Archive Analyzer Tests
```rust
// tests/unit/analyzers/archive_tests.rs
#[test]
fn test_archive_format_detection() {
    let analyzer = ArchiveAnalyzer::new();
    // Test ZIP, 7Z, RAR format detection
}
```

#### 1.3 Wheel Analyzer Tests
```rust
// tests/unit/analyzers/wheel_tests.rs
#[test]
fn test_wheel_metadata_parsing() {
    // Test Python wheel metadata extraction
}
```

### Phase 2: Error Handling Coverage (Priority 2)
**Target: 80% coverage for error paths**

#### 2.1 CLI Error Scenarios
- Invalid file paths
- Unsupported formats
- Permission errors
- Network timeouts (sandbox)

#### 2.2 Analyzer Error Scenarios
- Corrupted files
- Missing dependencies
- Memory limitations
- Invalid file structures

### Phase 3: Integration Testing (Priority 3)
**Target: 50% coverage for integration modules**

#### 3.1 Reporting Module Tests
```rust
// tests/unit/reporting/generator_tests.rs
#[test]
fn test_html_report_generation() {
    let generator = ReportGenerator::new();
    let analysis_result = create_mock_analysis_result();
    let html = generator.generate_html(&analysis_result).unwrap();
    assert!(html.contains("<html>"));
}
```

#### 3.2 Sandbox Controller Tests
```rust
// tests/unit/sandbox/controller_tests.rs
#[test]
fn test_sandbox_initialization() {
    let controller = SandboxController::new();
    // Test sandbox setup and teardown
}
```

## Testing Infrastructure Improvements

### Mock Data Framework
```rust
// tests/common/mock_data.rs
pub fn create_mock_msi_analysis() -> AnalysisResult {
    AnalysisResult {
        metadata: create_mock_metadata(),
        files: create_mock_file_list(),
        registry_operations: vec![],
    }
}
```

### Test Utilities
```rust
// tests/common/test_utils.rs
pub fn create_temp_installer(content: &[u8], extension: &str) -> PathBuf {
    // Create temporary test files
}

pub fn assert_valid_json_output(output: &str) {
    // Validate JSON structure
}
```

## Coverage Targets by Timeline

### Week 1: Analyzer Core (Target: 40% overall)
- [ ] MSI analyzer unit tests
- [ ] Archive analyzer unit tests  
- [ ] Wheel analyzer unit tests
- [ ] Basic error handling tests

### Week 2: CLI Enhancement (Target: 60% overall)
- [ ] Complete CLI command coverage
- [ ] Error scenario testing
- [ ] Format detection edge cases
- [ ] Browser opening functionality

### Week 3: Integration & Reporting (Target: 70% overall)
- [ ] Report generation tests
- [ ] Template rendering tests
- [ ] Sandbox controller tests
- [ ] End-to-end integration tests

### Week 4: Edge Cases & Polish (Target: 80% overall)
- [ ] Error recovery scenarios
- [ ] Performance edge cases
- [ ] Memory limitation handling
- [ ] Cross-platform compatibility

## Automated Coverage Monitoring

### CI Integration
```yaml
# .github/workflows/coverage.yml
- name: Generate Coverage Report
  run: |
    cargo install cargo-llvm-cov
    cargo llvm-cov --all-features --workspace --lcov --output-path coverage.lcov
    
- name: Upload to Codecov
  uses: codecov/codecov-action@v3
  with:
    file: coverage.lcov
```

### Coverage Gates
- **Minimum overall coverage**: 70%
- **New code coverage**: 80%
- **Critical modules coverage**: 60%

## Tools and Commands

### Generate Coverage Report
```bash
# Fast coverage (exclude slow tests)
./scripts/coverage.ps1

# Full coverage (include all tests)
./scripts/coverage.ps1 -Full

# Open HTML report
./scripts/coverage.ps1 -Open

# CI mode (LCOV only)
./scripts/coverage.ps1 -CI
```

### Manual Coverage Commands
```bash
# HTML report
cargo llvm-cov --lib --html --ignore-filename-regex="real_data_tests"

# LCOV report
cargo llvm-cov --lib --lcov --output-path coverage.lcov

# Text summary
cargo llvm-cov --lib --ignore-filename-regex="real_data_tests"
```

## Success Metrics

### Coverage Quality Indicators
1. **Line Coverage**: >70% for critical modules
2. **Function Coverage**: >60% for all modules
3. **Branch Coverage**: >50% for decision points
4. **Integration Coverage**: >40% for end-to-end flows

### Testing Quality Indicators
1. **Test Reliability**: <1% flaky test rate
2. **Test Performance**: <30s for full test suite
3. **Test Maintainability**: Clear, documented test cases
4. **Real-world Validation**: Tests with actual installer files

This plan provides a structured approach to achieving comprehensive code coverage while maintaining test quality and development velocity.
