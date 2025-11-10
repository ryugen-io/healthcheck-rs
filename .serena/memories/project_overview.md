# Healthcheck Project Overview

## Purpose
Modular, config-driven healthcheck system for Docker containers. Supports multiple check types (TCP, HTTP, database, process) configured via simple key=value config files. Designed for minimal dependencies and maximum flexibility.

## Tech Stack
- **Language**: Rust
- **Edition**: 2024
- **Minimum Rust Version**: 1.91
- **License**: MIT OR Apache-2.0
- **Architecture**: Workspace with two crates:
  - `health-core`: Library with probe implementations, registry pattern, config parsing
  - `health-bin`: Binary that reads config and executes checks

## Key Dependencies
- `log` (0.4): Logging facade
- `postgres` (0.19.12): PostgreSQL client for database checks
- `divan` (0.1): Benchmarking framework (dev-dependency)

## Build Configuration
- Optimized for size: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`
- Binary is built with `cargo auditable` and compressed with UPX
- Final binary size: ~517KB after UPX compression

## Deployment Targets
- MetaMCP container: `/usr/local/bin/metamcp-healthcheck`
- RustDesk container: `/usr/local/bin/healthcheck`
- Cross-compiled for ARM64 (aarch64-unknown-linux-musl)
