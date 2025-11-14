set shell := ["bash", "-eu", "-o", "pipefail", "-c"]
set dotenv-load := true

default:
    @just --list

fmt:
    cargo fmt

check:
    cargo check

clippy: fmt
    cargo clippy -- -D warnings

test:
    cargo test

geiger:
    cargo geiger

geiger-json:
    cargo geiger --output-format Json

geiger-md:
    cargo geiger --output-format GitHubMarkdown

geiger-ratio:
    cargo geiger --output-format Ratio

geiger-forbid:
    cargo geiger --forbid-only

dev: fmt clippy check test
    @echo "All dev checks passed."

audit:
    cargo audit

auditable-build:
    cargo auditable build -p health-bin --release

auditable-run:
    cargo auditable run -p health-bin --release

run:
    cargo run -p health-bin

release:
    cargo run -p health-bin --release

bench:
    cargo bench -p healthcheck-core

# Complete rebuild: format, clippy, and auditable build
rebuild:
    ./rebuild.sh

# Count lines in all .rs files with color coding
count:
    ./lines.sh

# Install healthcheck using cargo-auditable
install:
    ./install.sh

# Compress binaries with UPX for all targets
compress:
    ./compress.sh

# Compress binary with UPX for native target only (faster)
compress-native:
    ./compress.sh --native

# Generate example configuration file
gen-conf output="healthcheck.config":
    cargo run --release -- generate-conf --output {{output}}

# Generate deployment binary
gen-bin output="./bin":
    cargo run --release -- generate-bin --output {{output}}

# Clean build artifacts and generated files
clean-all: clean
    rm -rf bin/
    rm -f healthcheck.config example.config
