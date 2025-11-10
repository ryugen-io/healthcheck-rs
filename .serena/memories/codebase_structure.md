# Codebase Structure

## Workspace Layout
```
healthcheck-src/
├── Cargo.toml           # Workspace definition with release profile
├── deny.toml            # cargo-deny configuration for licenses/advisories
├── justfile             # Task runner with common commands
├── .cargo/config.toml   # Cross-compilation linker settings
├── health-core/         # Library crate
│   ├── src/
│   │   ├── lib.rs
│   │   ├── probes/      # Health check implementations
│   │   │   ├── tcp/     # TCP port connectivity check
│   │   │   ├── http/    # HTTP endpoint check
│   │   │   ├── process/ # Process running check
│   │   │   └── database.rs # PostgreSQL database check
│   │   ├── registry/    # Registry pattern for extensibility
│   │   ├── config/      # Config parsing and data structures
│   │   └── memory/      # Memory stats (host/container)
│   ├── tests/           # Integration tests (NOT inline)
│   └── benches/         # Divan benchmarks
└── health-bin/          # Binary crate
    ├── src/
    │   ├── main.rs      # Entry point, registry setup
    │   ├── lib.rs       # Re-exports for testing
    │   └── status/      # Status JSON output (legacy)
    └── tests/           # Integration tests

## Module Organization

### health-core
- **probes/**: Each check type has its own module
  - Config structs (params from HashMap)
  - HealthCheck trait implementation
  - Check execution logic
- **registry/**: Factory pattern for creating checks from config
- **config/**: Config file parser (custom, no TOML/serde)
- **memory/**: System/container memory stats (not used in current deployment)

### health-bin
- **main.rs**: Registers check types, loads config, executes checks
- **status/**: Old JSON output module (may be removed)

## File Size Constraint
ALL source files must be < 150 LOC. If exceeding, split into submodules.

## Test Organization
- NO inline tests (`#[cfg(test)]` modules)
- Unit tests: `crate/tests/*.rs`
- Integration tests: `health-bin/tests/*.rs`
- Benchmarks: `health-core/benches/*.rs` using divan
