# Suggested Commands

## Development Workflow (using just)

### Quick Checks
```bash
just fmt              # Format code with rustfmt
just check            # Fast incremental check
just clippy           # Run clippy with -D warnings (also runs fmt)
just test             # Run all tests
just dev              # Run all dev checks: fmt + clippy + check + test
```

### Building
```bash
cargo build                                    # Debug build
cargo build --release                          # Optimized build
just auditable-build                           # Build with cargo-auditable
cargo build --release --target aarch64-unknown-linux-musl  # Cross-compile for ARM64
```

### Running
```bash
just run                                       # Run debug binary
just release                                   # Run release binary
cargo run -p health-bin -- <config-file>       # Run with config
```

### Testing & Benchmarking
```bash
just test                                      # All tests
cargo test -p health-core                      # Test core only
cargo test -p health-bin                       # Test bin only
cargo bench -p health-core                     # Run benchmarks (divan)
```

### Security & Quality
```bash
just audit                                     # cargo audit - check for vulnerabilities
just geiger                                    # cargo geiger - check for unsafe code
just geiger-forbid                             # Only show forbidden unsafe
cargo deny check                               # Check licenses, advisories, bans
```

### LOC Check
```bash
find . -name "*.rs" -not -path "./target/*" -exec wc -l {} + | sort -n
```

### Cross-Compilation Setup
```bash
# On ryujin (x86_64) targeting ryucore (aarch64):
cargo build --release --target aarch64-unknown-linux-musl -p health-bin
```

### Binary Compression (after cross-compilation)
```bash
upx --best --lzma target/aarch64-unknown-linux-musl/release/healthcheck
```

### Deployment
```bash
# Copy binary to deployment locations:
# MetaMCP: /srv/mcp/build-cache/_META_/binaries/healthcheck/
# RustDesk: /srv/rdp/rustdesk/
```

## Manual Commands (without just)

```bash
cargo fmt                                      # Format
cargo check                                    # Check
cargo clippy -- -D warnings                    # Lint
cargo test                                     # Test
cargo audit                                    # Audit
cargo geiger                                   # Check unsafe
cargo deny check                               # License/advisory check
cargo auditable build -p health-bin --release  # Auditable build
```

## Git Commands
```bash
git status                                     # Check status
git add .                                      # Stage all
git commit -m "message"                        # Commit
git log --oneline -n 10                        # View recent commits
```
