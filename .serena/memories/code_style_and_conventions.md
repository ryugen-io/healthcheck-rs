# Code Style and Conventions

## Rust Edition and Idioms
- **Edition**: 2024
- **Minimum Rust Version**: 1.91
- Follow Rust 2024 idioms: snake_case for functions/modules, UpperCamelCase for types
- Use let-chains where appropriate: `if let Ok(x) = ... && condition {}`

## Formatting
- **Tool**: `cargo fmt` (required before all commits)
- 4-space indentation
- Max line length: Follow rustfmt defaults

## Linting
- **Tool**: `cargo clippy -- -D warnings`
- Zero warnings policy: fix or explicitly justify
- Common fixes:
  - Use `.ok_or("error")` instead of `.ok_or_else(|| "error")` for static strings
  - Avoid unnecessary collapsible if statements
  - Remove unnecessary lazy evaluations

## File Organization
- **Max LOC per file**: 150 lines
- If file exceeds 150 LOC, split into submodules with clear subdirectories
- NO inline tests: all tests in `/tests` directory

## Testing
- NO `#[cfg(test)]` inline test modules
- Unit tests: `crate/tests/*.rs`
- Integration tests: `health-bin/tests/*.rs`
- Test naming: descriptive snake_case (`tcp_check_localhost_succeeds`)

## Error Handling
- Use `Result<T, String>` for most internal errors
- Use `?` operator to propagate errors
- Provide descriptive error messages

## Dependencies
- Minimize external dependencies
- Write custom parsers (config/file.rs) instead of pulling in serde/toml
- Only use well-maintained crates with minimal transitive deps

## Documentation
- Use `///` doc comments for public API
- Keep comments concise and relevant
- Document non-obvious behavior

## Naming Conventions
- Modules: `snake_case`
- Structs/Enums: `UpperCamelCase`
- Functions/methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Check types: lowercase strings in config ("tcp", "http", "database", "process")
