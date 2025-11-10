# Benchmark Container

Comprehensive benchmarking environment for healthcheckrs with both end-to-end and unit-level benchmarks.

## Features

- **hyperfine**: End-to-end black-box benchmarks (measures full binary execution)
- **criterion/divan**: Unit-level benchmarks with detailed statistics and HTML reports
- **Realistic environment**: Ubuntu with SSH server for TCP checks
- **Reproducible**: Containerized environment ensures consistent results

## Quick Start

```bash
# Build benchmark container
./tests/bench-build.sh

# Run all benchmarks (hyperfine + divan)
./tests/bench-run.sh

# Stop and cleanup
./tests/bench-stop.sh

# Or run interactively
docker-compose -f tests/docker-compose.bench.yml run --rm healthcheck-bench bash

# Inside container:
/usr/local/bin/run-bench.sh
```

## Manual Usage

```bash
# Build container
docker build -f tests/Dockerfile.bench -t healthcheck-bench ..

# Run benchmarks
docker run --rm healthcheck-bench

# Extract criterion HTML reports
docker run --name bench-tmp healthcheck-bench
docker cp bench-tmp:/workspace/target/criterion ./criterion-reports
docker rm bench-tmp
```

## Benchmark Types

### End-to-End (hyperfine)
Measures complete binary execution including:
- Process startup
- Config parsing
- Check execution
- Output formatting

Results exported as markdown table.

### Unit-Level (criterion/divan)
Detailed profiling of individual components:
- TCP connection establishment
- HTTP request/response
- Database queries
- Process checks
- Config parsing

Results include:
- Statistical analysis (mean, median, std dev)
- Performance comparison over time
- HTML reports with graphs
- Regression detection

## Output

- **Hyperfine**: `/tmp/hyperfine-results.md`
- **Criterion**: `target/criterion/` (HTML reports)

## Notes

- Container includes full Rust toolchain (~1.5GB)
- First build compiles from source (adds ~2-3 minutes)
- Subsequent runs use cached builds
- SSH server runs on port 22 for TCP checks
