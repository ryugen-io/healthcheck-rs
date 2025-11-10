# healthcheckrs

<!-- Build & CI -->
[![CI](https://github.com/ryugen-io/healthcheck-rs/workflows/CI/badge.svg)](https://github.com/ryugen-io/healthcheck-rs/actions)
![Last Commit](https://img.shields.io/badge/commit-d9af197-blue?logo=git)

<!-- Rust -->
![Rust Edition](https://img.shields.io/badge/rust-2024-orange?logo=rust)
![Rust Version](https://img.shields.io/badge/rustc-1.91+-blue?logo=rust)

<!-- Quality -->
![Code Style](https://img.shields.io/badge/code%20style-rustfmt-blue)
![Tests](https://img.shields.io/badge/tests-21%20passing-brightgreen?logo=checkmarx)
![Benchmarks](https://img.shields.io/badge/benchmarks-4%20suites-blue?logo=timer)
![Lines of Code](https://img.shields.io/badge/max%20LOC-150%2Ffile-yellow)

<!-- License -->
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green)

Modular, config-driven health check system for Docker containers.

A small project as I am just learning Rust and swapped to Linux in general.
Bear with me.
Built with my buddy Claude Code.

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
./healthcheckrs /path/to/config.conf

# Example config format:
# tcp:host=127.0.0.1,port=22,timeout_ms=1000
# http:url=http://localhost:8080,timeout_ms=5000
# database:conn_str=postgresql://user:pass@host:5432/db,timeout_ms=3000
# process:name=systemd
```

## Docker Integration

### Method 1: COPY binary into image

```dockerfile
FROM alpine:latest

# Install dependencies (if needed)
RUN apk add --no-cache ca-certificates

# Copy healthcheckrs binary
COPY --chmod=755 healthcheckrs /usr/local/bin/healthcheckrs

# Create healthcheckrs config
RUN echo "tcp:host=127.0.0.1,port=8080,timeout_ms=1000" > /etc/healthcheckrs.conf

# Your application setup here
COPY your-app /app/your-app

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD ["/usr/local/bin/healthcheckrs", "/etc/healthcheckrs.conf"]

CMD ["/app/your-app"]
```

### Method 2: Volume mount in docker-compose

```yaml
volumes:
  - ./healthcheckrs:/usr/local/bin/healthcheckrs:ro
  - ./healthcheckrs.conf:/etc/healthcheckrs.conf:ro
healthcheck:
  test: ["/usr/local/bin/healthcheckrs", "/etc/healthcheckrs.conf"]
  interval: 30s
  timeout: 3s
  retries: 3
  start_period: 5s
```

### Example configs for common scenarios

**Web service:**
```conf
http:url=http://localhost:8080/health,timeout_ms=5000
```

**Database container:**
```conf
tcp:host=127.0.0.1,port=5432,timeout_ms=3000
database:conn_str=postgresql://user:pass@localhost:5432/db,timeout_ms=3000
```

**Multi-check:**
```conf
tcp:host=127.0.0.1,port=8080,timeout_ms=1000
http:url=http://localhost:8080/health,timeout_ms=5000
process:name=my-app
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

Recommended installation:
- Copy `healthcheckrs` binary to `/usr/local/bin/healthcheckrs`
- Place config files in `/etc/` directory
- Set binary permissions to `755`
