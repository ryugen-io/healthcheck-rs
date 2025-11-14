# healthcheck

<!-- Build & CI -->
[![CI](https://img.shields.io/github/actions/workflow/status/ryugen-io/healthcheck-rs/ci.yml?branch=master&label=CI&logo=github)](https://github.com/ryugen-io/healthcheck-rs/actions)
[![Release](https://img.shields.io/github/v/release/ryugen-io/healthcheck-rs?logo=github)](https://github.com/ryugen-io/healthcheck-rs/releases)
![Last Commit](https://img.shields.io/badge/commit-6f040bd-blue?logo=git)

<!-- Rust -->
![Rust Edition](https://img.shields.io/badge/rust-2024-orange?logo=rust)
![Rust Version](https://img.shields.io/badge/rustc-1.91+-blue?logo=rust)

<!-- Quality -->
[![Tests](https://img.shields.io/github/actions/workflow/status/ryugen-io/healthcheck-rs/ci.yml?branch=master&label=tests&logo=checkmarx)](https://github.com/ryugen-io/healthcheck-rs/actions/workflows/ci.yml)
![Code Style](https://img.shields.io/badge/code%20style-rustfmt-blue)
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

End-to-end benchmarks using [hyperfine](https://github.com/sharkdp/hyperfine):

```bash
# Run benchmarks with hyperfine
hyperfine --warmup 10 './target/release/healthcheck config.conf'
```

### Performance Results

#### ARM64 (Container)

Benchmarked on Raspberry Pi 5 (Cortex-A76, 4 cores, 8GB RAM, Arch Linux ARM) in Ubuntu 24.04 Docker container:

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
| Process check (nonexistent) | 24.31 µs | 23.31 µs | 23.22 µs - 80.81 µs |
| TCP config parse | 205.2 ns | 198.4 ns | 196 ns - 325.7 ns |
| Process config parse | 110.7 ns | 79.2 ns | 78.01 ns - 2.014 µs |
| HTTP config parse | 138.4 ns | 138.2 ns | 137 ns - 152 ns |
| HTTP config parse (HTTPS) | 140.6 ns | 138.2 ns | 137 ns - 235.4 ns |
| Database config parse | 201.5 ns | 184.3 ns | 165.3 ns - 2.166 µs |
| Database config parse (complex) | 174.8 ns | 163.6 ns | 161.3 ns - 794.4 ns |

#### x86_64 (Native)

Benchmarked on AMD Ryzen 7 7800X3D, 96GB RAM (binary in tmpfs):

| Command | Mean [µs] | Min [µs] | Max [µs] | Relative |
|:---|---:|---:|---:|---:|
| `TCP Check (localhost:22)` | 566.5 ± 95.5 | 351.0 | 961.6 | 1.00 |
| `Process Check (systemd)` | 658.9 ± 96.5 | 417.4 | 975.5 | 1.16 ± 0.26 |

### Benchmark Container

Run comprehensive benchmarks in a containerized environment:

```bash
# Build benchmark container
./tests/bench-build.sh

# Run all benchmarks (hyperfine + divan)
./tests/bench-run.sh

# Stop/cleanup benchmark containers
./tests/bench-stop.sh
```

The benchmark container includes:
- Full Rust toolchain for building from source
- hyperfine for end-to-end benchmarks
- divan for detailed unit-level profiling
- SSH server for realistic TCP check testing

See [`tests/README.bench.md`](tests/README.bench.md) for detailed usage.

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

## Docker Integration

### Method 1: COPY binary into image

```dockerfile
FROM alpine:latest

# Install dependencies (if needed)
RUN apk add --no-cache ca-certificates

# Copy healthcheck binary
COPY --chmod=755 healthcheck /usr/local/bin/healthcheck

# Create healthcheck config
RUN echo "tcp:host=127.0.0.1,port=8080,timeout_ms=1000" > /etc/healthcheck.conf

# Your application setup here
COPY your-app /app/your-app

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD ["/usr/local/bin/healthcheck", "/etc/healthcheck.conf"]

CMD ["/app/your-app"]
```

### Method 2: Volume mount in docker-compose

```yaml
volumes:
  - ./healthcheck:/usr/local/bin/healthcheck:ro
  - ./healthcheck.conf:/etc/healthcheck.conf:ro
healthcheck:
  test: ["/usr/local/bin/healthcheck", "/etc/healthcheck.conf"]
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

## Planned Features

- **API Access**: Fetch external data from APIs for health checks (e.g., check service status via REST endpoint)

## Deployment

Recommended installation:
- Copy `healthcheck` binary to `/usr/local/bin/healthcheck`
- Place config files in `/etc/` directory
- Set binary permissions to `755`
