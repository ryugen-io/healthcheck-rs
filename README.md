# Healthcheck

![Rust Edition](https://img.shields.io/badge/rust-2024-orange?logo=rust)
![Rust Version](https://img.shields.io/badge/rustc-1.91+-blue?logo=rust)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green)
![Code Style](https://img.shields.io/badge/code%20style-rustfmt-blue)
![Lines of Code](https://img.shields.io/badge/max%20LOC-150%2Ffile-yellow)

Modular, config-driven healthcheck system for Docker containers.

## Features

- **TCP Port Check**: Verify TCP connectivity
- **HTTP Endpoint Check**: Test HTTP/HTTPS endpoints
- **Database Check**: PostgreSQL connection with authentication
- **Process Check**: Verify Linux process is running
- **Config-based**: Simple key=value configuration format
- **Minimal Dependencies**: Custom parsers, no serde/TOML
- **Size-optimized**: ~517KB compressed binary with UPX

## Building

```bash
# Debug build
cargo build

# Release build (optimized for size)
cargo build --release

# Cross-compile for ARM64
cargo build --release --target aarch64-unknown-linux-musl

# With cargo-auditable
cargo auditable build --release
```

## Testing

```bash
# Run all tests
cargo test --all

# Run tests for specific crate
cargo test -p healthcheck-core
cargo test -p healthcheck-bin

# Run specific test
cargo test tcp_check_localhost_succeeds
```

## Benchmarks

Benchmarks use [divan](https://github.com/nvzqz/divan) for accurate performance measurements:

```bash
# Run all benchmarks
cargo bench -p healthcheck-core

# Run specific benchmark
cargo bench -p healthcheck-core --bench tcp_check

# Available benchmarks:
# - tcp_check.rs      - TCP check performance
# - http_check.rs     - HTTP config parsing
# - process_check.rs  - Process check performance
# - database_check.rs - Database config parsing
```

## Usage

```bash
# Run with config file
./healthcheck /path/to/config.conf

# Example config format:
# tcp:host=127.0.0.1,port=22,timeout_ms=1000
# http:url=http://localhost:8080,timeout_ms=5000
# database:conn_str=postgresql://user:pass@host:5432/db,timeout_ms=3000
# process:name=systemd
```

## Development

```bash
# Quick dev workflow
just dev          # fmt + clippy + check + test

# Individual steps
just fmt          # Format code
just clippy       # Run clippy with -D warnings
just test         # Run tests
just check        # Fast incremental check

# Quality checks
just audit        # cargo audit
just geiger       # Check unsafe code
cargo deny check  # License/advisory check
```

## Code Quality

- **Rust Edition**: 2024
- **Min Rust Version**: 1.91
- **Max LOC per file**: 150 lines
- **No inline tests**: All tests in `/tests` directories
- **Clippy**: Zero warnings (`-D warnings`)
- **License**: MIT OR Apache-2.0

## Deployment

Binary deployed to:
- MetaMCP: `/usr/local/bin/metamcp-healthcheck`
- RustDesk: `/usr/local/bin/healthcheck`

See `AGENTS.md` for repository guidelines and coding standards.
