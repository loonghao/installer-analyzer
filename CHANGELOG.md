# Changelog

## [0.7.0](https://github.com/loonghao/installer-analyzer/compare/v0.6.0...v0.7.0) (2025-06-16)


### Features

* add comprehensive auto-update functionality with Chocolatey support ([5888744](https://github.com/loonghao/installer-analyzer/commit/5888744672bb82f30ef090112248acbe80d8b2b8))


### Bug Fixes

* improve PGO build robustness and error handling ([bc9687a](https://github.com/loonghao/installer-analyzer/commit/bc9687a1f0f27665a81369b2018f8e330cefaaf9))

## [0.6.0](https://github.com/loonghao/installer-analyzer/compare/v0.5.0...v0.6.0) (2025-06-12)


### Features

* add HTML report example image and optimize help command performance ([0287bdc](https://github.com/loonghao/installer-analyzer/commit/0287bdc2ea54becc725f3e86235ec7e638ac7506))

## [0.5.0](https://github.com/loonghao/installer-analyzer/compare/v0.4.0...v0.5.0) (2025-06-10)


### Features

* add Profile-Guided Optimization (PGO) build support ([111ff2d](https://github.com/loonghao/installer-analyzer/commit/111ff2d03e8c9c55933df3ae0889d022acef733a))
* enhance UI with improved copy functionality and compact layout ([9f9a267](https://github.com/loonghao/installer-analyzer/commit/9f9a267cd02d59bd6c9ae5965e795ee974affece))
* implement unified data structure with original filename and SHA256 display ([0c16c97](https://github.com/loonghao/installer-analyzer/commit/0c16c972691fe547211f1bfb33c3f8f853b11dfb))
* restore modern web frontend with Bootstrap and macOS Finder style ([fc076f0](https://github.com/loonghao/installer-analyzer/commit/fc076f0cfc1935bf1bc43836464d6843603b1c02))


### Bug Fixes

* add missing frontend configuration files to Git ([683fa00](https://github.com/loonghao/installer-analyzer/commit/683fa004828f8e7627ba624f4538e953d16e472b))
* improve npm detection in build script for CI compatibility ([95c3c8d](https://github.com/loonghao/installer-analyzer/commit/95c3c8d8c87ac3b67b3c3a176faf0638135a003a))
* specify correct Rust toolchain for CI commands ([693b351](https://github.com/loonghao/installer-analyzer/commit/693b351cb4f94a7ada09f8786de8173b90d137fb))
* update GitHub Actions to use latest versions ([f52d98d](https://github.com/loonghao/installer-analyzer/commit/f52d98d8aeac1e306baa1238f59a2aec86110921))

## [0.4.0](https://github.com/loonghao/installer-analyzer/compare/v0.3.0...v0.4.0) (2025-06-09)


### Features

* implement enhanced metadata extraction system ([e4a89bc](https://github.com/loonghao/installer-analyzer/commit/e4a89bc90f45a2df846b8621d145a376d26c9c28))

## [0.3.0](https://github.com/loonghao/installer-analyzer/compare/v0.2.0...v0.3.0) (2025-06-08)


### Features

* add comprehensive code coverage analysis and improvement plan ([d31c39b](https://github.com/loonghao/installer-analyzer/commit/d31c39bde0850d27f1161d838bf526506d97411a))
* enhance CLI with cross-platform output and progress bars ([83c5064](https://github.com/loonghao/installer-analyzer/commit/83c50644743812ff57e1e0b407fcac4a2dbfb017))


### Bug Fixes

* optimize CI performance and resolve dependency issues ([a38545e](https://github.com/loonghao/installer-analyzer/commit/a38545ede646dfa9d0baae77b7b28aa18a218e52))
* resolve all clippy warnings for CI compliance ([1243d4d](https://github.com/loonghao/installer-analyzer/commit/1243d4dfe21b86c5a826c1358b38a874ab0ea3ec))

## [0.2.0](https://github.com/loonghao/installer-analyzer/compare/v0.1.0...v0.2.0) (2025-06-08)


### Features

* add browser auto-open and optimize CI build performance ([f86b802](https://github.com/loonghao/installer-analyzer/commit/f86b80281e19280180d6c45097287dca2f16e2ee))
* add comprehensive unit tests, CI improvements, and release automation ([fba68d1](https://github.com/loonghao/installer-analyzer/commit/fba68d125b3fdb59d3590e44fe2bdca19c949149))
* implement comprehensive installer analyzer with multi-format support ([5b9dc53](https://github.com/loonghao/installer-analyzer/commit/5b9dc53f42633c23e22a7aa4eee1e420f7615fd7))
* optimize CI for Windows-first development strategy ([e2e8a79](https://github.com/loonghao/installer-analyzer/commit/e2e8a79e2b9c75837f3cc82c298bfe5f477adf40))
* remove cross-platform support, focus on Windows-only ([5c8ccf8](https://github.com/loonghao/installer-analyzer/commit/5c8ccf8f423020452e5aa6c63a669c36a14c370c))


### Bug Fixes

* add missing template file to repository ([9f318f7](https://github.com/loonghao/installer-analyzer/commit/9f318f7ad5b3b1c2b96e83e9d2ae8842bfd483bc))
* add proper GitHub Actions permissions based on official documentation ([776b370](https://github.com/loonghao/installer-analyzer/commit/776b370ce4a257051df3baa22fd1dec75130ace8))
* **ci:** change primary CI to Windows-based builds ([6c20852](https://github.com/loonghao/installer-analyzer/commit/6c20852a235fc4f537aa0ab686fd8c427064aef5))
* **ci:** make Codecov upload non-blocking to prevent CI failures ([b58f14a](https://github.com/loonghao/installer-analyzer/commit/b58f14acb4b4b74b7d242f5f1d74b8bf2ac5c7b1))
* disable labeling in release-please to resolve permission issues ([82fe6bc](https://github.com/loonghao/installer-analyzer/commit/82fe6bcf9c200c91c1d08b8ea479665e5d406585))
* embed HTML template directly in code to resolve CI path issues ([35e8cd3](https://github.com/loonghao/installer-analyzer/commit/35e8cd37f5555e30ad27a1a2139f22572fc35040))
* optimize release workflow to prevent infinite loops and permission issues ([fe189f3](https://github.com/loonghao/installer-analyzer/commit/fe189f3dd0d238d71adb4559bb52c2ece20ccd79))
* resolve all clippy warnings and errors ([49313f8](https://github.com/loonghao/installer-analyzer/commit/49313f8c6da584cfab555e6634ec362b30842e11))
* resolve GitHub Actions permissions for release-please ([1684a7f](https://github.com/loonghao/installer-analyzer/commit/1684a7f112506530cb8accc39b59b1de69a666f3))
* resolve template embedding issue for CI builds ([7fe40b1](https://github.com/loonghao/installer-analyzer/commit/7fe40b11282dd12317e5f16a77c3164703cf70ac))
* resolve template path issues with relative path and build verification ([f02b820](https://github.com/loonghao/installer-analyzer/commit/f02b820e41ef8b52026ad37138a23889fa3d129c))
* update release-please action to latest version ([84c8243](https://github.com/loonghao/installer-analyzer/commit/84c8243947ecc48c5c35fceaa187f0fc2fcf1387))
* use external template file with reliable path resolution ([ba7acce](https://github.com/loonghao/installer-analyzer/commit/ba7acce83e01b2ff46ee7e1fa7ba6f745577edd7))

## 0.1.0 (2025-06-08)


### Features

* add browser auto-open and optimize CI build performance ([f86b802](https://github.com/loonghao/installer-analyzer/commit/f86b80281e19280180d6c45097287dca2f16e2ee))
* add comprehensive unit tests, CI improvements, and release automation ([fba68d1](https://github.com/loonghao/installer-analyzer/commit/fba68d125b3fdb59d3590e44fe2bdca19c949149))
* implement comprehensive installer analyzer with multi-format support ([5b9dc53](https://github.com/loonghao/installer-analyzer/commit/5b9dc53f42633c23e22a7aa4eee1e420f7615fd7))
* optimize CI for Windows-first development strategy ([e2e8a79](https://github.com/loonghao/installer-analyzer/commit/e2e8a79e2b9c75837f3cc82c298bfe5f477adf40))
* remove cross-platform support, focus on Windows-only ([5c8ccf8](https://github.com/loonghao/installer-analyzer/commit/5c8ccf8f423020452e5aa6c63a669c36a14c370c))


### Bug Fixes

* add missing template file to repository ([9f318f7](https://github.com/loonghao/installer-analyzer/commit/9f318f7ad5b3b1c2b96e83e9d2ae8842bfd483bc))
* **ci:** change primary CI to Windows-based builds ([6c20852](https://github.com/loonghao/installer-analyzer/commit/6c20852a235fc4f537aa0ab686fd8c427064aef5))
* **ci:** make Codecov upload non-blocking to prevent CI failures ([b58f14a](https://github.com/loonghao/installer-analyzer/commit/b58f14acb4b4b74b7d242f5f1d74b8bf2ac5c7b1))
* embed HTML template directly in code to resolve CI path issues ([35e8cd3](https://github.com/loonghao/installer-analyzer/commit/35e8cd37f5555e30ad27a1a2139f22572fc35040))
* resolve all clippy warnings and errors ([49313f8](https://github.com/loonghao/installer-analyzer/commit/49313f8c6da584cfab555e6634ec362b30842e11))
* resolve GitHub Actions permissions for release-please ([1684a7f](https://github.com/loonghao/installer-analyzer/commit/1684a7f112506530cb8accc39b59b1de69a666f3))
* resolve template embedding issue for CI builds ([7fe40b1](https://github.com/loonghao/installer-analyzer/commit/7fe40b11282dd12317e5f16a77c3164703cf70ac))
* resolve template path issues with relative path and build verification ([f02b820](https://github.com/loonghao/installer-analyzer/commit/f02b820e41ef8b52026ad37138a23889fa3d129c))
* update release-please action to latest version ([84c8243](https://github.com/loonghao/installer-analyzer/commit/84c8243947ecc48c5c35fceaa187f0fc2fcf1387))
* use external template file with reliable path resolution ([ba7acce](https://github.com/loonghao/installer-analyzer/commit/ba7acce83e01b2ff46ee7e1fa7ba6f745577edd7))
