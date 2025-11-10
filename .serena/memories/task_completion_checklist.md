# Task Completion Checklist

When completing a task on this project, follow these steps:

## 1. Code Quality Checks

### Formatting
```bash
cargo fmt
```
- MUST be run before committing
- Auto-fixes formatting issues

### Linting
```bash
cargo clippy -- -D warnings
```
- MUST pass with zero warnings
- Fix all warnings before proceeding

### Testing
```bash
cargo test
```
- All tests must pass
- Add tests for new functionality

### LOC Check
```bash
find . -name "*.rs" -not -path "./target/*" -exec wc -l {} + | sort -n
```
- Verify all files < 150 LOC
- Split files if needed

## 2. Security & Dependencies

### Audit
```bash
cargo audit
```
- Check for known vulnerabilities
- Update dependencies if needed

### Unsafe Code Check
```bash
cargo geiger
```
- Review any unsafe code usage
- Warnings about known issues are acceptable

### License & Advisory Check
```bash
cargo deny check
```
- Verify all dependencies have allowed licenses
- Check for security advisories
- Ensure no duplicate dependency warnings

## 3. Build Verification

### Debug Build
```bash
cargo check
```

### Release Build
```bash
cargo auditable build -p health-bin --release
```

### Cross-Compilation (if deploying)
```bash
cargo build --release --target aarch64-unknown-linux-musl -p health-bin
```

### Binary Size Check
```bash
ls -lh target/aarch64-unknown-linux-musl/release/healthcheck
```

### UPX Compression (if deploying)
```bash
upx --best --lzma target/aarch64-unknown-linux-musl/release/healthcheck
```

## 4. Git Workflow (if requested by user)

### Stage Changes
```bash
git add .
```

### Commit
```bash
git commit -m "descriptive message

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### Verify Commit
```bash
git log --oneline -n 1
git status
```

## 5. Deployment (if applicable)

### Copy Binary
- MetaMCP: Copy to `/srv/mcp/build-cache/_META_/binaries/healthcheck/`
- RustDesk: Copy to `/srv/rdp/rustdesk/`

### Update Docker Images
- Rebuild affected Docker images
- Test healthcheck in container

### Verify Deployment
```bash
docker ps  # Check container status
docker logs <container>  # Check logs
```

## Quick All-in-One Check
```bash
just dev && cargo deny check && cargo audit
```

## NEVER Do
- Never commit without running `cargo fmt`
- Never commit with clippy warnings
- Never commit with failing tests
- Never commit files > 150 LOC
- Never commit inline tests (`#[cfg(test)]` modules)
