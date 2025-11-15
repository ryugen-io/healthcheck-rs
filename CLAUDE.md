# CLAUDE.md - AI Assistant Development Guide

> Comprehensive guide for AI assistants working with the healthcheck-rs codebase

Last Updated: 2025-11-15

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Codebase Structure](#codebase-structure)
3. [Architecture Patterns](#architecture-patterns)
4. [Development Workflows](#development-workflows)
5. [Key Conventions](#key-conventions)
6. [Security Guidelines](#security-guidelines)
7. [Testing Strategy](#testing-strategy)
8. [Common Tasks](#common-tasks)
9. [Git Workflow](#git-workflow)
10. [Important Files Reference](#important-files-reference)

---

## Project Overview

**healthcheck-rs** is a lightweight, security-focused health check system for Docker containers and production deployments. It's written in Rust (Edition 2024) with an emphasis on minimal dependencies, security-first design, and size optimization.

### Core Principles

- **Minimal Dependencies**: Only 3 runtime dependencies (log, env_logger, postgres)
- **Security-First**: Path validation, TOCTOU prevention, system directory protection
- **Size-Optimized**: ~530KB compressed binary with UPX
- **Library-First**: Core functionality separated from CLI binary
- **Deployment-Oriented**: Built-in commands for generating deployment binaries and configs

### Project Stats

- **Languages**: Rust 2024 (Edition), requires rustc 1.91+
- **Total Lines**: ~2,877 lines across 47 .rs files
- **Max LOC per file**: 150 lines (enforced)
- **Test Coverage**: 34 tests (21 unit + 13 integration)
- **Benchmarks**: 4 performance benchmark suites (divan)
- **License**: MIT OR Apache-2.0 (dual licensing)

---

## Codebase Structure

### Workspace Organization

This is a **Cargo workspace** with two main crates:

```
healthcheck-rs/
├── Cargo.toml                   # Workspace root configuration
├── health-core/                 # Core library (reusable logic)
│   ├── Cargo.toml              # healthcheck-core crate
│   ├── src/
│   │   ├── lib.rs              # Public API exports
│   │   ├── config/             # Configuration parsing
│   │   │   ├── mod.rs
│   │   │   ├── file.rs         # Config file parser
│   │   │   ├── http.rs         # HTTP config
│   │   │   ├── database.rs     # Database config
│   │   │   └── helpers.rs      # Shared utilities
│   │   ├── memory/             # Memory statistics
│   │   │   ├── mod.rs
│   │   │   ├── container.rs    # cgroup memory stats
│   │   │   └── host.rs         # /proc/meminfo parsing
│   │   ├── probes/             # Health check implementations
│   │   │   ├── mod.rs          # ProbeResult, HealthCheck trait
│   │   │   ├── tcp/            # TCP connectivity check
│   │   │   ├── http/           # HTTP/HTTPS endpoint check
│   │   │   ├── process/        # Process existence check
│   │   │   └── database.rs     # PostgreSQL check
│   │   └── registry/           # Probe registration system
│   │       └── mod.rs          # CheckRegistry, factory pattern
│   ├── tests/                  # Integration tests (no inline tests!)
│   │   ├── config_parse.rs
│   │   ├── config_env.rs
│   │   ├── tcp_probe.rs
│   │   ├── process_probe.rs
│   │   ├── registry.rs
│   │   ├── memory.rs
│   │   └── meminfo.rs
│   └── benches/                # Performance benchmarks
│       ├── tcp_check.rs
│       ├── http_check.rs
│       ├── process_check.rs
│       └── database_check.rs
│
├── health-bin/                 # CLI binary application
│   ├── Cargo.toml              # healthcheck-bin crate
│   ├── src/
│   │   ├── main.rs             # Entry point and routing
│   │   ├── lib.rs              # Internal modules (empty, no public API)
│   │   ├── cli/                # Argument parsing
│   │   │   └── mod.rs          # Manual CLI parser (no clap)
│   │   ├── commands/           # Deployment commands
│   │   │   ├── mod.rs
│   │   │   ├── generate_bin.rs # Platform binary generation
│   │   │   └── generate_conf.rs # Config file generation
│   │   ├── path_validation/    # Security: path checks
│   │   │   └── mod.rs          # TOCTOU prevention, path traversal
│   │   ├── runner.rs           # Health check orchestration
│   │   └── status/             # JSON output formatting
│   │       └── mod.rs          # Manual JSON serialization
│   └── tests/                  # Integration tests
│       ├── common/             # Shared test utilities
│       │   └── mod.rs
│       ├── healthcheck_execution.rs
│       ├── generate_bin.rs
│       └── generate_conf.rs
│
├── tests/                      # E2E tests and benchmarks
│   ├── Dockerfile.bench
│   ├── docker-compose.bench.yml
│   ├── bench-build.sh
│   ├── bench-run.sh
│   └── bench-stop.sh
│
├── justfile                    # Task runner commands
├── rebuild.sh                  # Complete rebuild pipeline
├── install.sh                  # Install with cargo-auditable
├── compress.sh                 # Cross-compile and UPX compression
├── lines.sh                    # LOC analysis (150 line limit)
├── deny.toml                   # License and supply chain checks
├── README.md                   # User-facing documentation
├── CHANGELOG.md                # Version history
└── SECURITY_PII_AUDIT.md       # Security audit documentation
```

### Key Separation of Concerns

1. **health-core**: Library with all health check logic
   - Public API: `config`, `memory`, `probes`, `registry`
   - No CLI code, completely reusable
   - Can be published to crates.io

2. **health-bin**: CLI wrapper and deployment tools
   - Depends on health-core
   - Provides `healthcheck` binary
   - Commands: run checks, generate-bin, generate-conf

---

## Architecture Patterns

### 1. Plugin/Registry Pattern

All health checks are dynamically registered and created:

```rust
// Define trait
pub trait HealthCheck: Send + Sync {
    fn check(&self) -> ProbeResult;
    fn name(&self) -> &str;
}

// Register factories in runner.rs
let mut registry = CheckRegistry::new();
registry.register("tcp", TcpCheck::from_params);
registry.register("http", HttpCheck::from_params);
registry.register("database", DatabaseCheck::from_params);
registry.register("process", ProcessCheck::from_params);

// Create checks from config
let check = registry.create_check("tcp", params)?;
let result = check.check();
```

**Location**: health-core/src/registry/mod.rs

### 2. Strategy Pattern (Probe Types)

Each probe type is a separate strategy implementing `HealthCheck`:

- **TcpCheck** (health-core/src/probes/tcp/): TCP connectivity with timeout
- **HttpCheck** (health-core/src/probes/http/): HTTP/HTTPS endpoint check
- **DatabaseCheck** (health-core/src/probes/database.rs): PostgreSQL connection
- **ProcessCheck** (health-core/src/probes/process/): Linux process via /proc

### 3. Factory Pattern

Each probe provides a factory function:

```rust
impl TcpCheck {
    pub fn from_params(params: &HashMap<String, String>)
        -> Result<Box<dyn HealthCheck>, String> {
        // 1. Validate required parameters
        // 2. Apply defaults
        // 3. Return boxed instance
    }
}
```

### 4. Workspace Structure

- **Multi-crate workspace**: Defined in root Cargo.toml
- **One-way dependency**: health-bin → health-core
- **Separate versioning**: Each crate has its own version (currently 0.1.0)

---

## Development Workflows

### Quick Commands (using `just`)

```bash
# Complete dev workflow (recommended before commit)
just dev                    # fmt + clippy + check + test

# Individual tasks
just fmt                    # Format code
just clippy                 # Run clippy with -D warnings
just test                   # Run all tests
just check                  # Fast incremental check
just bench                  # Run benchmarks

# Security and quality
just audit                  # cargo audit
just geiger                 # Check unsafe code
just geiger-forbid          # Ensure no unsafe code

# Build and release
just auditable-build        # Build with cargo-auditable
just rebuild                # Run ./rebuild.sh
just compress               # Cross-compile and compress
just compress-native        # Compress native target only

# Deployment commands
just gen-conf               # Generate healthcheck.config
just gen-bin                # Generate deployment binary

# Utilities
just count                  # Count lines with color coding
```

### Complete Rebuild Pipeline

```bash
./rebuild.sh
```

**Pipeline steps**:
1. Check for cargo and cargo-auditable
2. Run `cargo fmt --all`
3. Run `cargo clippy --all-targets --all-features -- -D warnings`
4. Run `cargo auditable build --release`
5. Show binary info and metadata

**Features**:
- Colored output with visual feedback
- Exit code validation
- Automatic cargo-auditable installation
- Metadata verification with `readelf`

### Cross-Compilation and Compression

```bash
# All targets (Linux x86_64, ARM64, Windows)
./compress.sh

# Native target only (faster for development)
./compress.sh --native
```

**Targets**:
- x86_64-unknown-linux-musl (Linux x86_64)
- aarch64-unknown-linux-musl (Linux ARM64)
- x86_64-pc-windows-gnu (Windows x86_64)

**Result**: ~530KB compressed binaries

### Line Count Analysis

```bash
./lines.sh              # Default 150 line limit
./lines.sh 120          # Custom limit
```

**Color coding**:
- Green: <80% of limit
- Yellow: 80-100% of limit
- Red: >100% of limit (violates standard)

---

## Key Conventions

### Code Quality Standards

1. **Rust Edition 2024** (requires rustc 1.91+)
2. **Max 150 lines per file** (enforced by lines.sh)
3. **Zero clippy warnings** (`cargo clippy -- -D warnings`)
4. **No inline tests** (all tests in `tests/` directories)
5. **Manual parsing** (no serde, clap to minimize dependencies)
6. **Idiomatic Rust** (src/module/mod.rs structure)

### Naming Conventions

**Files**:
- Module files: `src/probes/tcp/mod.rs`
- Config files: `src/probes/tcp/config.rs`
- Test files: `tests/tcp_probe.rs`
- Bench files: `benches/tcp_check.rs`

**Functions**:
- Factory functions: `from_params(params: &HashMap<String, String>)`
- Check methods: `fn check(&self) -> ProbeResult`
- Test functions: `test_function_scenario_expected()`

**Types**:
- Probe structs: `TcpCheck`, `HttpCheck`, `ProcessCheck`
- Config structs: `TcpConfig`, `HttpConfig`, `DatabaseConfig`
- Result types: `ProbeResult`, `Result<T, String>`

### Module Visibility

**health-core/src/lib.rs** (public API):
```rust
pub mod config;
pub mod memory;
pub mod probes;
pub mod registry;
```

**health-bin/src/lib.rs** (internal only):
```rust
// Empty - all modules are internal
```

### Configuration Format

```conf
# Comment
type:param1=value1,param2=value2

# Examples
tcp:host=127.0.0.1,port=8080,timeout_ms=1000
http:url=http://localhost:8080/health,timeout_ms=5000
database:conn_str=${DATABASE_URL},timeout_ms=3000
process:name=myapp
```

**Features**:
- One check per line
- Comments start with `#`
- Environment variables: `${VAR_NAME}`
- Timeout in milliseconds: `timeout_ms=1000`

### Dependency Guidelines

**THINK TWICE before adding dependencies!**

This project prioritizes minimal dependencies:

1. Check if functionality can be implemented manually
2. Verify license compatibility (must be in deny.toml)
3. Run `cargo deny check` to validate
4. Run `cargo geiger` to check for unsafe code
5. Justify the dependency in PR description

**Current runtime dependencies** (keep this minimal):
- `log` - Logging facade
- `env_logger` - Logger implementation
- `postgres` - PostgreSQL client (feature-gated)

---

## Security Guidelines

### Critical Security Principles

1. **Path Validation**: ALL file paths must go through validation
2. **TOCTOU Prevention**: Use atomic operations for file creation
3. **System Protection**: Block writes to critical directories
4. **Credential Warnings**: Add warnings for sensitive data

### Path Validation (REQUIRED!)

**Location**: health-bin/src/path_validation/mod.rs

```rust
use crate::path_validation::validate_output_path;

// ALWAYS validate paths before file operations
let safe_path = validate_output_path(&user_input, is_interactive)?;
```

**Protection against**:
- Path traversal attacks (`../../../etc/passwd`)
- Symlink exploitation (TOCTOU)
- System directory writes (`/etc`, `/bin`, `/sys`, `/proc`)
- Windows system directories (`C:\Windows`, `C:\System32`)

### TOCTOU Prevention

**BAD** (vulnerable to TOCTOU):
```rust
if !path.exists() {
    fs::write(path, content)?;  // Race condition!
}
```

**GOOD** (atomic operation):
```rust
use std::fs::OpenOptions;

OpenOptions::new()
    .write(true)
    .create_new(true)  // Fails if file exists (atomic)
    .open(&path)?
    .write_all(content.as_bytes())?;
```

### System Directory Protection

**Blocked directories** (writes will fail):
- Linux: `/etc`, `/bin`, `/sbin`, `/lib`, `/lib64`, `/usr/*`, `/root`, `/var/run`, `/var/lock`, `/boot`, `/dev`, `/sys`, `/proc`
- Windows: `C:\Windows`, `C:\System32`, `C:\Program Files`

### Credential Handling

**NEVER** commit credentials to config files:
- Add warnings in generated templates
- Document environment variable usage
- Use `${VAR_NAME}` substitution
- Include examples in README

---

## Testing Strategy

### No Inline Tests!

**NEVER use `#[cfg(test)] mod tests { }`**

All tests go in separate `tests/` directories.

### Three Test Levels

#### 1. Unit Tests (health-core/tests/*.rs)

**Purpose**: Test individual probe functionality

**Example**: health-core/tests/tcp_probe.rs
```rust
#[test]
fn test_tcp_check_localhost_succeeds() {
    // Test specific probe logic
}
```

**Run**: `cargo test -p health-core`

#### 2. Integration Tests (health-bin/tests/*.rs)

**Purpose**: End-to-end CLI testing

**Example**: health-bin/tests/healthcheck_execution.rs
```rust
mod common;
use common::get_healthcheck_bin;

#[test]
fn test_healthcheck_with_config() {
    let bin = get_healthcheck_bin();
    // Test CLI behavior
}
```

**Run**: `cargo test -p health-bin`

#### 3. Benchmark Tests (health-core/benches/*.rs)

**Purpose**: Performance benchmarking with divan

**Example**: health-core/benches/tcp_check.rs
```rust
use divan;

#[divan::bench]
fn tcp_check_localhost() {
    // Benchmark probe performance
}
```

**Run**: `cargo bench -p health-core`

### Common Test Module Pattern

**health-bin/tests/common/mod.rs**:
```rust
use std::path::PathBuf;

pub fn get_healthcheck_bin() -> PathBuf {
    // Shared test utilities
}
```

**Usage in tests**:
```rust
mod common;
use common::get_healthcheck_bin;
```

### Test Naming Convention

```rust
#[test]
fn test_<function>_<scenario>_<expected>() {
    // Example: test_tcp_check_timeout_fails()
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p health-core
cargo test -p health-bin

# Specific test
cargo test tcp_check_localhost_succeeds

# With output
cargo test -- --nocapture

# Benchmarks
cargo bench -p health-core
```

---

## Common Tasks

### Adding a New Probe Type

1. **Create probe module** in `health-core/src/probes/xxx/`:

```rust
// health-core/src/probes/xxx/mod.rs
use crate::probes::{HealthCheck, ProbeResult};
use std::collections::HashMap;

pub struct XxxCheck {
    config: XxxConfig,
}

impl XxxCheck {
    pub fn from_params(params: &HashMap<String, String>)
        -> Result<Box<dyn HealthCheck>, String> {
        // Parse and validate params
        let config = XxxConfig::from_params(params)?;
        Ok(Box::new(XxxCheck { config }))
    }
}

impl HealthCheck for XxxCheck {
    fn check(&self) -> ProbeResult {
        let start = std::time::Instant::now();
        // Implement check logic
        ProbeResult {
            success: true,
            message: "Check passed".to_string(),
            elapsed_ms: start.elapsed().as_millis() as u64,
        }
    }

    fn name(&self) -> &str {
        "xxx"
    }
}
```

2. **Create config module** in `health-core/src/probes/xxx/config.rs`:

```rust
use std::collections::HashMap;

pub struct XxxConfig {
    pub param1: String,
    pub timeout_ms: u64,
}

impl XxxConfig {
    pub fn from_params(params: &HashMap<String, String>)
        -> Result<Self, String> {
        // Parse parameters with defaults
        Ok(XxxConfig {
            param1: params.get("param1")
                .ok_or("Missing param1")?.clone(),
            timeout_ms: params.get("timeout_ms")
                .and_then(|v| v.parse().ok())
                .unwrap_or(5000),
        })
    }
}
```

3. **Export in probes/mod.rs**:

```rust
pub mod xxx;
```

4. **Register in health-bin/src/runner.rs**:

```rust
registry.register("xxx", XxxCheck::from_params);
```

5. **Add tests** in `health-core/tests/xxx_probe.rs`:

```rust
#[test]
fn test_xxx_check_success() {
    // Test probe
}
```

6. **Add benchmark** in `health-core/benches/xxx_check.rs`:

```rust
use divan;

#[divan::bench]
fn xxx_check_benchmark() {
    // Benchmark probe
}
```

7. **Update Cargo.toml** in health-core:

```toml
[[bench]]
name = "xxx_check"
harness = false
```

### Modifying Configuration Format

1. Update parser in `health-core/src/config/file.rs`
2. Update probe config structs in `health-core/src/probes/*/config.rs`
3. Update template in `health-bin/src/commands/generate_conf.rs`
4. Add tests in `health-core/tests/config_parse.rs`
5. Update README.md examples

### Adding a CLI Command

1. **Parse command** in `health-bin/src/cli/mod.rs`:

```rust
pub enum CliAction {
    GenerateBin { output_dir: String },
    GenerateConf { output_path: String },
    YourNewCommand { param: String },  // Add here
    // ...
}
```

2. **Implement command** in `health-bin/src/commands/your_command.rs`:

```rust
pub fn execute_your_command(param: &str) -> Result<(), String> {
    // Command implementation
}
```

3. **Route in main.rs**:

```rust
match cli_action {
    CliAction::YourNewCommand { param } => {
        commands::your_command::execute_your_command(&param)?;
    }
    // ...
}
```

4. **Add tests** in `health-bin/tests/your_command.rs`

5. **Update help text** in `health-bin/src/cli/mod.rs`

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run -- config.conf

# Run with verbose output
RUST_LOG=trace cargo run -- generate-bin

# Check binary metadata
readelf -p .dep-v0 target/release/healthcheck

# Verify no unsafe code
cargo geiger --forbid-only
```

---

## Git Workflow

### Branch Naming Convention

- Feature branches: `claude/feature-name-<session-id>`
- All branches MUST start with `claude/` and end with session ID
- Example: `claude/create-codebase-documentation-019qaKVECbGYdWrZNZvG7K74`

### Commit Guidelines

1. **Format code first**: `cargo fmt --all`
2. **Run clippy**: `cargo clippy -- -D warnings`
3. **Run tests**: `cargo test`
4. **Commit with clear message**:

```bash
git add .
git commit -m "feat: add new health check probe for Redis

- Implement RedisCheck with PING command
- Add configuration parsing for Redis host/port
- Add unit tests and benchmarks
- Update documentation with examples"
```

5. **Push to feature branch**:

```bash
git push -u origin claude/feature-name-<session-id>
```

### Commit Message Format

```
<type>: <short summary>

<optional detailed description>

<optional footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Build/tooling changes
- `perf`: Performance improvements
- `security`: Security fixes

### Git Push Requirements

**CRITICAL**: Always use `git push -u origin <branch-name>`

- Branch MUST start with `claude/` and end with session ID
- Push will fail with 403 if branch name is incorrect
- Retry up to 4 times with exponential backoff (2s, 4s, 8s, 16s) on network errors

### Pull Request Guidelines

1. **Ensure all checks pass**:
   - `just dev` (fmt + clippy + check + test)
   - `cargo deny check` (license/advisory checks)
   - `./lines.sh` (LOC limit enforcement)

2. **Update documentation**:
   - README.md (user-facing changes)
   - CHANGELOG.md (version history)
   - CLAUDE.md (this file, if needed)

3. **Write clear PR description**:
   - What: Changes made
   - Why: Motivation and context
   - How: Technical approach
   - Testing: How you tested it

4. **Link to issues** (if applicable)

---

## Important Files Reference

### Configuration Files

**Cargo.toml** (workspace root):
- Workspace members definition
- Release profile optimization (opt-level="z", LTO, strip, etc.)
- Single codegen unit for size optimization

**health-core/Cargo.toml**:
- Crate metadata (version, authors, license)
- Dependencies: log, postgres
- Dev dependencies: divan
- Benchmark configurations

**health-bin/Cargo.toml**:
- Binary configuration (`[[bin]]`)
- Dependencies: health-core, log, env_logger
- Crate metadata

**deny.toml**:
- License allowlist (MIT, Apache-2.0, BSD, etc.)
- Ban multiple versions (warn)
- Block unknown registries and git dependencies
- Security advisory tracking

**justfile**:
- Task runner recipes
- Shell configuration (bash with pipefail)
- Dotenv loading
- All development commands

### Build Scripts

**rebuild.sh**:
- Complete rebuild pipeline
- Format → Clippy → Auditable build
- Colored output with status indicators
- Binary metadata verification

**install.sh**:
- Install healthcheck binary
- Cargo-auditable integration
- Install from local source or GitHub
- Usage examples

**compress.sh**:
- Cross-compilation for multiple targets
- UPX compression with --best --lzma
- Before/after size comparison
- Compression ratio statistics

**lines.sh**:
- Line count analysis per file
- Color-coded output (green/yellow/red)
- Configurable limit (default 150)
- Statistics: total, average, min, max

### Documentation Files

**README.md**:
- User-facing documentation
- Installation instructions
- Usage examples
- Docker integration
- Performance benchmarks

**CHANGELOG.md**:
- Version history (Keep a Changelog format)
- Breaking changes documentation
- Migration guides
- Semantic versioning

**SECURITY_PII_AUDIT.md**:
- Security audit findings
- Path validation documentation
- TOCTOU prevention details
- Credential handling guidelines

**CLAUDE.md** (this file):
- AI assistant development guide
- Codebase structure and conventions
- Development workflows
- Security guidelines

### Git Configuration

**.gitignore**:
- Rust build artifacts (target/, *.rs.bk)
- Binaries (healthcheck-bin, healthcheckrs)
- IDE files (.vscode/, .idea/)
- Secrets (*.key, *.pem, credentials.json)
- Environment files (.env, .env.*)
- Database files (*.db, *.sqlite)
- Local configs (config.local.*, *.config.local)

### Test Infrastructure

**tests/Dockerfile.bench**:
- Benchmark container configuration
- Ubuntu 24.04 base with Rust
- SSH server for process checks
- Test dependencies

**tests/docker-compose.bench.yml**:
- Benchmark service orchestration
- Port mappings for health checks
- Volume mounts for binary

**tests/bench-*.sh**:
- `bench-build.sh`: Build benchmark container
- `bench-run.sh`: Run comprehensive benchmarks
- `bench-stop.sh`: Cleanup containers

---

## Best Practices Summary

### DO

- ✅ Run `just dev` before committing
- ✅ Keep files under 150 lines
- ✅ Use `path_validation::validate_output_path()` for all file operations
- ✅ Use `OpenOptions::create_new()` for atomic file creation
- ✅ Write tests in separate `tests/` directories
- ✅ Add benchmarks for performance-critical code
- ✅ Use environment variables for credentials (`${VAR_NAME}`)
- ✅ Update CHANGELOG.md for user-visible changes
- ✅ Verify license compatibility (`cargo deny check`)
- ✅ Check for unsafe code (`cargo geiger --forbid-only`)
- ✅ Use manual parsing to minimize dependencies
- ✅ Document security implications

### DON'T

- ❌ Add dependencies without justification
- ❌ Use inline `#[cfg(test)]` tests
- ❌ Skip path validation for file operations
- ❌ Use `fs::write()` directly (TOCTOU vulnerability)
- ❌ Commit credentials or secrets
- ❌ Write to system directories
- ❌ Exceed 150 lines per file
- ❌ Leave clippy warnings
- ❌ Use `unwrap()` in production code (prefer `?` or `Result`)
- ❌ Add emoji to code or commits (unless requested)
- ❌ Break backwards compatibility without documenting in CHANGELOG

---

## Quick Reference

### File Locations

| Purpose | Location |
|---------|----------|
| Health check trait | health-core/src/probes/mod.rs |
| Registry system | health-core/src/registry/mod.rs |
| Config parsing | health-core/src/config/file.rs |
| Path validation | health-bin/src/path_validation/mod.rs |
| CLI parsing | health-bin/src/cli/mod.rs |
| Main entry point | health-bin/src/main.rs |
| Test utilities | health-bin/tests/common/mod.rs |

### Command Quick Reference

```bash
# Development
just dev                    # Complete workflow
cargo fmt                   # Format code
cargo clippy -- -D warnings # Lint code
cargo test                  # Run tests

# Building
cargo build                 # Debug build
cargo build --release       # Release build
cargo auditable build --release  # With metadata

# Quality Checks
cargo deny check            # License/advisory check
cargo geiger --forbid-only  # Unsafe code check
./lines.sh                  # LOC check

# Deployment
just gen-bin                # Generate deployment binary
just gen-conf               # Generate config template
./compress.sh               # Cross-compile and compress
```

### Key File Size Limits

- **Max LOC per .rs file**: 150 lines
- **Compressed binary**: ~530KB with UPX
- **Release binary (stripped)**: ~2-3MB before compression

---

## Support and Resources

- **Repository**: https://github.com/ryugen-io/healthcheck-rs
- **Issues**: https://github.com/ryugen-io/healthcheck-rs/issues
- **Changelog**: CHANGELOG.md
- **Security**: SECURITY_PII_AUDIT.md
- **License**: MIT OR Apache-2.0 (dual licensing)

---

**Built with Claude Code**

*This document is maintained for AI assistants. For user-facing documentation, see README.md.*
