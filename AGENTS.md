# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` is the single binary entry point; keep HTTP, database, and system probes in dedicated helper functions to avoid a sprawling `main`.
- Add shared utilities under `src/` submodules (e.g., `src/checks/http.rs`) and re-export them in `lib.rs` if the binary starts to grow.
- Integration tests belong in `tests/` (create the directory when needed); sample JSON fixtures for health output can live in `tests/fixtures/`.

## Build, Test, and Development Commands
- `cargo check` — fast incremental validation; run before pushing any change.
- `cargo fmt` — applies `rustfmt` using the default nightly/stable style; required before committing.
- `cargo clippy -- -D warnings` — enforces lint cleanliness; fix or explicitly justify any warning.
- `cargo test` — executes unit and integration suites; use environment variables like `METAMCP_URL` or `POSTGRES_HOST` to point to test services.
- `cargo run --release` — emits the optimized healthcheck binary used in production automation.

## Coding Style & Naming Conventions
- Follow Rust 2021 defaults: four-space indentation, snake_case for functions/modules, UpperCamelCase for types.
- Keep I/O code resilient: prefer `Result<T, anyhow::Error>` returning helpers and bubble errors up to `main`.
- Document non-obvious probes with `///` rustdoc comments so that `cargo doc` remains useful.

## Testing Guidelines
- Add unit tests beside the code with `#[cfg(test)]` modules, named `mod tests` and functions like `fn returns_error_on_timeout()`.
- Place integration tests in `tests/*.rs`, mirroring scenario names (`tests/http_ok.rs`).
- Target >90% branch coverage for new logic; fake external systems via dependency injection or feature-gated mocks.

## Commit & Pull Request Guidelines
- Current snapshot lacks Git history; adopt Conventional Commits (`feat:`, `fix:`, `chore:`) with imperative subject lines under 72 characters.
- Reference tracking issues in the footer (`Refs #123`) and describe runtime impacts in the body.
- Pull requests must include: change summary, manual/automated test evidence (`cargo test` output), config notes (env vars touched), and screenshots or JSON samples when the health payload shape changes.

## Configuration & Security Tips
- Never commit real credentials; the binary reads `POSTGRES_*` and `METAMCP_URL` from the environment—document default `.env` templates instead.
- When reproducing incidents, cap timeouts via `METAMCP_TIMEOUT_MS`-style overrides rather than editing constants so the binary stays production-safe.
