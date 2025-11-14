# healthcheck

<!-- Build & CI -->
[![CI](https://img.shields.io/github/actions/workflow/status/ryugen-io/healthcheck-rs/ci.yml?branch=master&label=CI&logo=github)](https://github.com/ryugen-io/healthcheck-rs/actions)
[![Release](https://img.shields.io/github/v/release/ryugen-io/healthcheck-rs?logo=github)](https://github.com/ryugen-io/healthcheck-rs/releases)

<!-- Rust -->
![Rust Edition](https://img.shields.io/badge/rust-2024-orange?logo=rust)
![Rust Version](https://img.shields.io/badge/rustc-1.91+-blue?logo=rust)

<!-- Quality -->
[![Tests](https://img.shields.io/github/actions/workflow/status/ryugen-io/healthcheck-rs/ci.yml?branch=master&label=tests&logo=checkmarx)](https://github.com/ryugen-io/healthcheck-rs/actions/workflows/ci.yml)
![Code Style](https://img.shields.io/badge/code%20style-rustfmt-blue)
![Lines of Code](https://img.shields.io/badge/max%20LOC-150%2Ffile-yellow)

<!-- License -->
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green)

Lightweight, security-focused health check system for Docker containers and production deployments.

A modular Rust-based health check tool with deployment-first CLI commands, comprehensive security features, and minimal dependencies.

## Features

### Health Check Probes
- **TCP Port Check**: Verify TCP connectivity with configurable timeout
- **HTTP/HTTPS Check**: Test HTTP endpoints with full TLS support
- **Database Check**: PostgreSQL connection verification with authentication
- **Process Check**: Linux process existence verification
- **Memory Stats**: Container and host memory usage monitoring

### Deployment Tools
- **`generate-bin`**: Create platform-specific deployment binaries (linux-amd64, linux-arm64, etc.)
- **`generate-conf`**: Generate example configuration files with TOCTOU-safe creation
- **Config-based**: Simple key=value configuration format with environment variable support
- **Docker-optimized**: Designed for container healthchecks with minimal footprint

### Security
- **Path Validation**: Comprehensive path traversal attack prevention
- **TOCTOU Prevention**: Atomic file operations with symlink resolution
- **System Directory Protection**: Blocks writes to /etc, /bin, /sys, /proc, Windows system dirs
- **Credential Warnings**: Built-in warnings for sensitive data in configs
- **Supply Chain Security**: Built with `cargo-auditable` for dependency tracking

### Performance
- **Minimal Dependencies**: Only `log` and `env_logger` for runtime
- **Size-optimized**: ~530KB compressed binary with UPX
- **Fast Execution**: Parallel health checks with microsecond-level latency
- **Cross-platform**: Linux (x86_64, ARM64), macOS, Windows support

## Quick Start

```bash
# Run health checks with config file
healthcheck myconfig.conf

# Generate deployment binary for current platform
healthcheck generate-bin

# Generate example configuration
healthcheck generate-conf
```

## Installation

### From Release

Download the latest binary from [releases](https://github.com/ryugen-io/healthcheck-rs/releases):

```bash
# Linux x86_64
wget https://github.com/ryugen-io/healthcheck-rs/releases/latest/download/healthcheck-linux-amd64
chmod +x healthcheck-linux-amd64
sudo mv healthcheck-linux-amd64 /usr/local/bin/healthcheck

# Linux ARM64
wget https://github.com/ryugen-io/healthcheck-rs/releases/latest/download/healthcheck-linux-arm64
chmod +x healthcheck-linux-arm64
sudo mv healthcheck-linux-arm64 /usr/local/bin/healthcheck
```

### From Source

```bash
# Standard build
cargo build --release

# With cargo-auditable (recommended for production)
cargo auditable build --release

# Cross-compile for ARM64
cargo build --release --target aarch64-unknown-linux-musl
```

## Configuration

Create a config file with one check per line:

```conf
# TCP connectivity check
tcp:host=127.0.0.1,port=8080,timeout_ms=1000

# HTTP endpoint check
http:url=http://localhost:8080/health,timeout_ms=5000

# Database connection check
database:conn_str=postgresql://user:pass@localhost:5432/db,timeout_ms=3000

# Process existence check
process:name=myapp
```

**Environment Variables:**

Use `${VAR_NAME}` syntax in config files:

```conf
database:conn_str=${DATABASE_URL},timeout_ms=3000
http:url=${API_ENDPOINT}/health
```

**Generate Example Config:**

```bash
# Creates healthcheck.config with examples
healthcheck generate-conf

# Create at custom path
healthcheck generate-conf --output custom.conf
```

## Usage

### Basic Health Checks

```bash
# Run with default config (healthcheck.config)
healthcheck

# Run with specific config file
healthcheck /path/to/config.conf

# Show help
healthcheck --help

# Show version
healthcheck --version
```

### Deployment Commands

**Generate Platform Binary:**

```bash
# Generate binary in ./bin directory
healthcheck generate-bin

# Custom output directory
healthcheck generate-bin --output ./deploy

# Output example:
#   Platform: linux-amd64
#   Target:   ./bin/healthcheck-linux-amd64
```

**Generate Configuration:**

```bash
# Generate in current directory
healthcheck generate-conf

# Custom output path
healthcheck generate-conf --output /etc/healthcheck.conf
```

## Docker Integration

### Method 1: COPY Binary in Dockerfile

```dockerfile
FROM alpine:latest

# Install CA certificates for HTTPS checks
RUN apk add --no-cache ca-certificates

# Copy healthcheck binary (generated via `healthcheck generate-bin`)
COPY --chmod=755 bin/healthcheck-linux-amd64 /usr/local/bin/healthcheck

# Create healthcheck config
COPY healthcheck.conf /etc/healthcheck.conf

# Your application setup
COPY your-app /app/your-app

# Configure Docker healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD ["/usr/local/bin/healthcheck", "/etc/healthcheck.conf"]

CMD ["/app/your-app"]
```

### Method 2: Volume Mount in docker-compose

```yaml
services:
  webapp:
    image: your-app:latest
    volumes:
      - ./bin/healthcheck-linux-amd64:/usr/local/bin/healthcheck:ro
      - ./healthcheck.conf:/etc/healthcheck.conf:ro
    healthcheck:
      test: ["/usr/local/bin/healthcheck", "/etc/healthcheck.conf"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 5s
```

### Example Configurations

**Web Service:**
```conf
http:url=http://localhost:8080/health,timeout_ms=5000
tcp:host=127.0.0.1,port=8080,timeout_ms=1000
```

**Database Container:**
```conf
tcp:host=127.0.0.1,port=5432,timeout_ms=3000
database:conn_str=postgresql://healthcheck:pass@localhost:5432/postgres,timeout_ms=3000
```

**Multi-Service App:**
```conf
tcp:host=127.0.0.1,port=8080,timeout_ms=1000
http:url=http://localhost:8080/api/health,timeout_ms=5000
process:name=my-app
database:conn_str=${DATABASE_URL},timeout_ms=3000
```

## Development

### Prerequisites

- Rust 1.91+ (Edition 2024)
- [just](https://github.com/casey/just) command runner (optional)
- [cargo-auditable](https://github.com/rust-secure-code/cargo-auditable) (recommended)

### Quick Development Workflow

```bash
# Complete dev workflow: format + clippy + check + test
just dev

# Individual commands
just fmt          # Format code with rustfmt
just clippy       # Run clippy with -D warnings
just test         # Run all tests
just check        # Fast incremental check
just run          # Run debug build
just release      # Run release build
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized for size)
cargo build --release

# With supply chain security auditing
cargo auditable build --release

# Cross-compile for ARM64
cargo build --release --target aarch64-unknown-linux-musl
```

### Testing

```bash
# Run all tests (21 unit + 13 integration = 34 total)
cargo test

# Run tests for specific crate
cargo test -p health-core
cargo test -p health-bin

# Run specific test
cargo test tcp_check_localhost_succeeds
```

### Quality Checks

```bash
# Security audit
just audit                    # cargo audit
cargo auditable audit         # audit with supply chain info

# Unsafe code analysis
just geiger                   # cargo geiger analysis
just geiger-forbid           # check for forbidden unsafe

# License and advisory check
cargo deny check
```

## Benchmarks

### Running Benchmarks

```bash
# Unit-level benchmarks (divan)
cargo bench -p health-core

# End-to-end benchmarks (hyperfine)
hyperfine --warmup 10 './target/release/healthcheck config.conf'

# Container benchmarks
./tests/bench-build.sh        # Build benchmark container
./tests/bench-run.sh          # Run comprehensive benchmarks
./tests/bench-stop.sh         # Cleanup
```

See [`tests/README.bench.md`](tests/README.bench.md) for detailed benchmark documentation.

### Performance Results

#### ARM64 (Raspberry Pi 5)

Benchmarked on Raspberry Pi 5 (Cortex-A76, 4 cores, 8GB RAM) in Ubuntu 24.04 container:

**End-to-End (hyperfine):**
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `TCP Check (localhost:22)` | 3.6 ± 3.1 | 0.8 | 20.1 | 6.46 ± 5.61 |
| `Process Check (sshd)` | 0.6 ± 0.1 | 0.5 | 1.8 | 1.00 |

**Unit-Level (divan):**
| Benchmark | Mean | Median | Range |
|:---|---:|---:|---:|
| TCP check | 45.39 µs | 31.43 µs | 19.72 µs - 420.5 µs |
| Process check (existing) | 23.82 µs | 23.21 µs | 23.11 µs - 80.12 µs |
| TCP config parse | 205.2 ns | 198.4 ns | 196 ns - 325.7 ns |
| Process config parse | 110.7 ns | 79.2 ns | 78.01 ns - 2.014 µs |

#### x86_64 (AMD Ryzen 7 7800X3D)

Benchmarked on Ryzen 7 7800X3D, 96GB RAM (binary in tmpfs):

| Command | Mean [µs] | Min [µs] | Max [µs] | Relative |
|:---|---:|---:|---:|---:|
| `TCP Check (localhost:22)` | 566.5 ± 95.5 | 351.0 | 961.6 | 1.00 |
| `Process Check (systemd)` | 658.9 ± 96.5 | 417.4 | 975.5 | 1.16 ± 0.26 |

## Code Quality Standards

- **Rust Edition**: 2024
- **Min Rust Version**: 1.91+
- **Max LOC per file**: 150 lines (enforced by linter)
- **No inline tests**: All tests in `tests/` directories
- **Clippy**: Zero warnings with `-D warnings`
- **Test Coverage**: 34 tests (21 unit + 13 integration)
- **License**: MIT OR Apache-2.0 dual licensing

## Security

This project follows security-first design principles:

- **Path Validation**: All file paths are validated and canonicalized to prevent path traversal attacks
- **TOCTOU Prevention**: Atomic file operations using `OpenOptions::create_new()` prevent time-of-check-time-of-use vulnerabilities
- **System Protection**: Blocks writes to critical system directories (/etc, /bin, /sys, /proc, Windows system dirs)
- **Safe Defaults**: Interactive confirmation for file overwrites, non-interactive mode fails atomically
- **Supply Chain**: Built with `cargo-auditable` for dependency auditing
- **Minimal Attack Surface**: Only 2 runtime dependencies (log, env_logger)

See [SECURITY_PII_AUDIT.md](SECURITY_PII_AUDIT.md) for comprehensive security audit.

## Project Structure

```
healthcheck-rs/
├── health-core/          # Core library with probe implementations
│   ├── src/
│   │   ├── probes/      # TCP, HTTP, database, process probes
│   │   ├── config/      # Configuration parsing and validation
│   │   ├── memory/      # Memory stats (container + host)
│   │   └── registry/    # Probe registry and execution
│   ├── tests/           # Unit tests
│   └── benches/         # Performance benchmarks
├── health-bin/          # CLI application
│   ├── src/
│   │   ├── commands/    # generate-bin, generate-conf
│   │   ├── cli/         # Argument parsing and help
│   │   ├── path_validation/  # Security: path validation
│   │   ├── runner.rs    # Health check execution
│   │   └── status.rs    # Exit code management
│   └── tests/           # Integration tests
└── tests/               # E2E tests and benchmarks
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and migration guides.

## Planned Features

- **`serve` command**: HTTP API server mode for remote health monitoring
- **`watch` command**: Continuous monitoring with configurable intervals
- **Custom Probes**: Plugin system for user-defined health checks
- **API Access**: External API health checks (REST endpoint monitoring)

## License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contributing

Contributions are welcome! This project maintains high code quality standards:

1. All code must pass `cargo clippy -- -D warnings`
2. Format code with `cargo fmt`
3. Add tests for new features
4. Keep files under 150 lines
5. Run `just dev` before submitting PRs

Built with Claude Code.
