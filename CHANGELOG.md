# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-11-10

### Added
- Initial release of modular healthcheck system
- TCP port connectivity check with configurable timeout
- HTTP/HTTPS endpoint check with configurable timeout
- PostgreSQL database connection check with authentication
- Process existence check for Linux systems
- Config-based system using simple key=value format
- Custom parsers with minimal dependencies (no serde/TOML)
- Size-optimized binary (~517KB compressed with UPX)
- Cross-compilation support for ARM64 (aarch64-unknown-linux-musl)
- Comprehensive test suite (21 tests)
- Performance benchmarks using divan (4 benchmark suites)
- Code quality enforcement:
  - Rust Edition 2024
  - Min Rust version 1.91
  - Max 150 LOC per file
  - Clippy with zero warnings
  - MIT OR Apache-2.0 dual licensing

[Unreleased]: https://github.com/ryugen-io/healthcheck-rs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/ryugen-io/healthcheck-rs/releases/tag/v1.0.0
