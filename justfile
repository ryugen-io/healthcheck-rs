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
